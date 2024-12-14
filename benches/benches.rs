use advent_of_codspeed_2024::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    c.bench_function("day01::part1", |b| {
        b.iter(|| day01::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day1.txt");
    c.bench_function("day01::part2", |b| {
        b.iter(|| day01::part2(black_box(input)))
    });
}

pub fn day02(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    c.bench_function("day02::part1", |b| {
        b.iter(|| day02::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day2.txt");
    c.bench_function("day02::part2", |b| {
        b.iter(|| day02::part2(black_box(input)))
    });
}

pub fn day03(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    c.bench_function("day03::part1", |b| {
        b.iter(|| day03::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day3.txt");
    c.bench_function("day03::part2", |b| {
        b.iter(|| day03::part2(black_box(input)))
    });
}

pub fn day04(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");
    c.bench_function("day04::part1", |b| {
        b.iter(|| day04::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day4.txt");
    c.bench_function("day04::part2", |b| {
        b.iter(|| day04::part2(black_box(input)))
    });
}

pub fn day05(c: &mut Criterion) {
    let input = include_str!("../input/2024/day5.txt");
    c.bench_function("day05::part1", |b| {
        b.iter(|| day05::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day5.txt");
    c.bench_function("day05::part2", |b| {
        b.iter(|| day05::part2(black_box(input)))
    });
}

pub fn day06(c: &mut Criterion) {
    let input = include_str!("../input/2024/day6.txt");
    c.bench_function("day06::part1", |b| {
        b.iter(|| day06::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day6.txt");
    c.bench_function("day06::part2", |b| {
        b.iter(|| day06::part2(black_box(input)))
    });
}

pub fn day07(c: &mut Criterion) {
    let input = include_str!("../input/2024/day7.txt");
    c.bench_function("day07::part1", |b| {
        b.iter(|| day07::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day7.txt");
    c.bench_function("day07::part2", |b| {
        b.iter(|| day07::part2(black_box(input)))
    });
}

pub fn day08(c: &mut Criterion) {
    let input = include_str!("../input/2024/day8.txt");
    c.bench_function("day08::part1", |b| {
        b.iter(|| day08::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day8.txt");
    c.bench_function("day08::part2", |b| {
        b.iter(|| day08::part2(black_box(input)))
    });
}

pub fn day09(c: &mut Criterion) {
    let input = include_str!("../input/2024/day9.txt");
    c.bench_function("day09::part1", |b| {
        b.iter(|| day09::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day9.txt");
    c.bench_function("day09::part2", |b| {
        b.iter(|| day09::part2(black_box(input)))
    });
}

pub fn day10(c: &mut Criterion) {
    let input = include_str!("../input/2024/day10.txt");
    c.bench_function("day10::part1", |b| {
        b.iter(|| day10::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day10.txt");
    c.bench_function("day10::part2", |b| {
        b.iter(|| day10::part2(black_box(input)))
    });
}

pub fn day11(c: &mut Criterion) {
    let input = include_str!("../input/2024/day11.txt");
    c.bench_function("day11::part1", |b| {
        b.iter(|| day11::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day11.txt");
    c.bench_function("day11::part2", |b| {
        b.iter(|| day11::part2(black_box(input)))
    });
}

pub fn day13(c: &mut Criterion) {
    let input = include_str!("../input/2024/day13.txt");
    c.bench_function("day13::part1", |b| {
        b.iter(|| day13::part1(black_box(input)))
    });
    let input = include_str!("../input/2024/day13.txt");
    c.bench_function("day13::part2", |b| {
        b.iter(|| day13::part2(black_box(input)))
    });
}

criterion_group!(
    benches, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day13,
);
criterion_main!(benches);
