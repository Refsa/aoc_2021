use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::aoc6::AOC6;
use aoc::runner::Runner;

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(6);

    c.bench_function("aoc6-1", |a| {
        a.iter(||{
            let mut solver = AOC6::default();
            solver.parse(&data);
            solver.run_p1();
        });
    });

    c.bench_function("aoc6-2", |a| {
        a.iter(|| {
            let mut solver = AOC6::default();
            solver.parse(&data);
            solver.run_p2();
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);