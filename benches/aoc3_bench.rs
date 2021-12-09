use aoc::aoc3::AOC3;
use aoc::runner::Runner;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(3);

    c.bench_function("aoc3-parse", |a| {
        a.iter(|| {
            let mut solver = AOC3::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc3-1", |a| {
        let mut solver = AOC3::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc3-2", |a| {
        let mut solver = AOC3::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

fn setup() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::from_secs_f32(10.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
