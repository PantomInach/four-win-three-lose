use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};
use four_win_three_lose::field::Field;

fn branch_all_possible_moves(c: &mut Criterion) {
    let t = Some(true);
    let f = Some(false);
    let n: Option<bool> = None;

    c.bench_function("All moves: upper half filled", |b| {
        b.iter(|| {
            let mut field = Field::from([t, t, f, f, f, f, t, t, n, n, n, n, n, n, n, n]);
            field.brute_force_game_state(false, false, &Field::possible_moves);
        })
    });

    let mut slow_group = c.benchmark_group("slow_group");
    slow_group.sample_size(10);
    slow_group.bench_function("All moves: first row filled", |b| {
        b.iter(|| {
            let mut field = Field::from([t, t, f, f, n, n, n, n, n, n, n, n, n, n, n, n]);
            field.brute_force_game_state(false, false, &Field::possible_moves);
        })
    });
}

fn branch_unique_x_y_mirrored(c: &mut Criterion) {
    let t = Some(true);
    let f = Some(false);
    let n: Option<bool> = None;

    c.bench_function("Neglect x,y mirrored: upper half filled", |b| {
        b.iter(|| {
            let mut field = Field::from([t, t, f, f, f, f, t, t, n, n, n, n, n, n, n, n]);
            field.brute_force_game_state(false, false, &Field::possible_non_symmetrical_moves);
        })
    });

    let mut slow_group = c.benchmark_group("slow_group");
    slow_group.sample_size(10);
    slow_group.bench_function("Neglect x,y mirrored: first row filled", |b| {
        b.iter(|| {
            let mut field = Field::from([t, t, f, f, n, n, n, n, n, n, n, n, n, n, n, n]);
            field.brute_force_game_state(false, false, &Field::possible_non_symmetrical_moves);
        })
    });
}

fn branch_symmetric_if_sparse(c: &mut Criterion) {
    let t = Some(true);
    let f = Some(false);
    let n: Option<bool> = None;

    c.bench_function("Mix: Upper half filled", |b| {
        b.iter(|| {
            let mut field = Field::from([t, t, f, f, f, f, t, t, n, n, n, n, n, n, n, n]);
            field.brute_force_game_state(false, false, &Field::possible_non_symmetrical_moves);
        })
    });

    let mut slow_group = c.benchmark_group("slow_group");
    slow_group.sample_size(10);
    slow_group.bench_function("Mix: first row filled", |b| {
        b.iter(|| {
            let mut field = Field::from([t, t, f, f, n, n, n, n, n, n, n, n, n, n, n, n]);
            field.brute_force_game_state(false, false, &Field::possible_non_symmetrical_moves);
        })
    });
}

criterion_group!(
    brute_force,
    branch_all_possible_moves,
    branch_unique_x_y_mirrored,
    branch_symmetric_if_sparse
);
criterion_main!(brute_force);
