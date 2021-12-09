use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::aoc8::AOC8;
use aoc::runner::Runner;

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(8);

    c.bench_function("aoc8-parse", |a| {
        a.iter(|| {
            let mut solver = AOC8::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc8-1", |a| {
        let mut solver = AOC8::default();
        solver.parse(&data);

        a.iter(||{
            solver.run_p1();
        });
    });

    c.bench_function("aoc8-2", |a| {
        let mut solver = AOC8::default();
        solver.parse(&data);
        
        a.iter(|| {
            solver.run_p2();
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);