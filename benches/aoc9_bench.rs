use criterion::measurement::WallTime;
use std::time::Duration;
use aoc::aoc9::AOC9;
use aoc::runner::Runner;
use criterion::{criterion_group, criterion_main, Criterion};

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(9);

    c.bench_function("aoc9-parse", |a| {
        a.iter(|| {
            let mut solver = AOC9::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc9-1", |a| {
        let mut solver = AOC9::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc9-2", |a| {
        let mut solver = AOC9::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

fn setup() -> Criterion<WallTime> {
    Criterion::default().measurement_time(Duration::from_secs_f32(10.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
