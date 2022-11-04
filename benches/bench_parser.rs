use std::{fs::File, io::Read};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lidar_viewer::{byteorder_parser, Point};


fn nope_parser(_: &[u8])->Vec<Point>{
    Vec::new()
}

fn bench_parser(c: &mut Criterion) {
    //prepare
    let path = "../SemanticKITTI/dataset/sequences/00/velodyne/000000.bin";
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    //bench
    let mut group = c.benchmark_group("Parser");
    group.bench_function("byteorder parser 000000bin", |b| b.iter(|| byteorder_parser(black_box(&buffer))));
    group.bench_function("nom parser 000000bin", |b| b.iter(|| nope_parser(black_box(&buffer))));
    group.finish()
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);