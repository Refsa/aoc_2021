use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::aoc1::AOC1;
use aoc::runner::Runner;

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(1);

    c.bench_function("aoc1-1", |a| {
        a.iter(||{
            let mut solver = AOC1::default();
            solver.parse(&data);
            solver.run_p1();
        });
    });

    c.bench_function("aoc1-2", |a| {
        a.iter(|| {
            let mut solver = AOC1::default();
            solver.parse(&data);
            solver.run_p2();
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);