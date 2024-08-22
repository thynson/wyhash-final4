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

    use super::*;
    use crate::generics::test::*;

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

    const EXTENDED_TEST_VECTOR: ExtendedTestVector = [
        (0, 0x090d3db895794f51u64),
        (1, 0x5c4240833cac92a0u64),
        (2, 0x5927a76ccdbd2a69u64),
        (3, 0x3c3dff9e28ee3aa7u64),
        (4, 0xc6032b2421a39485u64),
        (5, 0x493060b44c8ba393u64),
        (6, 0xc6530c1ab0e75661u64),
        (7, 0xb9fd91ea39b70baau64),
        (8, 0x4cbead4b00cec38bu64),
        (9, 0x7fec2df588a51419u64),
        (10, 0x4cb68d38a9f9bef6u64),
        (11, 0x5706765edcd6c4e9u64),
        (12, 0x48a0733e5e291d68u64),
        (13, 0x3751610c7b6fe13bu64),
        (14, 0x1ebf5276bcd25770u64),
        (15, 0x2500a76517877410u64),
        (16, 0xa3bea891c98779edu64),
        (17, 0x3978cded66b9903bu64),
        (18, 0x7090de6a888d3c79u64),
        (19, 0xc2711748ba14a40au64),
        (20, 0x2451971a98eab449u64),
        (21, 0x6a90d0bb8b7ebc47u64),
        (22, 0x948534b134e8dde6u64),
        (23, 0xede3dffb7a52a62fu64),
        (24, 0x736bf9ff3239e3f3u64),
        (25, 0x2799da6a7fd994e1u64),
        (26, 0x06febf33f1d2b793u64),
        (27, 0xca54f734464b4f71u64),
        (28, 0x56d05dc05c98eaceu64),
        (29, 0x7f54dc1e812dd741u64),
        (30, 0x83f7e64ace8c1476u64),
        (31, 0x497a876b86954adcu64),
        (32, 0x4927d53c964c8480u64),
        (33, 0x54e1e1b460317b98u64),
        (34, 0x9a08ec3cf5f2566fu64),
        (35, 0xa24fbbc0c9a33004u64),
        (36, 0x7ac5450726a37c49u64),
        (37, 0xd74c1aff391bc2aeu64),
        (38, 0xe8474002dc7fb0feu64),
        (39, 0x3bef8e8a1c3d7033u64),
        (40, 0x717143c8ad25c7f5u64),
        (41, 0x27871753c1e5d391u64),
        (42, 0x9d5c533b782068c1u64),
        (43, 0x3af774fefc196ce3u64),
        (44, 0xa62a6bd4afa723c2u64),
        (45, 0xe50a27ecd4ba0752u64),
        (46, 0x56d9fc55c6081abeu64),
        (47, 0x0fbd0dcf98edba62u64),
        (48, 0xbf5be5288b96b228u64),
        (49, 0x029cf445f50d1263u64),
        (50, 0xfc942adda4ec648eu64),
        (51, 0x1e522a359961cc78u64),
        (52, 0x13d193aa80589b02u64),
        (53, 0x33016678373b43e2u64),
        (54, 0x6eb00f0bb2685af7u64),
        (55, 0xc72e87aa395f765bu64),
        (56, 0xd70aca673033547au64),
        (57, 0xde37287385c1f052u64),
        (58, 0x985497f486c810a9u64),
        (59, 0x61424d0fba2d2f6cu64),
        (60, 0x8ff12a13a3a52958u64),
        (61, 0x7dfd0d81f5605557u64),
        (62, 0x35874d831d91363du64),
        (63, 0x090cbe31bde3cd6eu64),
        (64, 0x5673caeab747c2dfu64),
        (65, 0xfdcc5d843d2aeefcu64),
        (66, 0x0730eef46d5eff5au64),
        (67, 0x06f532d1ab13fd07u64),
        (68, 0x78d8fd8e47b88c0bu64),
        (69, 0xa2e297245f54506fu64),
        (70, 0x435a224b0a450613u64),
        (71, 0x4b7e19a876c98f1du64),
        (72, 0x8328a42038acb4ceu64),
        (73, 0x2f8e484b926abe12u64),
        (74, 0xd367912efef11e56u64),
        (75, 0x015058b27d0ac7eeu64),
        (76, 0x1c9d5a01c1df32a2u64),
        (77, 0xd02d8524b08b3f54u64),
        (78, 0x504628395f10a642u64),
        (79, 0x9afe5b40d807e2cfu64),
        (80, 0x636227bd9452ce91u64),
        (81, 0xcf57942f1cb2f9f5u64),
        (82, 0xda6e2a1cb5e4be73u64),
        (83, 0x1589a625fa349726u64),
        (84, 0xdb5b2d5fffb61e34u64),
        (85, 0x42c4b15e3e726b46u64),
        (86, 0x8781ec21dc327557u64),
        (87, 0x16c9dabc4cab3fffu64),
        (88, 0x5cc29141b01772abu64),
        (89, 0xfd6b9586898e0ec4u64),
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
    fn extended_test() {
        for (seed_and_len, result) in EXTENDED_TEST_VECTOR.iter() {
            let input = &EXTENDED_TEST_VECTOR_BUFFER[..*seed_and_len];
            let hasher = WyHash64Condom::with_seed(*seed_and_len as u64);
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
            assert_eq!(WyHash64Condom::generate_secret(seed), result);
        }
    }
}
