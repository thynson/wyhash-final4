use crate::generics::WyHashVariant;

///
/// A type represents the wyhash variant that use 64bit multiplication for mixing,
/// which is the default variant of wyhash, and should be faster than all other variants.
///
#[derive(Copy, Clone)]
pub struct WyHash64 {
    _private: (),
}

impl WyHashVariant for WyHash64 {
    #[inline(always)]
    fn mul_mum(a: u64, b: u64) -> (u64, u64) {
        let m = (a as u128) * (b as u128);
        (m as u64, (m >> 64) as u64)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod test {
    use crate::generics::test::TestVector;

    use super::*;

    const TEST_VECTOR: TestVector = [
        ("",0, 0x409638ee2bde459),
        ("a",1, 0xa8412d091b5fe0a9),
        ("abc",2, 0x32dd92e4b2915153),
        ("wyhash",3, 0x12c7ef9314847163),
        ("message digest",4, 0xa2608b1b6ec6ebbf),
        ("abcdefghijklmnopqrstuvwxyz",5, 0x3f213f97faf16439),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",6, 0xd730106570676f54),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",7, 0x2e9c2f693249ca6),
        ("12345678901234567890123456789012345678901234567890123456789012345678901234567890",8, 0x3c353b8b0b931bb0),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()`~-_=+[{]};:,<.>/?",9, 0x3903fc47ba3ef81c),
    ];

    #[rustfmt::skip]
    const GENERATE_SECRET_TEST_VECTOR: [(u64, [u64; 4]); 10] = [
        (0x0, [0x95d49a959ca5a395, 0xb4a9716ac94da695, 0x5635cc6355956559, 0xe1e18e3a9c591da9]),
        (0x1, [0x8b66d82b5ccaac2b, 0xf08d3cc98ecae895, 0x72b4c64e6a1dcc27, 0x1ee1c995c9c9d187]),
        (0x2, [0xa98ec3e887b15327, 0x72cc33657895392b, 0xd2a556e187d2b8d1, 0xd499938d3aa3c693]),
        (0x3, [0xa9c64d71a6e2a3c9, 0x5cac27591d9ad1e1, 0x3574d14eb45987a5, 0xd8b85963273c4d1d]),
        (0x4, [0x6a36a69cacd16ca3, 0x4b9335f0d2aca963, 0x365c2d66ac3ca669, 0x33599959e2d26ad1]),
        (0x5, [0x8be499e8275c0fa5, 0x170fd2d1b2f02799, 0x2dac724b598b87a3, 0xb471d447935acc35]),
        (0x6, [0x4dc3d12e36b1272d, 0xaa5a8b35b4781d1b, 0xcc36354be4e24e4b, 0x3c554da34d748787]),
        (0x7, [0x4e72aca5a6a95cc3, 0x35cca54daa596363, 0x6ce24e1b4e78b159, 0x53f0f0d8e8a6711b]),
        (0x8, [0x5c2e2ee45ab17435, 0x63552e78d863a3a9, 0x55d1f0d15c931e1b, 0x3cca1e4de15a3a2b]),
        (0x9, [0xf04eccd1e259f099, 0xa91b590f9399a92d, 0xd2c6e18d1d0fc971, 0x55c5476695c9744d]),
    ];

    #[test]
    fn test_hasher() {
        for (input, seed, result) in TEST_VECTOR.iter().cloned() {
            let input = input.as_bytes();
            let mut hasher = WyHash64::with_seed(seed).streamed();
            hasher.write(input);
            assert_eq!(hasher.finish(), result);

            for chunksize in 1..=48 {
                let mut hasher = WyHash64::with_seed(seed).streamed();
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
            let hasher = WyHash64::with_seed(seed);
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
            assert_eq!(WyHash64::generate_secret(seed), result);
        }
    }
}
