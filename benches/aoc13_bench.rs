use criterion::measurement::WallTime;
use std::time::Duration;
use aoc::aoc13::AOC13;
use aoc::runner::Runner;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(13);

    c.bench_function("aoc13-parse", |a| {
        a.iter(|| {
            let mut solver = AOC13::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc13-1", |a| {
        let mut solver = AOC13::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc13-2", |a| {
        let mut solver = AOC13::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

fn setup() -> Criterion<WallTime> {
    Criterion::default().measurement_time(Duration::from_secs_f32(13.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
