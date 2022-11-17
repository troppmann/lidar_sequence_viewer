use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lidar_viewer::io::read_frame;


fn bench_parser(c: &mut Criterion) {
    //prepare
    let path = Path::new("../SemanticKITTI/dataset/sequences/00/velodyne/000000.bin");
    //bench
    //let mut group = c.benchmark_group("Parser");
    c.bench_function("byteorder parser 000000bin", |b| b.iter(|| read_frame(black_box(path))));
    //group.finish()
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);