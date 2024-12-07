use advent_of_codspeed_2024::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day1_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    c.bench_function("day1::part1", |b| b.iter(|| day1::part1(black_box(input))));
}
pub fn day1_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    c.bench_function("day1::part2", |b| b.iter(|| day1::part2(black_box(input))));
}

pub fn day2_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    c.bench_function("day2::part1", |b| b.iter(|| day2::part1(black_box(input))));
}
pub fn day2_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    c.bench_function("day2::part2", |b| b.iter(|| day2::part2(black_box(input))));
}

pub fn day3_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    c.bench_function("day3::part1", |b| b.iter(|| day3::part1(black_box(input))));
}
pub fn day3_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    c.bench_function("day3::part2", |b| b.iter(|| day3::part2(black_box(input))));
}

pub fn day4_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");
    c.bench_function("day4::part1", |b| b.iter(|| day4::part1(black_box(input))));
}
pub fn day4_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");
    c.bench_function("day4::part2", |b| b.iter(|| day4::part2(black_box(input))));
}

pub fn day5_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day5.txt");
    c.bench_function("day5::part1", |b| b.iter(|| day5::part1(black_box(input))));
}
pub fn day5_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day5.txt");
    c.bench_function("day5::part2", |b| b.iter(|| day5::part2(black_box(input))));
}

pub fn day6_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day6.txt");
    c.bench_function("day6::part1", |b| b.iter(|| day6::part1(black_box(input))));
}
pub fn day6_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day6.txt");
    c.bench_function("day6::part2", |b| b.iter(|| day6::part2(black_box(input))));
}

pub fn day7_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day7.txt");
    c.bench_function("day7::part1", |b| b.iter(|| day7::part1(black_box(input))));
}
pub fn day7_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day7.txt");
    c.bench_function("day7::part2", |b| b.iter(|| day7::part2(black_box(input))));
}

criterion_group!(
    benches, day1_part1, day1_part2, day2_part1, day2_part2, day3_part1, day3_part2, day4_part1,
    day4_part2, day5_part1, day5_part2, day6_part1, day6_part2, day7_part1, day7_part2
);
criterion_main!(benches);
