use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecrs::aco::FMatrix;
use itertools::Itertools;
use aco_bbp::{BinAnt, BinSharedState, MyAnt};

fn bench_bin_ant(mut ant: BinAnt, ss: &BinSharedState, pher: &mut FMatrix) {
    for _ in 0..100 {
        ant.build_solution(pher ,ss);
    }

}



fn criterion_benchmark(c: &mut Criterion) {
    let ant  = BinAnt::new();
    let problem = aco_bbp::problem::ProblemLoader::new()
        .pick_uniform(true)
        .problem_size(1000)
        .load_problem(5);


    let (size_count, size_to_index, index_to_size) = aco_bbp::util::process_items(&problem.items);
    let mut i2count = vec![0; size_count];
    for i in problem.items.clone() {
        i2count[size_to_index[&i]] += 1;
    }

    let ss = BinSharedState {
        alpha:4.5,
        beta:3.5,
        i2size: index_to_size.clone(),
        solution_size: problem.items.len(),
        bin_cap: problem.bin_cap,
        i2count: i2count.clone(),
        heuristic: index_to_size.iter().map(|x| (*x as f64).powf(3.5)).collect_vec()
    };

    let mut start_pheromone = FMatrix::repeat(size_count, size_count, 1.0);


     c.bench_function("bin ant", |b| b.iter(|| bench_bin_ant(black_box(ant.clone()), &ss, &mut start_pheromone)));
    // c.bench_function("pow int", |b| b.iter(|| pow_unsafe(black_box(10000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);