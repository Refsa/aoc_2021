use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::aoc7::AOC7;
use aoc::runner::Runner;

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(7);

    c.bench_function("aoc7-parse", |a| {
        a.iter(|| {
            let mut solver = AOC7::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc7-1", |a| {
        let mut solver = AOC7::default();
        solver.parse(&data);
        a.iter(||{
            solver.run_p1();
        });
    });

    c.bench_function("aoc7-2", |a| {
        let mut solver = AOC7::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);