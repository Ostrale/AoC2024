use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_09::{Disk, compact_disk, compact_disk_v2};

fn benchmark_compact_disk(c: &mut Criterion) {
    let input = "2333133121414131402";
    let disk = Disk::from_string(input);

    c.bench_function("compact_disk", |b| b.iter(|| {
        let mut disk_clone = disk.clone();
        compact_disk(&mut disk_clone);
        black_box(disk_clone.checksum());
    }));
}

fn benchmark_compact_disk_v2(c: &mut Criterion) {
    let input = "2333133121414131402";
    let disk = Disk::from_string(input);

    c.bench_function("compact_disk_v2", |b| b.iter(|| {
        let mut disk_clone = disk.clone();
        compact_disk_v2(&mut disk_clone);
        black_box(disk_clone.checksum());
    }));
}

criterion_group!(benches, benchmark_compact_disk, benchmark_compact_disk_v2);
criterion_main!(benches);