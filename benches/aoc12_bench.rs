use criterion::measurement::WallTime;
use std::time::Duration;
use aoc::aoc12::AOC12;
use aoc::runner::Runner;
use criterion::{criterion_group, criterion_main, Criterion};

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(12);

    c.bench_function("aoc12-parse", |a| {
        a.iter(|| {
            let mut solver = AOC12::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc12-1", |a| {
        let mut solver = AOC12::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc12-2", |a| {
        let mut solver = AOC12::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

fn setup() -> Criterion<WallTime> {
    Criterion::default().measurement_time(Duration::from_secs_f32(12.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
