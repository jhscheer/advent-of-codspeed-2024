use advent_of_codspeed_2024::day1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let path = std::path::PathBuf::from("input/2024/day1.txt");
    let f = std::fs::File::open(path).unwrap();
    let input = std::io::read_to_string(f).unwrap();

    c.bench_function("day1::part1", |b| {
        b.iter(|| day1::part1(black_box(&day1::input_generator(&input))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
