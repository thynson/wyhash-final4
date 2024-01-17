use crate::util::{
    likely, unlikely, wy_read_4, wy_read_8, wy_read_tail3, wy_read_tail8, wy_rotate
};
use core::marker::PhantomData;

pub trait WyHashVariant: Sized {
    fn mul_mum(a: u64, b: u64) -> (u64, u64);

    #[inline(always)]
    fn mul_mix(a: u64, b: u64) -> u64 {
        let (a, b) = Self::mul_mum(a, b);
        a ^ b
    }

    ///
    /// Create a new [`WyHasher`] instance with default seed and secret
    ///
    fn with_default() -> WyHasher<Self>
    where
        Self: Sized,
    {
        WyHasher::<Self>::from_seed(0)
    }

    ///
    /// Create a new [`WyHasher`] instance with given seed and default secret
    ///
    fn with_seed(seed: u64) -> WyHasher<Self>
    where
        Self: Sized,
    {
        WyHasher::<Self>::from_seed(seed)
    }

    ///
    /// Create a new [`WyHasher`] instance with given seed and secret
    ///
    fn with_seed_and_secret(seed: u64, secret: [u64; 4]) -> WyHasher<Self>
    where
        Self: Sized,
    {
        WyHasher::<Self>::new(seed, secret)
    }

    ///
    /// Generate a new secret with given seed
    ///
    fn generate_secret(mut seed: u64) -> [u64; 4] {
        let mut secret = [0u64; 4];

        for i in 0..4 {
            'restart: loop {
                secret[i] = 0;
                for j in 0..8 {
                    let idx = (wyrand::<Self>(&mut seed) as usize) % C.len();
                    secret[i] |= (C[idx]) << (j << 3);
                }

                if secret[i] & 1 == 0 {
                    continue;
                }
                for j in 0..i {
                    let x = secret[j] ^ secret[i];
                    if x.count_ones() != 32 {
                        continue 'restart;
                    }
                }
                break;
            }
        }
        secret
    }

    ///
    /// One-shot wyhash function
    ///
    /// This function hash the input with default seed and secret, and it's the preferred way
    /// to hash the input with default seed and secret. And its speed shall be equivalent to
    /// `WyHasher::<V>::hash`, if the compiler properly apply the optimization of constant propagation.
    ///
    fn hash(input: &[u8]) -> u64 {
        Self::with_default().hash(input)
    }

    ///
    /// One-shot wyhash function with custom seed and default secret
    ///
    /// This function hash the input with custom seed and default secret. Through it's fairly fast,
    /// if the seed can be reused, it's recommended to instantiate a [`WyHasher`] instance with
    /// [`Self::with_seed`], and hash the input with [`WyHasher::<V>::hash`], so that the initialization
    /// work can be reused.
    ///
    fn hash_with_seed(input: &[u8], seed: u64) -> u64 {
        Self::with_seed(seed).hash(input)
    }

    ///
    /// This function hash the input with custom seed and default secret. Through it's fairly fast,
    /// if the seed and secret can be reused, it's recommended to instantiate a [`WyHasher`] instance with
    /// [`Self::with_seed_and_secret`], and hash the input with [`WyHasher::<V>::hash`], so that the initialization
    /// work can be reused.
    ///
    fn hash_with_seed_and_secret(input: &[u8], seed: u64, secret: [u64; 4]) -> u64 {
        Self::with_seed_and_secret(seed, secret).hash(input)
    }
}

pub struct WyHasher<T: WyHashVariant> {
    secret: [u64; 4],
    seed: u64,
    _marker: PhantomData<T>,
}

impl<T: WyHashVariant> WyHasher<T> {
    ///
    /// Create a new [`WyHasher`] instance with given seed and secret
    ///
    pub fn new(mut seed: u64, secret: [u64; 4]) -> Self {
        seed ^= T::mul_mix(seed ^ secret[0], secret[1]);
        Self {
            secret,
            seed,
            _marker: PhantomData,
        }
    }

    ///
    /// Create a new [`StreamedWyHasher`] instance with the same seed and secret used
    /// by this hasher instance
    ///
    pub fn streamed(&self) -> StreamedWyHasher<T> {
        StreamedWyHasher::<T>::new_internal(self.seed, self.secret)
    }

    pub fn from_seed(seed: u64) -> Self {
        Self::new(seed, DEFAULT_SECRET)
    }

    fn hash_short_input(&self, input: &[u8]) -> u64 {
        let len = input.len();
        let mut a = 0u64;
        let mut b = 0u64;
        let seed = self.seed;
        if likely(len < 8) {
            if len >= 4 {
                let u = wy_read_4(input);
                let v = wy_read_4(&input[(len - 4)..]);
                a = (u << 32) | u;
                b = (v << 32) | v;
            } else if likely(len > 0) {
                a = wy_read_tail3(input);
            }
        } else {
            a = wy_rotate(wy_read_8(input));
            b = wy_read_8(&input[len - 8..]);
        }
        self.epilogue(a, b, len, seed)
    }

    pub fn hash(&self, mut input: &[u8]) -> u64 {
        if likely(input.len() <= 16) {
            return self.hash_short_input(input);
        }

        let len = input.len();
        let mut a = 0u64;
        let mut b = 0u64;
        let mut seed = self.seed;

        if unlikely(input.len() > 48) {
            let mut s1 = seed;
            let mut s2 = seed;
            loop {
                seed = T::mul_mix(
                    wy_read_8(input) ^ self.secret[1],
                    wy_read_8(&input[8..]) ^ seed,
                );
                s1 = T::mul_mix(
                    wy_read_8(&input[16..]) ^ self.secret[2],
                    wy_read_8(&input[24..]) ^ s1,
                );
                a = wy_read_8(&input[32..]);
                b = wy_read_8(&input[40..]);
                s2 = T::mul_mix(a ^ self.secret[3], b ^ s2);
                input = &input[48..];
                if likely(input.len() <= 48) {
                    break;
                }
            }
            seed ^= s1 ^ s2;
        }

        if input.len() > 32 {
            a = wy_read_8(input);
            b = wy_read_8(&input[8..]);
            seed = T::mul_mix(a ^ self.secret[1], b ^ seed);
            input = &input[16..];
        }

        if input.len() > 16 {
            a = wy_read_8(input);
            b = wy_read_8(&input[8..]);
            seed = T::mul_mix(a ^ self.secret[1], b ^ seed);
            input = &input[16..];
        }

        if input.len() == 16 {
            a = wy_read_8(input);
            b = wy_read_8(&input[8..]);
        } else if input.len() > 8 {
            let shift = ((input.len() - 8) << 3) as u32;
            a = b.wrapping_shr(shift);
            b = wy_read_8(input).wrapping_shl(64 - shift);
            a |= b;
            b = wy_read_8(&input[input.len() - 8..]);
        } else {
            let shift = (input.len() << 3) as u32;
            a = a.wrapping_shr(shift) | b.wrapping_shl(64 - shift);
            b = b.wrapping_shr(shift) | wy_read_tail8(input).wrapping_shl(64 - shift);
        }
        self.epilogue(a, b, len, seed)
    }

    fn epilogue(&self, mut a: u64, mut b: u64, len: usize, seed: u64) -> u64 {
        a ^= self.secret[1];
        b ^= seed;
        (a, b) = T::mul_mum(a, b);
        T::mul_mix(a ^ self.secret[0] ^ (len as u64), b ^ self.secret[1])
    }
}

impl<T: WyHashVariant> Default for WyHasher<T> {
    fn default() -> Self {
        Self::new(0, DEFAULT_SECRET)
    }
}

#[cfg(feature = "std")]
impl<T: WyHashVariant> std::hash::Hasher for WyHasher<T> {
    fn finish(&self) -> u64 {
        self.seed
    }

    fn write(&mut self, bytes: &[u8]) {
        self.seed = self.hash(bytes);
    }
}

#[cfg(feature = "std")]
impl<T: WyHashVariant> std::hash::BuildHasher for WyHasher<T> {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self {
            secret: self.secret,
            seed: self.seed,
            _marker: PhantomData,
        }
    }
}


// Used when generating secret
const C: [u64; 70] = [
    15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86, 89, 90,
    92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149, 150, 153,
    154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201, 202, 204, 209,
    210, 212, 216, 225, 226, 228, 232, 240,
];

const DEFAULT_SECRET: [u64; 4] = [
    0xa0761d6478bd642fu64,
    0xe7037ed1a0b428dbu64,
    0x8ebc6af09c88c6e3u64,
    0x589965cc75374cc3u64,
];

fn wyrand<V: WyHashVariant + ?Sized>(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0xa0761d6478bd642fu64);
    V::mul_mix(*seed, (*seed) ^ 0xe7037ed1a0b428dbu64)
}

pub struct StreamedWyHasher<T: WyHashVariant> {
    seed: u64,
    secret: [u64; 4],
    buffer: [u8; 48],
    len: usize,
    off: usize,
    _marker: PhantomData<T>,
}

impl<T: WyHashVariant> StreamedWyHasher<T> {
    pub fn new(mut seed: u64, secret: [u64; 4]) -> Self {
        seed ^= T::mul_mix(seed ^ secret[0], secret[1]);
        Self::new_internal(seed, secret)
    }

    fn new_internal(seed: u64, secret: [u64; 4]) -> Self {
        Self {
            seed,
            secret,
            buffer: [0; 48],
            len: 0,
            off: 0,
            _marker: PhantomData,
        }
    }

    fn process_buffer(&mut self) {
        let mut seed = self.seed;
        let mut s1 = self.seed;
        let mut s2 = self.seed;

        seed = T::mul_mix(
            wy_read_8(&self.buffer[0..]) ^ self.secret[1],
            wy_read_8(&self.buffer[8..]) ^ seed,
        );
        s1 = T::mul_mix(
            wy_read_8(&self.buffer[16..]) ^ self.secret[2],
            wy_read_8(&self.buffer[24..]) ^ s1,
        );
        s2 = T::mul_mix(
            wy_read_8(&self.buffer[32..]) ^ self.secret[3],
            wy_read_8(&self.buffer[40..]) ^ s2,
        );
        self.seed = seed ^ s1 ^ s2;
    }

    pub fn finish(&self) -> u64 {
        let mut a = 0u64;
        let mut b = 0u64;
        let len = self.len;
        let mut seed = self.seed;
        let input = &self.buffer[0..self.off];
        if len >= 48 {
            if self.off > 32 {
                seed = T::mul_mix(
                    wy_read_8(&self.buffer[0..]) ^ self.secret[1],
                    wy_read_8(&self.buffer[8..]) ^ seed,
                );
                seed = T::mul_mix(
                    wy_read_8(&self.buffer[16..]) ^ self.secret[1],
                    wy_read_8(&self.buffer[24..]) ^ seed,
                );

                a = wy_read_8(&self.buffer[(self.off - 16)..]);
                b = wy_read_8(&self.buffer[(self.off - 8)..]);
            } else if self.off > 16 {
                seed = T::mul_mix(
                    wy_read_8(&self.buffer[0..]) ^ self.secret[1],
                    wy_read_8(&self.buffer[8..]) ^ seed,
                );
                a = wy_read_8(&self.buffer[(self.off - 16)..]);
                b = wy_read_8(&self.buffer[(self.off - 8)..]);
            } else {
                let mut tmp = [0u8; 16];
                let size_read_back = 16 - self.off;
                tmp[0..size_read_back].copy_from_slice(&self.buffer[(48 - size_read_back)..]);
                tmp[size_read_back..].copy_from_slice(&self.buffer[0..self.off]);

                a = wy_read_8(&tmp);
                b = wy_read_8(&tmp[8..]);
            }
        } else if len > 32 {
            seed = T::mul_mix(
                wy_read_8(&input[0..]) ^ self.secret[1],
                wy_read_8(&input[8..]) ^ seed,
            );
            seed = T::mul_mix(
                wy_read_8(&input[16..]) ^ self.secret[1],
                wy_read_8(&input[24..]) ^ seed,
            );
            a = wy_read_8(&input[(len - 16)..]);
            b = wy_read_8(&input[(len - 8)..]);
        } else if len > 16 {
            seed = T::mul_mix(
                wy_read_8(&input[0..]) ^ self.secret[1],
                wy_read_8(&input[8..]) ^ seed,
            );
            a = wy_read_8(&input[(len - 16)..]);
            b = wy_read_8(&input[(len - 8)..]);
        } else if len >= 8 {
            a = wy_rotate(wy_read_8(input));
            b = wy_read_8(&input[len - 8..]);
        } else if len >= 4 {
            let u = wy_read_4(input);
            let v = wy_read_4(&input[(len - 4)..]);
            a = (u << 32) | u;
            b = (v << 32) | v;
        } else if len > 0 {
            a = wy_read_tail3(input);
        }
        a ^= self.secret[1];
        b ^= seed;
        (a, b) = T::mul_mum(a, b);
        T::mul_mix(a ^ self.secret[0] ^ (self.len as u64), b ^ self.secret[1])
    }

    pub fn write(&mut self, mut bytes: &[u8]) {
        let remained = 48 - self.off;
        if self.off + bytes.len() > 48 {
            if self.off != 0 {
                self.buffer[self.off..48].copy_from_slice(&bytes[0..remained]);
                bytes = &bytes[remained..];
                self.len += remained;
                self.off = 0;
                self.process_buffer();
            }

            if bytes.len() > 48 {
                let mut seed = self.seed;
                let mut s1 = self.seed;
                let mut s2 = self.seed;

                while bytes.len() >= 48 {
                    seed = T::mul_mix(
                        wy_read_8(bytes) ^ self.secret[1],
                        wy_read_8(&bytes[8..]) ^ seed,
                    );
                    s1 = T::mul_mix(
                        wy_read_8(&bytes[16..]) ^ self.secret[2],
                        wy_read_8(&bytes[24..]) ^ s1,
                    );
                    let w1 = wy_read_8(&bytes[32..]);
                    let w2 = wy_read_8(&bytes[40..]);

                    s2 = T::mul_mix(w1 ^ self.secret[3], w2 ^ s2);
                    if bytes.len() < 64 {
                        // If we don't have extra 16bytes, we need to save them for reading back
                        self.buffer[32..48].copy_from_slice(&bytes[32..48]);
                    }
                    bytes = &bytes[48..];
                    self.len += 48;
                }
                self.seed = seed ^ s1 ^ s2;
            }
        }

        self.buffer[self.off..(self.off + bytes.len())].copy_from_slice(bytes);
        self.off += bytes.len();
        self.len += bytes.len();
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
pub(crate) mod test {
    pub(crate) type TestVector = [(&'static str, u64, u64); 10];
}
