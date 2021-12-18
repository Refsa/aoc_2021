use aoc::aoc15::{find_path, generate_risks, Point, AOC15, generate_flowfield};
use aoc::runner::Runner;
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

mod get_input;

fn bench(c: &mut Criterion) {
    let data = get_input::get_input(15);

    c.bench_function("aoc15-parse", |a| {
        a.iter(|| {
            let mut solver = AOC15::default();
            solver.parse(&data);
        });
    });

    c.bench_function("aoc15-1", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        a.iter(|| solver.run_p1());
    });

    c.bench_function("aoc15-2", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        a.iter(|| solver.run_p2());
    });

    c.bench_function("aoc15-2-grow-map", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        a.iter(|| {
            solver.map.clone().grow();
        });
    });

    c.bench_function("aoc15-2-gen-flowfield", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        let mut map = solver.map.clone();
        map.grow();
        let end = Point(map.w as isize - 1, map.h as isize - 1);

        a.iter(|| generate_risks(&map, end));
    });

    c.bench_function("aoc15-2-gen-dirs", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        let mut map = solver.map.clone();
        map.grow();
        let end = Point(map.w as isize - 1, map.h as isize - 1);
        let flowfield = generate_risks(&map, end).unwrap();

        a.iter(|| generate_flowfield(&flowfield));
    });

    c.bench_function("aoc15-2-find-path", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        let mut map = solver.map.clone();
        map.grow();
        let end = Point(map.w as isize - 1, map.h as isize - 1);
        let flowfield = generate_risks(&map, end).unwrap();
        let dirs = generate_flowfield(&flowfield);

        a.iter(|| find_path(&flowfield, &map, &dirs, Point(0, 0), end));
    });
}

fn setup() -> Criterion<WallTime> {
    Criterion::default().measurement_time(Duration::from_secs_f32(15.0))
}

criterion_group! {
    name = benches;
    config = setup();
    targets = bench
}
criterion_main!(benches);
