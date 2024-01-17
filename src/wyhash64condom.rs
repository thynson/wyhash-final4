use crate::generics::WyHashVariant;

///
/// A type represents the wyhash variant that use 64bit multiplication and an extra xor
/// for mixing, which survives against entropy loss that could happens in the probability of 2^-64.
///
#[derive(Copy, Clone)]
pub struct WyHash64Condom {
    _private: (),
}

impl WyHashVariant for WyHash64Condom {
    #[inline(always)]
    fn mul_mum(mut a: u64, mut b: u64) -> (u64, u64) {
        let m = (a as u128) * (b as u128);
        a ^= m as u64;
        b ^= (m >> 64) as u64;
        (a, b)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
pub mod test {

    use crate::generics::test::TestVector;

    use super::*;

    const TEST_VECTOR: TestVector = [
        ("",0, 0x90d3db895794f51),
        ("a",1, 0xc9654dd2d8b02dce),
        ("abc",2, 0xc04b780dfa37c941),
        ("wyhash",3, 0x7608115e227884a3),
        ("message digest",4, 0x4ec3b6028298a4b5),
        ("abcdefghijklmnopqrstuvwxyz",5, 0x65302f854904d2f1),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",6, 0xb87c8f2de51183b6),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",7, 0xb01396bb9606984c),
        ("12345678901234567890123456789012345678901234567890123456789012345678901234567890",8, 0xb50e1d83321bb20f),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()`~-_=+[{]};:,<.>/?",9, 0x3b46861b56efcd2), 
    ];

    #[rustfmt::skip]
    const GENERATE_SECRET_TEST_VECTOR: [(u64, [u64; 4]); 10] = [
        (0x0, [0x3655e8b13a63d847, 0x0f7195a31d1b2e71, 0x5ca369ac1bb8b169, 0xe133f04bd85ad133]),
        (0x1, [0x4b35931736b2e147, 0x635a8dc9a5a5a51d, 0x3c27d1c9aa1739a5, 0x1ee29617e127ca39]),
        (0x2, [0x8b74ca5353f0f0e1, 0x2dc6782d33b48d27, 0x6936964d1e4e4dc9, 0x4d5cf01be4532b93]),
        (0x3, [0x66552dd2aa1dd2c9, 0x55334bac99c9c393, 0x1bc571c9d4ac5999, 0x1d965cd247c687c5]),
        (0x4, [0xa935e28d72356c0f, 0x749593696359a5d1, 0x6a1b0fb4470f6ac3, 0xb82d2b55ac4e1d59]),
        (0x5, [0x53993a1ed83c6393, 0xb88b2d65cc2778c9, 0x5965a5360f39c58d, 0x47c62e55a3a32ba5]),
        (0x6, [0xb29517aaa5d4e463, 0xd43aaa17954d6c33, 0x36c64e693c276ab1, 0x4b9ad859a69565c5]),
        (0x7, [0xb1711df0959a330f, 0x69634e3a4ba98b55, 0x87f0e4a9d11d59d1, 0x66cca5c687e89353]),
        (0x8, [0xaa17c33ca54b2e87, 0x3a59e84e36b1c947, 0xacb155787878c335, 0x9636cce2b82e998b]),
        (0x9, [0x55a91bd82b9387a9, 0x99b18da53663554d, 0x477453171d56d165, 0x9ae2748e69b2c94b]),
    ];

    #[test]
    fn test_hasher() {
        for (input, seed, result) in TEST_VECTOR.iter().cloned() {
            let input = input.as_bytes();
            let mut hasher = WyHash64Condom::with_seed(seed).streamed();
            hasher.write(input);
            assert_eq!(hasher.finish(), result);

            for chunksize in 1..=48 {
                let mut hasher = WyHash64Condom::with_seed(seed).streamed();
                for chunk in input.chunks(chunksize) {
                    hasher.write(chunk);
                }
                assert_eq!(
                    hasher.finish(),
                    result,
                    "input len: {}, seed: {}, chunksize: {}",
                    input.len(),
                    seed,
                    chunksize
                );
            }
        }
    }

    #[test]
    fn test() {
        for (input, seed, result) in TEST_VECTOR.iter().cloned() {
            let input = input.as_bytes();
            let hasher = WyHash64Condom::with_seed(seed);
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
            assert_eq!(WyHash64Condom::generate_secret(seed), result);
        }
    }
}
