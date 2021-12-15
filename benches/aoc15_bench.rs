use criterion::measurement::WallTime;
use std::time::Duration;
use aoc::aoc15::{AOC15, Point, generate_flowfield, find_path, find_dirs};
use aoc::runner::Runner;
use criterion::{criterion_group, criterion_main, Criterion};

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
        a.iter(|| {
            solver.run_p1();
        });
    });

    c.bench_function("aoc15-2", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        a.iter(|| {
            solver.run_p2();
        });
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

        a.iter(|| {
            let _flowfield = generate_flowfield(&map, end);
        });
    });

    c.bench_function("aoc15-2-gen-dirs", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        let mut map = solver.map.clone();
        map.grow();
        let end = Point(map.w as isize - 1, map.h as isize - 1);
        let flowfield = generate_flowfield(&map, end);
        
        a.iter(|| {
            let _dirs = find_dirs(&flowfield);
        });
    });

    c.bench_function("aoc15-2-find-path", |a| {
        let mut solver = AOC15::default();
        solver.parse(&data);
        let mut map = solver.map.clone();
        map.grow();
        let end = Point(map.w as isize - 1, map.h as isize - 1);
        let flowfield = generate_flowfield(&map, end);
        let dirs = find_dirs(&flowfield);

        a.iter(|| {
            let (_path, _cost) = find_path(&flowfield, &map, &dirs, Point(0, 0), end);
        });
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
