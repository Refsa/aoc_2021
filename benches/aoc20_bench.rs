use aoc::aoc20::AOC20;
use aoc::runner::Runner;
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(20);

    c.bench_function("aoc20-parse", |a| {
        a.iter(|| {
            let mut solver = AOC20::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc20-1", |a| {
        let mut solver = AOC20::default();
        solver.parse(&data);
        a.iter(|| solver.run_p1());
    });

    c.bench_function("aoc20-2", |a| {
        let mut solver = AOC20::default();
        solver.parse(&data);
        a.iter(|| solver.run_p2());
    });
}

fn setup() -> Criterion<WallTime> {
    Criterion::default().measurement_time(Duration::from_secs_f32(20.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
