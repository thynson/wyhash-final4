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

    use super::*;
    use crate::generics::test::*;

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

    const EXTENDED_TEST_VECTOR: ExtendedTestVector = [
        (0, 0x4b80acaa567a5c84u64),
        (1, 0xf4d6babc623413cbu64),
        (2, 0x33bcbc7c16534995u64),
        (3, 0xd2a2f48f6f76868fu64),
        (4, 0x24dcaf7dfd053b04u64),
        (5, 0xdba1f7bd99a41852u64),
        (6, 0xa8c532dbd68620acu64),
        (7, 0x4c90bdb5d2f10a9fu64),
        (8, 0x34cb23888c74aaf6u64),
        (9, 0xd691726b6b9e3ff1u64),
        (10, 0xc3cac02fa1deccadu64),
        (11, 0x70c2e407f14cc049u64),
        (12, 0x0b07fe1d24435decu64),
        (13, 0x350a2c90f60b74a4u64),
        (14, 0x63067f0ab4ee160eu64),
        (15, 0x2310acc47cee0924u64),
        (16, 0xc59ad274d3bc0bfeu64),
        (17, 0x4622e3070a32ea11u64),
        (18, 0x85dbc199ea2467bfu64),
        (19, 0xb82bd53d0a84ee59u64),
        (20, 0x93ff08769555ea36u64),
        (21, 0x8aa89d80c712e7a8u64),
        (22, 0x0239272637b59813u64),
        (23, 0x9c2b3b66ea7a1940u64),
        (24, 0xc54172c76e09ec16u64),
        (25, 0x360de87e229071b4u64),
        (26, 0x27a21b39cdc07467u64),
        (27, 0x0806c510f32fdd04u64),
        (28, 0x549d962d730d2aaeu64),
        (29, 0xb3261afd93a6941au64),
        (30, 0xf7af604fd62b7430u64),
        (31, 0xdd2583ce299fa1b2u64),
        (32, 0xb7c1b788b49b8283u64),
        (33, 0xa5b7fb6e4a2479fbu64),
        (34, 0x67982fc9bf0ecd9fu64),
        (35, 0xf3579d64131c3b66u64),
        (36, 0xa9a0fed45165d3a4u64),
        (37, 0x733934e00e257c57u64),
        (38, 0xa4e22f0b3d4346f7u64),
        (39, 0x2af16a2561f14f01u64),
        (40, 0x9fee6a293eec61f8u64),
        (41, 0x7f2d2487bc08b095u64),
        (42, 0x147d054901144774u64),
        (43, 0xf71dd6dcebfdbf6au64),
        (44, 0xeae96c7e03ab2e89u64),
        (45, 0x16c0785b8b15d44au64),
        (46, 0xf03737eea59fc9f3u64),
        (47, 0x15f8c86c006956b5u64),
        (48, 0xca4621d9b62f8f38u64),
        (49, 0xea92d235d32fb6a8u64),
        (50, 0x6f90269b70c69f77u64),
        (51, 0xe776985c576ef082u64),
        (52, 0xb35ab3374da886d3u64),
        (53, 0x16536028be894c3du64),
        (54, 0xfd3fc21c3a9be100u64),
        (55, 0x79ecc98aa93f26d7u64),
        (56, 0x1c56ed673bebd996u64),
        (57, 0xae76810dd7d34c7fu64),
        (58, 0x0dd1d3a4523c0825u64),
        (59, 0x00af729b18f9daf0u64),
        (60, 0x1e6c14f5e6c97e7au64),
        (61, 0x9f1c01a41f9cd573u64),
        (62, 0x129cb0285ebb101du64),
        (63, 0x84557d24231fd3f6u64),
        (64, 0xa1489cd2d8d20c4bu64),
        (65, 0x50a349beabe17ed7u64),
        (66, 0xdd2a814a408f78e6u64),
        (67, 0xf26552152e27c5d2u64),
        (68, 0x7417ff4206aebd85u64),
        (69, 0x9e3b14d9a0ba00b3u64),
        (70, 0xaf4cc62ce54fd4fdu64),
        (71, 0xd76f364e069a08f9u64),
        (72, 0xfe07d7e9aac788a1u64),
        (73, 0x206bf22e036791ffu64),
        (74, 0xf37b6cb6f44d2a87u64),
        (75, 0x378da81cda027835u64),
        (76, 0x01b3b412eac3cc7cu64),
        (77, 0xc45bd053a3e083ebu64),
        (78, 0xb63d0cfbe4bf96edu64),
        (79, 0xd906f7385921b990u64),
        (80, 0x2eff6e684b628e3cu64),
        (81, 0x6d8864c72afaa023u64),
        (82, 0x29f635850c4e085fu64),
        (83, 0x7d7425d22b1ab91au64),
        (84, 0x447826b563795358u64),
        (85, 0xf2b8f0575cc1af5cu64),
        (86, 0x31609413a648ff75u64),
        (87, 0x5deafc68e5fd755cu64),
        (88, 0xdc77488324d8ffcbu64),
        (89, 0x4207f6bca3f7e593u64),
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
    fn extended_test() {
        for (seed_and_len, result) in EXTENDED_TEST_VECTOR.iter() {
            let input = &EXTENDED_TEST_VECTOR_BUFFER[..*seed_and_len];
            let hasher = WyHash32::with_seed(*seed_and_len as u64);
            assert_eq!(
                hasher.hash(input),
                *result,
                "length_and_len: {}",
                seed_and_len
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
