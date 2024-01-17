use crate::generics::WyHashVariant;
use crate::util::wy_rotate;

///
/// A type represents the wyhash variant that use 32bit multiplication and an extra xor
/// for mixing, which survives against entropy loss that could happens in the probability of 2^-64.
/// Note that the 32 bit variants of wyhash is faster than 64bit variants on 32bit platforms, but
/// much slower on 64bit platforms.
///
#[derive(Copy, Clone)]
pub struct WyHash32Condom {
    _private: (),
}

impl WyHashVariant for WyHash32Condom {
    #[inline(always)]
    fn mul_mum(a: u64, b: u64) -> (u64, u64) {
        let hh = (a >> 32) * (b >> 32);
        let ll = (a as u32 as u64) * (b as u32 as u64);
        let hl = (a >> 32) * ((b as u32) as u64);
        let lh = ((a as u32) as u64) * (b >> 32);
        (a ^ wy_rotate(hl) ^ hh, b ^ wy_rotate(lh) ^ ll)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod test {

    use crate::generics::test::TestVector;

    use super::*;

    const TEST_VECTOR: TestVector = [
        ("",0, 0xeea54221671289db),
        ("a",1, 0xac4d02accfbeae5f),
        ("abc",2, 0xe6a807320c2ecb45),
        ("wyhash",3, 0x193194dc4d7ae61d),
        ("message digest",4, 0x62fb33aec500aa9),
        ("abcdefghijklmnopqrstuvwxyz",5, 0xd5b2f2a561107e8f),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",6, 0xdc0cb661c3a5717e),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",7, 0xda836825fca55370),
        ("12345678901234567890123456789012345678901234567890123456789012345678901234567890",8, 0x46b1039fe4e9ed20),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()`~-_=+[{]};:,<.>/?",9, 0x1720cb8c72189461),
    ];

    #[rustfmt::skip]
    const GENERATE_SECRET_TEST_VECTOR: [(u64, [u64; 4]); 10] = [
        (0x0, [ 0x1b99a3391da93a1d, 0x78a6a9a98e8795b1, 0xe45cd83956d4591d, 0xe4968be12bb86ac3]),
        (0x1, [ 0x5ad82e99651e8e2b, 0xa993c3394e4b3c63, 0xc54b4e4bb85995a9, 0xe274d4696a8dd20f]),
        (0x2, [ 0x9a35563aca2de44d, 0x9593a59359716c71, 0x5c8e338d7447c54d, 0x665a937278b1d88d]),
        (0x3, [ 0x4bb1ace499a34da5, 0x2b9a8e8e2e552b69, 0xe81d17e81bd1f047, 0xc6e81e36539c8bc5]),
        (0x4, [ 0x56a6b1ac9639394d, 0x8d4d3ce8a66aca65, 0xf0a6ac6a2d478d53, 0x39c6a6b15c2de239]),
        (0x5, [ 0x1b4da54799aa3335, 0xe82ec678c9a50f95, 0x397499e14e932bc9, 0x4d695a4dd463ac2d]),
        (0x6, [ 0x875c4e0f6a952e1b, 0x3a8ea55a69c36a87, 0x7456aa633533b493, 0x9695e84da6e2c363]),
        (0x7, [ 0x3cb2b18b781b8769, 0xa94b879a55d4a31d, 0x8e3a2753934d7163, 0x9c17cce165175517]),
        (0x8, [ 0x93a68ef0a9d43ad1, 0x1d4753391eb19ab1, 0x8b36c68756397159, 0xe893c56c3cc3e853]),
        (0x9, [ 0x4b6a3acac91b9cb1, 0x8e3c694dc9d86a0f, 0xaa2772b44ef01d55, 0xe1b11be22b6c4e1b]),
    ];

    #[test]
    fn test_hasher() {
        for (input, seed, result) in TEST_VECTOR.iter().cloned() {
            let input = input.as_bytes();
            let mut hasher = WyHash32Condom::with_seed(seed).streamed();
            hasher.write(input);
            assert_eq!(hasher.finish(), result);

            for chunksize in 1..=48 {
                let mut hasher = WyHash32Condom::with_seed(seed).streamed();
                for chunk in input.chunks(chunksize) {
                    hasher.write(chunk);
                }
                assert_eq!(
                    hasher.finish(),
                    result,
                    "failed: chunksize: {}, seed: {}, chunks: {:?}",
                    chunksize,
                    seed,
                    input.chunks(chunksize).collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn test() {
        for (input, seed, result) in TEST_VECTOR.iter().cloned() {
            let input = input.as_bytes();
            let hasher = WyHash32Condom::with_seed(seed);
            assert_eq!(
                hasher.hash(input),
                result,
                "input len: {}, seed: {}",
                input.len(),
                seed
            );
        }
    }

    #[test]
    fn test_generate_secret() {
        for (seed, result) in GENERATE_SECRET_TEST_VECTOR.iter().cloned() {
            assert_eq!(WyHash32Condom::generate_secret(seed), result);
        }
    }
}
