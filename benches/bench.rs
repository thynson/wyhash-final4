use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::time::{SystemTime, UNIX_EPOCH};
use wyhash_final4::generics::WyHashVariant;
use wyhash_final4::wyhash32::*;
use wyhash_final4::wyhash32condom::*;
use wyhash_final4::wyhash64::*;
use wyhash_final4::wyhash64condom::*;

macro_rules! impl_oneshot_bench {
    ($brand: ident, $variant: ident; $($bench: ident, $size: literal);* ) => {
        mod $brand {
            use super::*;
            fn bench(c: &mut Criterion) {
                $(
                    c.bench_function(
                        format!("{}::{}", stringify!($brand), stringify!($bench)).as_str(),
                        |b| {
                        let mut content = [0u8; $size];
                        let mut seed = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64;
                        content.chunks_mut(8).for_each(|chunk| {
                            chunk.copy_from_slice(&seed.to_le_bytes()[..chunk.len()]);
                        });
                        b.iter(move || {
                            seed ^= black_box(<$variant>::hash_with_seed(&content, seed));
                        });
                    });
                )*
            }
            criterion_group!(
                name=benches;
                config=Criterion::default()
                    .sample_size(100)
                    .warm_up_time(std::time::Duration::from_millis(5))
                    .measurement_time(std::time::Duration::from_millis(50));
                targets=bench
            );
        }
    };
}

macro_rules! impl_hasher_bench {
    ($brand: ident, $variant: ty; $($bench: ident, $size: literal);* ) => {
        mod $brand {
            use super::*;
            fn bench(c: &mut Criterion) {
                $(
                    c.bench_function(
                        format!("{}::{}", stringify!($brand), stringify!($bench)).as_str(),
                        |b| {
                        let mut content = [0u8; $size];
                        let seed = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64;
                        content.chunks_mut(8).for_each(|chunk| {
                            chunk.copy_from_slice(&seed.to_le_bytes()[..chunk.len()]);
                        });
                        let hasher = <$variant>::with_seed(seed);
                        b.iter(move || {
                            black_box(hasher.hash(&content));
                        });
                    });
                )*
            }
            criterion_group!(
                name=benches;
                config=Criterion::default()
                    .sample_size(100)
                    .warm_up_time(std::time::Duration::from_millis(5))
                    .measurement_time(std::time::Duration::from_millis(50));
                targets=bench
            );
        }
    };
}

macro_rules! impl_streamed_bench {
    ($brand: ident, $variant: ty; $($bench: ident, $size: literal);* ) => {
        mod $brand {
            use super::*;
            fn bench(c: &mut Criterion) {
                $(
                    c.bench_function(
                        format!("{}::{}", stringify!($brand), stringify!($bench)).as_str(),
                        |b| {
                        let mut content = [0u8; $size];
                        let seed = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64;
                        content.chunks_mut(8).for_each(|chunk| {
                            chunk.copy_from_slice(&seed.to_le_bytes()[..chunk.len()]);
                        });
                        let hasher = <$variant>::with_seed(seed);
                        b.iter(move || {
                            let mut hasher = hasher.streamed();
                            hasher.write(&content);
                            black_box(hasher.finish());
                        });
                    });
                )*
            }
            criterion_group!(
                name=benches;
                config=Criterion::default()
                    .sample_size(1000)
                    .warm_up_time(std::time::Duration::from_millis(5))
                    .measurement_time(std::time::Duration::from_millis(50));
                targets=bench
            );
        }
    };
}

impl_oneshot_bench!(
    wyhash64_oneshot,
    WyHash64;
    oneshot_00004bytes, 4;
    oneshot_00007bytes, 7;
    oneshot_00008bytes, 8;
    oneshot_00012bytes, 12;
    oneshot_00015bytes, 15;
    oneshot_00016bytes, 16;
    oneshot_00023bytes, 23;
    oneshot_00024bytes, 24;
    oneshot_00031bytes, 31;
    oneshot_00032bytes, 32;
    oneshot_00064bytes, 64;
    oneshot_00256bytes, 256;
    oneshot_01024bytes, 1024;
    oneshot_04096bytes, 4096;
    oneshot_16384bytes, 16384
);

impl_hasher_bench!(
    wyhash64_hasher,
    WyHash64;
    hasher_00004bytes, 4;
    hasher_00007bytes, 7;
    hasher_00008bytes, 8;
    hasher_00012bytes, 12;
    hasher_00015bytes, 15;
    hasher_00016bytes, 16;
    hasher_00023bytes, 23;
    hasher_00024bytes, 24;
    hasher_00031bytes, 31;
    hasher_00032bytes, 32;
    hasher_00064bytes, 64;
    hasher_00256bytes, 256;
    hasher_01024bytes, 1024;
    hasher_04096bytes, 4096;
    hasher_16384bytes, 16384
);

impl_streamed_bench!(
    wyhash64_streamed,
    WyHash64;
    streamed_00004bytes, 4;
    streamed_00007bytes, 7;
    streamed_00008bytes, 8;
    streamed_00012bytes, 12;
    streamed_00015bytes, 15;
    streamed_00016bytes, 16;
    streamed_00023bytes, 23;
    streamed_00024bytes, 24;
    streamed_00031bytes, 31;
    streamed_00032bytes, 32;
    streamed_00064bytes, 64;
    streamed_00256bytes, 256;
    streamed_01024bytes, 1024;
    streamed_04096bytes, 4096;
    streamed_16384bytes, 16384
);

impl_oneshot_bench!(
    wyhash32_oneshot,
    WyHash32;
    oneshot_00004bytes, 4;
    oneshot_00007bytes, 7;
    oneshot_00008bytes, 8;
    oneshot_00012bytes, 12;
    oneshot_00015bytes, 15;
    oneshot_00016bytes, 16;
    oneshot_00023bytes, 23;
    oneshot_00024bytes, 24;
    oneshot_00031bytes, 31;
    oneshot_00032bytes, 32;
    oneshot_00064bytes, 64;
    oneshot_00256bytes, 256;
    oneshot_01024bytes, 1024;
    oneshot_04096bytes, 4096;
    oneshot_16384bytes, 16384
);

impl_hasher_bench!(
    wyhash32_hasher,
    WyHash32;
    hasher_00004bytes, 4;
    hasher_00007bytes, 7;
    hasher_00008bytes, 8;
    hasher_00012bytes, 12;
    hasher_00015bytes, 15;
    hasher_00016bytes, 16;
    hasher_00023bytes, 23;
    hasher_00024bytes, 24;
    hasher_00031bytes, 31;
    hasher_00032bytes, 32;
    hasher_00064bytes, 64;
    hasher_00256bytes, 256;
    hasher_01024bytes, 1024;
    hasher_04096bytes, 4096;
    hasher_16384bytes, 16384
);

impl_streamed_bench!(
    wyhash32_streamed,
    WyHash32;
    streamed_00004bytes, 4;
    streamed_00007bytes, 7;
    streamed_00008bytes, 8;
    streamed_00012bytes, 12;
    streamed_00015bytes, 15;
    streamed_00016bytes, 16;
    streamed_00023bytes, 23;
    streamed_00024bytes, 24;
    streamed_00031bytes, 31;
    streamed_00032bytes, 32;
    streamed_00064bytes, 64;
    streamed_00256bytes, 256;
    streamed_01024bytes, 1024;
    streamed_04096bytes, 4096;
    streamed_16384bytes, 16384
);

impl_oneshot_bench!(
    wyhash64condom_oneshot,
    WyHash64Condom;
    oneshot_00004bytes, 4;
    oneshot_00007bytes, 7;
    oneshot_00008bytes, 8;
    oneshot_00012bytes, 12;
    oneshot_00015bytes, 15;
    oneshot_00016bytes, 16;
    oneshot_00023bytes, 23;
    oneshot_00024bytes, 24;
    oneshot_00031bytes, 31;
    oneshot_00032bytes, 32;
    oneshot_00064bytes, 64;
    oneshot_00256bytes, 256;
    oneshot_01024bytes, 1024;
    oneshot_04096bytes, 4096;
    oneshot_16384bytes, 16384
);

impl_hasher_bench!(
    wyhash64condom_hasher,
    WyHash64Condom;
    hasher_00004bytes, 4;
    hasher_00007bytes, 7;
    hasher_00008bytes, 8;
    hasher_00012bytes, 12;
    hasher_00015bytes, 15;
    hasher_00016bytes, 16;
    hasher_00023bytes, 23;
    hasher_00024bytes, 24;
    hasher_00031bytes, 31;
    hasher_00032bytes, 32;
    hasher_00064bytes, 64;
    hasher_00256bytes, 256;
    hasher_01024bytes, 1024;
    hasher_04096bytes, 4096;
    hasher_16384bytes, 16384
);

impl_streamed_bench!(
    wyhash64condom_streamed,
    WyHash64Condom;
    streamed_00004bytes, 4;
    streamed_00007bytes, 7;
    streamed_00008bytes, 8;
    streamed_00012bytes, 12;
    streamed_00015bytes, 15;
    streamed_00016bytes, 16;
    streamed_00023bytes, 23;
    streamed_00024bytes, 24;
    streamed_00031bytes, 31;
    streamed_00032bytes, 32;
    streamed_00064bytes, 64;
    streamed_00256bytes, 256;
    streamed_01024bytes, 1024;
    streamed_04096bytes, 4096;
    streamed_16384bytes, 16384
);

impl_oneshot_bench!(
    wyhash32condom_oneshot,
    WyHash32Condom;
    oneshot_00004bytes, 4;
    oneshot_00007bytes, 7;
    oneshot_00008bytes, 8;
    oneshot_00012bytes, 12;
    oneshot_00015bytes, 15;
    oneshot_00016bytes, 16;
    oneshot_00023bytes, 23;
    oneshot_00024bytes, 24;
    oneshot_00031bytes, 31;
    oneshot_00032bytes, 32;
    oneshot_00064bytes, 64;
    oneshot_00256bytes, 256;
    oneshot_01024bytes, 1024;
    oneshot_04096bytes, 4096;
    oneshot_16384bytes, 16384
);

impl_hasher_bench!(
    wyhash32condom_hasher,
    WyHash32Condom;
    hasher_00004bytes, 4;
    hasher_00007bytes, 7;
    hasher_00008bytes, 8;
    hasher_00012bytes, 12;
    hasher_00015bytes, 15;
    hasher_00016bytes, 16;
    hasher_00023bytes, 23;
    hasher_00024bytes, 24;
    hasher_00031bytes, 31;
    hasher_00032bytes, 32;
    hasher_00064bytes, 64;
    hasher_00256bytes, 256;
    hasher_01024bytes, 1024;
    hasher_04096bytes, 4096;
    hasher_16384bytes, 16384
);

impl_streamed_bench!(
    wyhash32condom_streamed,
    WyHash32Condom;
    streamed_00004bytes, 4;
    streamed_00007bytes, 7;
    streamed_00008bytes, 8;
    streamed_00012bytes, 12;
    streamed_00015bytes, 15;
    streamed_00016bytes, 16;
    streamed_00023bytes, 23;
    streamed_00024bytes, 24;
    streamed_00031bytes, 31;
    streamed_00032bytes, 32;
    streamed_00064bytes, 64;
    streamed_00256bytes, 256;
    streamed_01024bytes, 1024;
    streamed_04096bytes, 4096;
    streamed_16384bytes, 16384
);

criterion_main!(
    wyhash64_oneshot::benches,
    wyhash64_hasher::benches,
    wyhash64_streamed::benches,
    wyhash64condom_oneshot::benches,
    wyhash64condom_hasher::benches,
    wyhash64condom_streamed::benches,
    wyhash32_oneshot::benches,
    wyhash32_hasher::benches,
    wyhash32_streamed::benches,
    wyhash32condom_oneshot::benches,
    wyhash32condom_hasher::benches,
    wyhash32condom_streamed::benches,
);
