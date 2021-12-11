use criterion::measurement::WallTime;
use std::time::Duration;
use aoc::aoc11::AOC11;
use aoc::runner::Runner;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(11);

    c.bench_function("aoc11-parse", |a| {
        a.iter(|| {
            let mut solver = AOC11::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc11-1", |a| {
        let mut solver = AOC11::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc11-2", |a| {
        let mut solver = AOC11::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

fn setup() -> Criterion<WallTime> {
    Criterion::default().measurement_time(Duration::from_secs_f32(11.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
