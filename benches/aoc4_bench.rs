use aoc::aoc4::AOC4;
use aoc::runner::Runner;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(4);

    c.bench_function("aoc4-parse", |a| {
        a.iter(|| {
            let mut solver = AOC4::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc4-1", |a| {
        let mut solver = AOC4::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc4-2", |a| {
        let mut solver = AOC4::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
