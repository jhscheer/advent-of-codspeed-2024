use advent_of_codspeed_2024::day2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");

    c.bench_function("day2::part1", |b| b.iter(|| day2::part1(black_box(input))));
}

pub fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");

    c.bench_function("day2::part2", |b| b.iter(|| day2::part2(black_box(input))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
