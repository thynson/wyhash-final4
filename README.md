wyhash-final4
=============

This crate provides a pure rust implementation of wyhash_final4, the latest
version of wyhash as of January 2024; wyhash which is an extremely fast hash 
function that can be implemented without any machine-specific instructions,
while producing high quality hash result. See [smhasher] for hash function
comparison details.

Related works
-------------
There are several other implementations of wyhash in rust, but none of them
provides the latest version of wyhash (that is, the `final4`). 
- [wyhash](https://github.com/eldruin/wyhash-rs)
- [wy](https://github.com/DoumanAsh/wyhash)
- [wyhash2](https://crates.io/crates/wyhash2) 

Variants
--------
This crate provides all the 4 variants of wyhash, namely

- `WyHash64`, the default variant, using 64-bit multiplication for mixing.
- `WyHash64Condom`, using 64-bit multiplication and an extra bit-xor for mixing
- `WyHash32`, using 32-bit multiplication for mixing.
- `WyHash32Condom`, using 32-bit multiplication and an extra bit-xor for mixing

It worth nothing to note that,
- the 32-bit variants are faster on 32-bit platform, but much slower on 64-bit platform;
- the `Condom` variants are slightly slower due to an extra bit-xor was used when mixing.
  As wyhash use multiplication for mixing, for some particular input (with a negligible 
  probability of 2^-64) the mixing becomes multiplying by zero, thus losing all the entropy.
  The `Condom` variants are resistant to this problem thanks to the extra bit-xor.
 
If you are not sure which variant to use, just use `WyHash64`.

Usage
-----
This crate provides three ways to hash inputs:

- One-shot hashing, e.g. just call `WyHash64::hash(input)` to get the hash result. This is
  the simplest and the recommended way when default seed and secrets are used.
  There are also `Wyhash::hash_with_seed` and `Wyhash::hash_with_secret` for one-shot hashing
  with custom seed and secret, but unless the seed and secret changed over time, it's better
  to use the following methods.

- Create an instance of`WyHash` with `with_seed` or `with_seed_and_secret` before hashing, e.g.

  ```
  let hasher = WyHash64::with_seed(fix_seed);
  hasher.hash(input1);
  hasher.hash(input2);
  ```
  
  The `hasher` can be reused for hashing multiple inputs, when seed and secret are fixed. It's
  faster than one-shot hashing in this scenario, as the initialization work will be done only
  once.

- Streamed hashing. When the input is too large to fit into a single buffer, or the length of 
  input is unknown at the beginning, streamed hasher can be used.
  
  ```
  let mut hasher = WyHash64::with_seed(seed).streamed();
  hasher.write(chunk1);
  hasher.write(chunk2);
  hasher.write(chunk3);
  let hash = hasher.finish();
  ```
  
Also, `WyHasher` implements the `std::hash::Hasher` and `std::hash::BuildHasher` trait, thus can be 
used as a custom hasher for `HashMap` and `HashSet`.

Build Features
-------------

There are several build features that can be enabled or disabled:

- `std`. Disable this feature will make `std::hash::Hasher` and `std::hash::BuildHasher` 
  not be implemented for `WhHasher`, while make this crate `no_std` compatible.

- `wyhash64`, `wyhash64_condom`, `wyhash32`, `wyhash32_condom`. Each of these features enables 
  the corresponding variant of wyhash.

All the features are enabled by default.

[smhasher]: https://github.com/rurban/smhasher

License
=======

Public domain. See [LICENSE](LICENSE) for details.