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
    use super::*;
    use crate::generics::test::*;

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

    const EXTENDED_TEST_VECTOR: ExtendedTestVector = [
        (0, 0x0409638ee2bde459u64),
        (1, 0x4f5ee2fada5971b1u64),
        (2, 0x38385a88e8c86c55u64),
        (3, 0xcaf4058fb822e46du64),
        (4, 0x577959bb8c92d137u64),
        (5, 0x462098e78d5489bdu64),
        (6, 0xa8939b328ba4cdb6u64),
        (7, 0xe8d098bc8ab18d25u64),
        (8, 0xc2a03b43bda0959du64),
        (9, 0xd48b21549a2e9c78u64),
        (10, 0x378f2b3405be18c8u64),
        (11, 0x5d6cf1dfa4722205u64),
        (12, 0x1e47d7c33baed53fu64),
        (13, 0xf05ed15251d76f60u64),
        (14, 0xafd90ad831ade132u64),
        (15, 0xf9d1d53f1571bcceu64),
        (16, 0xd84ae114eecf57c8u64),
        (17, 0xb98eb4548a1355a9u64),
        (18, 0x9670a3f585f79bb2u64),
        (19, 0x63e0578f867dd7e4u64),
        (20, 0x9e1b90a532be1cd1u64),
        (21, 0xd41fa72a9c3f2d95u64),
        (22, 0x5aedf9010b08009au64),
        (23, 0xbb66a3913a6f4ba5u64),
        (24, 0x0fc2b8b82893f418u64),
        (25, 0xf54f0e7a723f6446u64),
        (26, 0xc34098c7e840ec7bu64),
        (27, 0x4c9653f33bc3462au64),
        (28, 0x186d92e2195fd004u64),
        (29, 0xa39278b73e4aec27u64),
        (30, 0xe8ede06fa26da11eu64),
        (31, 0x69825a21750f8b48u64),
        (32, 0x55fe588f50750dbfu64),
        (33, 0x2dc683f89eaa4cbeu64),
        (34, 0xe9084ed9f9401e67u64),
        (35, 0x192488353ca3fc8cu64),
        (36, 0xc3898e0d9bc56685u64),
        (37, 0x97ea96f0e929825au64),
        (38, 0xd6cb8e47bf514ad2u64),
        (39, 0xd9a60ba666a5a7cbu64),
        (40, 0x7b0b61c2b80836c8u64),
        (41, 0xd13f2e36a677a32bu64),
        (42, 0xbc5b7db0dc92ee47u64),
        (43, 0x1802decf7bcd9645u64),
        (44, 0x810e26487f83479cu64),
        (45, 0xc6fbdc911dd3e6bdu64),
        (46, 0xb8fca7cb2234b8e1u64),
        (47, 0x90098186b0da3c28u64),
        (48, 0x18c54464ffead7f9u64),
        (49, 0x1179ee16801c99cbu64),
        (50, 0x2439fc2ad6179bd1u64),
        (51, 0x88a2673d32d71841u64),
        (52, 0x19d9883a6a0b8ec1u64),
        (53, 0xb5998bcc697b7b80u64),
        (54, 0x020eb14887a2d81cu64),
        (55, 0xb10e09d9754e6dddu64),
        (56, 0xa2af8b8a42753df2u64),
        (57, 0x5d2b6bca40cc4c97u64),
        (58, 0xc6736c7800815fb7u64),
        (59, 0x00bdb3dba121ea8bu64),
        (60, 0xa4d3d2b6b47ce26fu64),
        (61, 0xb8df6502550b3fbau64),
        (62, 0xd117d52ab437f5fau64),
        (63, 0x2d0c4b1b601acd69u64),
        (64, 0xa47a2a6a78458156u64),
        (65, 0x12db9420a984f7a9u64),
        (66, 0x181d11f0bbbd79c8u64),
        (67, 0x2080bb74bec9c5a6u64),
        (68, 0x270457eb27bb0f2fu64),
        (69, 0xa3af65efb0a2797au64),
        (70, 0x57922eda98f55151u64),
        (71, 0xca25356571b89b3du64),
        (72, 0x3ba5127b63c36011u64),
        (73, 0xaadf5575d72cf300u64),
        (74, 0xbf558044cc83ee90u64),
        (75, 0x6516f1c6f678ba51u64),
        (76, 0x1a2a1d72ef943881u64),
        (77, 0xaf2df500ed13763du64),
        (78, 0x586b5c04b2fdd181u64),
        (79, 0xb4fd4867dbdeed17u64),
        (80, 0xafbed6ac26cdc367u64),
        (81, 0x10a5b552abf2dc58u64),
        (82, 0x9a4833cbf59c88b5u64),
        (83, 0xb6763e82342bf04du64),
        (84, 0x1c181f90970213c4u64),
        (85, 0x27ea3bfe37f8daaau64),
        (86, 0xfc516fa0c4b2bf85u64),
        (87, 0x8ffeadb10a8947edu64),
        (88, 0x15700e9bae6549d8u64),
        (89, 0xf30f51889a34404cu64),
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
    fn extended_test() {
        for (seed_and_len, result) in EXTENDED_TEST_VECTOR.iter() {
            let input = &EXTENDED_TEST_VECTOR_BUFFER[..*seed_and_len];
            let hasher = WyHash64::with_seed(*seed_and_len as u64);
            assert_eq!(
                hasher.hash(input),
                *result,
                "length_and_len: {}",
                seed_and_len
            );

            for chunksize in 1..=48 {
                let mut hasher = WyHash64::with_seed(*seed_and_len as u64).streamed();
                for chunk in input.chunks(chunksize) {
                    hasher.write(chunk);
                }
                assert_eq!(
                    hasher.finish(),
                    *result,
                    "failed: chunksize: {}, seed: {}, chunks: {:?}",
                    chunksize,
                    *seed_and_len as u64,
                    input.chunks(chunksize).collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn test_generate_secret() {
        for (seed, result) in GENERATE_SECRET_TEST_VECTOR.iter().cloned() {
            assert_eq!(WyHash64::generate_secret(seed), result);
        }
    }
}
