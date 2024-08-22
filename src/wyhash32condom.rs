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

    use super::*;
    use crate::generics::test::*;

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

    const EXTENDED_TEST_VECTOR: ExtendedTestVector = [
        (0, 0xeea54221671289dbu64),
        (1, 0x020dd805695fc9eau64),
        (2, 0x33af332bbf5c9234u64),
        (3, 0x537caa1f96a07a21u64),
        (4, 0x5b937d3918b03427u64),
        (5, 0x00cd6d0d8b25806eu64),
        (6, 0x8b95df2cdc0a6228u64),
        (7, 0x0d78f4861e71a79du64),
        (8, 0xe92a71a082d5fab3u64),
        (9, 0x5355df371ef800ceu64),
        (10, 0xf9d899203066423cu64),
        (11, 0xa1eae7ff8f78bf13u64),
        (12, 0x5af17e7eef210373u64),
        (13, 0x6e613cd3184da253u64),
        (14, 0x3e1ba8003dfc7992u64),
        (15, 0x99aacc4d775059acu64),
        (16, 0x6331047b9e708282u64),
        (17, 0x0883b8dae86ca6c3u64),
        (18, 0xf785b14ce090e08eu64),
        (19, 0x02b1a401361aa789u64),
        (20, 0x17a30b7d5ae318c7u64),
        (21, 0x90b8e7a43a25660bu64),
        (22, 0x7dfdd0bf60a1250fu64),
        (23, 0xd5e78b11dacaccd6u64),
        (24, 0x65bee070ba4c8110u64),
        (25, 0xec2cdcdd17ab8c29u64),
        (26, 0x47287af2d365ba3cu64),
        (27, 0xa5a3e4cf39c7160eu64),
        (28, 0x4899fac01efb933eu64),
        (29, 0x67384cc63cd7c8acu64),
        (30, 0xd27bfbc2a2d3eac1u64),
        (31, 0x3a9f7b220e2bf222u64),
        (32, 0xb8a204cef1457013u64),
        (33, 0x4081d84535383e70u64),
        (34, 0xa4756d9516a80cc7u64),
        (35, 0x72e1d8b42b4d146bu64),
        (36, 0x266a25b74a3151d7u64),
        (37, 0x27d9f919c5be9195u64),
        (38, 0x12c278e155532207u64),
        (39, 0x65a631b85343b8f2u64),
        (40, 0x7ded684962edd62du64),
        (41, 0x9037050a2a44485cu64),
        (42, 0x2351e370d40fd2c7u64),
        (43, 0x1eefaa3c7edc4689u64),
        (44, 0x578d921d5d55bab3u64),
        (45, 0xb43038c904896b8fu64),
        (46, 0x8e47d05f5d29860cu64),
        (47, 0xdc592bda10ad8d49u64),
        (48, 0x7577fd8b9180a386u64),
        (49, 0x3ba1a2369b07f5f6u64),
        (50, 0x6b145894e9881814u64),
        (51, 0x3ec2e61b38dffd10u64),
        (52, 0xab20eee97d635eb1u64),
        (53, 0x8002818b7b5044f2u64),
        (54, 0x11df779270f4a4b2u64),
        (55, 0x086866ef240dd476u64),
        (56, 0xfb8c21696ac39f0bu64),
        (57, 0x8fdc8f0be7e9b480u64),
        (58, 0xc00775ad06f4f739u64),
        (59, 0x84a1666391a9580du64),
        (60, 0x5dce484a6cbc46e9u64),
        (61, 0x34217cc36cb14a63u64),
        (62, 0x1c62e7ae82902269u64),
        (63, 0x9140fbc329ab1960u64),
        (64, 0x4f5c1648ac9afbeau64),
        (65, 0x1677fce470e1a07bu64),
        (66, 0x503b5ab4ca36e5c9u64),
        (67, 0xd373c09fde0e770bu64),
        (68, 0xcbe3c0bb6f6a91d8u64),
        (69, 0xb3b5baa53e21b1c3u64),
        (70, 0x71c1f519064b6a14u64),
        (71, 0x78ce84779fb58c5du64),
        (72, 0xc970371e56ab1452u64),
        (73, 0x7c5845ede0515c3au64),
        (74, 0xd5478f981786fe25u64),
        (75, 0x4107ed3def81fe76u64),
        (76, 0x745242d260bc49b6u64),
        (77, 0xea830819a4a334fau64),
        (78, 0x93a12835843cf4e7u64),
        (79, 0x2b5639a41e360d38u64),
        (80, 0x3396a3191b1a2579u64),
        (81, 0x27381a78006c56a9u64),
        (82, 0x477669819f338d4bu64),
        (83, 0x7fc41434a2c61294u64),
        (84, 0x9d7374c51692f396u64),
        (85, 0xf6ee34eaaed19adau64),
        (86, 0xbcccf2482d285196u64),
        (87, 0x92d04ffc87eb5fc5u64),
        (88, 0xce3da2cf9b1d9262u64),
        (89, 0xd44e342f275ce012u64),
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
    fn extended_test() {
        for (seed_and_len, result) in EXTENDED_TEST_VECTOR.iter() {
            let input = &EXTENDED_TEST_VECTOR_BUFFER[..*seed_and_len];
            let hasher = WyHash32Condom::with_seed(*seed_and_len as u64);
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
            assert_eq!(WyHash32Condom::generate_secret(seed), result);
        }
    }
}
