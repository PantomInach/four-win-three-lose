use criterion::{criterion_group, criterion_main, Criterion};
use four_win_three_lose::field::Field;

fn scenarios() -> impl Iterator<Item = Field> {
    let t = Some(true);
    let f = Some(false);
    let n: Option<bool> = None;
    let fields = vec![
        Field::from([f, n, n, t, t, n, n, t, f, n, n, f, f, n, n, t]),
        Field::from([f, n, n, t, n, f, t, n, n, f, t, n, t, n, n, f]),
        Field::from([f, n, n, t, t, t, n, n, n, n, n, n, f, f, t, f]),
        Field::from([f, n, n, t, t, t, n, f, n, n, n, n, f, t, n, f]),
        Field::from([t, f, n, f, n, f, n, t, n, n, n, n, f, t, n, t]),
    ];
    fields.into_iter().cycle()
}

fn branch_all_possible_moves(c: &mut Criterion) {
    let t = Some(true);
    let f = Some(false);
    let n: Option<bool> = None;

    c.bench_function("All moves: upper half filled", |b| {
        let field = Field::from([t, t, f, f, f, f, t, t, n, n, n, n, n, n, n, n]);
        b.iter(|| {
            let mut f = field.clone();
            f.brute_force_game_state(false, false, &Field::possible_moves);
        })
    });

    c.bench_function("All moves: different starting positions", |b| {
        let mut fields = scenarios();
        b.iter(|| {
            let mut f = fields.next().unwrap();
            f.brute_force_game_state(false, false, &Field::possible_moves);
        })
    });

    let mut slow_group = c.benchmark_group("slow_group");
    slow_group.sample_size(10);
    slow_group.bench_function("All moves: first row filled", |b| {
        let field = Field::from([t, t, f, f, n, n, n, n, n, n, n, n, n, n, n, n]);
        b.iter(|| {
            let mut f = field.clone();
            f.brute_force_game_state(false, false, &Field::possible_moves);
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

    c.bench_function("Neglect x,y mirrored: different starting positions", |b| {
        let mut fields = scenarios();
        b.iter(|| {
            let mut f = fields.next().unwrap();
            f.brute_force_game_state(false, false, &Field::possible_non_symmetrical_moves);
        })
    });

    let mut slow_group = c.benchmark_group("slow_group");
    slow_group.sample_size(10);
    slow_group.bench_function("Neglect x,y mirrored: first row filled", |b| {
        let field = Field::from([t, t, f, f, n, n, n, n, n, n, n, n, n, n, n, n]);
        b.iter(|| {
            let mut f = field.clone();
            f.brute_force_game_state(false, false, &Field::possible_non_symmetrical_moves);
        })
    });
}

fn branch_symmetric_if_sparse(c: &mut Criterion) {
    let t = Some(true);
    let f = Some(false);
    let n: Option<bool> = None;

    c.bench_function("Mix: Upper half filled", |b| {
        let field = Field::from([t, t, f, f, f, f, t, t, n, n, n, n, n, n, n, n]);
        b.iter(|| {
            let mut f = field.clone();
            f.brute_force_game_state(false, false, &Field::possible_moves_symmetrical_if_sparse);
        })
    });

    c.bench_function("Mix: different starting positions", |b| {
        let mut fields = scenarios();
        b.iter(|| {
            let mut f = fields.next().unwrap();
            f.brute_force_game_state(false, false, &Field::possible_moves_symmetrical_if_sparse);
        })
    });

    // let mut slow_group = c.benchmark_group("slow_group");
    // slow_group.sample_size(10);
    // slow_group.bench_function("Mix: first row filled", |b| {
    //     let field = Field::from([t, t, f, f, n, n, n, n, n, n, n, n, n, n, n, n]);
    //     b.iter(|| {
    //         let mut f = field.clone();
    //         f.brute_force_game_state(false, false, &Field::possible_moves_symmetrical_if_sparse);
    //     })
    // });
}

criterion_group!(
    brute_force,
    branch_all_possible_moves,
    branch_unique_x_y_mirrored,
    branch_symmetric_if_sparse
);
criterion_main!(brute_force);
