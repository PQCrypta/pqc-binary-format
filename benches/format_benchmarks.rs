//! Benchmarks for PQC Binary Format serialization and deserialization

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pqc_binary_format::{Algorithm, EncParameters, PqcBinaryFormat, PqcMetadata};
use std::collections::HashMap;

fn create_test_format(data_size: usize) -> PqcBinaryFormat {
    let metadata = PqcMetadata {
        enc_params: EncParameters {
            iv: vec![1; 12],
            tag: vec![1; 16],
            params: HashMap::new(),
        },
        ..Default::default()
    };

    let data = vec![0u8; data_size];
    PqcBinaryFormat::new(Algorithm::Hybrid, metadata, data)
}

fn bench_serialization(c: &mut Criterion) {
    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024];

    for size in sizes {
        let format = create_test_format(size);

        let mut group = c.benchmark_group(format!("serialize_{}_bytes", size));
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function("to_bytes", |b| {
            b.iter(|| {
                black_box(format.to_bytes().unwrap());
            });
        });

        group.finish();
    }
}

fn bench_deserialization(c: &mut Criterion) {
    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024];

    for size in sizes {
        let format = create_test_format(size);
        let bytes = format.to_bytes().unwrap();

        let mut group = c.benchmark_group(format!("deserialize_{}_bytes", size));
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function("from_bytes", |b| {
            b.iter(|| {
                black_box(PqcBinaryFormat::from_bytes(&bytes).unwrap());
            });
        });

        group.finish();
    }
}

fn bench_roundtrip(c: &mut Criterion) {
    let format = create_test_format(10 * 1024);

    c.bench_function("roundtrip_10kb", |b| {
        b.iter(|| {
            let bytes = black_box(format.to_bytes().unwrap());
            let _recovered = black_box(PqcBinaryFormat::from_bytes(&bytes).unwrap());
        });
    });
}

criterion_group!(
    benches,
    bench_serialization,
    bench_deserialization,
    bench_roundtrip
);
criterion_main!(benches);
