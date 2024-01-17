use crate::generics::WyHashVariant;
use crate::util::wy_rotate;

///
/// A type represents the wyhash variant that use 32bit multiplication for mixing.
/// Note that the 32 bit variants of wyhash is faster than 64bit variants on 32bit platforms, but
/// much slower on 64bit platforms.
///
#[derive(Copy, Clone)]
pub struct WyHash32 {
    _private: (),
}

impl WyHashVariant for WyHash32 {
    #[inline(always)]
    fn mul_mum(a: u64, b: u64) -> (u64, u64) {
        let hh = (a >> 32) * (b >> 32);
        let ll = (a as u32 as u64) * (b as u32 as u64);
        let hl = (a >> 32) * (b as u32 as u64);
        let lh = (a as u32 as u64) * (b >> 32);
        (wy_rotate(hl) ^ hh, wy_rotate(lh) ^ ll)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod test {

    use crate::generics::test::TestVector;

    use super::*;

    const TEST_VECTOR: TestVector = [
        ("", 0, 0x4b80acaa567a5c84),
        ("a", 1, 0xb78e7daf065068fa),
        ("abc", 2, 0xd176a04d1bbbff00),
        ("wyhash", 3, 0xb27a7d813cd8bcdc),
        ("message digest", 4, 0x3b7e054046616be3),
        ("abcdefghijklmnopqrstuvwxyz", 5, 0xec124cc1dfaaf40a),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", 6, 0x4745dfacef9c61cb),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", 7, 0xd3ad9a6fbad6eafe),
        ("12345678901234567890123456789012345678901234567890123456789012345678901234567890", 8, 0x8d310928e8f89d37),
        ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()`~-_=+[{]};:,<.>/?", 9, 0x117e23eaf7259a56),
    ];

    #[rustfmt::skip]
    const GENERATE_SECRET_TEST_VECTOR: [(u64, [u64; 4]); 10] = [
        (0x0, [ 0x56d81b53c9e16ad1, 0x7427a6a969a5b8c3, 0xcab8c98b1d2eac4b, 0x39f0a52dcce8692d]),
        (0x1, [ 0xaa6a4b3a2bca8e27, 0x95aaa3e1b1a9a92d, 0xe1c3c666cc93d22d, 0xca71e49539aaf0e1]),
        (0x2, [ 0x87c5e8e4662ec3b1, 0xa9a53c1e964de427, 0x996336c571ca1765, 0xc99c8d8e63b2960f]),
        (0x3, [ 0x53e8d8a327e2a687, 0x0f4eca8db85c740f, 0xac0fc687c3c3ca95, 0x3a394e9c1d271ec3]),
        (0x4, [ 0x1e3959c3a936d195, 0x66d4e1636693741d, 0xaa4d7478c574788b, 0x9c17874d63c59a99]),
        (0x5, [ 0xaaca5cb1c61d6653, 0x59d439722dd17447, 0x71b24d17958d8d2b, 0x874e3c3671749c39]),
        (0x6, [ 0xc9366371e8272d71, 0x5c93d13996c3b8c5, 0xac960f8e956a3371, 0x3ca566991bb4c969]),
        (0x7, [ 0x27e2594be8d478a9, 0x2b1d744d4b1eb165, 0xe2656917e1990f47, 0x72e4b253936695c9]),
        (0x8, [ 0xa3951e4eaa1b3693, 0x552ed8b2a9c32b93, 0x72a666aa1ed1f08d, 0xb28e9578e8e8d171]),
        (0x9, [ 0x6a2ee8594d361d69, 0x96746aac470fcc35, 0x8e72d47174c393a5, 0xd2e48b1771e474a3]),
    ];

    #[test]
    fn test_streaming() {
        for (input, seed, result) in TEST_VECTOR.iter().cloned() {
            let input = input.as_bytes();
            let mut hasher = WyHash32::with_seed(seed).streamed();
            hasher.write(input);
            assert_eq!(hasher.finish(), result);

            for chunksize in 1..=48 {
                let mut hasher = WyHash32::with_seed(seed).streamed();
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
            let hasher = WyHash32::with_seed(seed);
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
            assert_eq!(WyHash32::generate_secret(seed), result);
        }
    }
}
