use std::time::{Duration};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecrs::aco::FMatrix;
use itertools::Itertools;
use rand::{Error, Rng, RngCore};
use aco_bbp::{BinAnt, BinSharedState, MyAnt, PP};

fn bench_bin_ant<R: Rng + Clone>(mut ant: BinAnt<R>, ss: &BinSharedState, pher: &mut FMatrix) {
    for _ in 0..100 {
        ant.build_solution(pher ,ss);
    }

}

fn bench_bin_ant2d<R: Rng + Clone>(mut ant: BinAnt<R>, ss: &BinSharedState, pher: &mut Vec<FMatrix>) {
    for _ in 0..100 {
        ant.build_solution(pher ,ss);
    }

}

#[derive(Clone)]
struct FakeRng;


impl RngCore for FakeRng {
    fn next_u32(&mut self) -> u32 {
        1
    }

    fn next_u64(&mut self) -> u64 {
        1
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill(1)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(dest.fill(1))
    }
}



fn criterion_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1)
        .sample_size(100)
        .measurement_time(Duration::from_secs(30))
        .warm_up_time(Duration::from_secs(5));
    let ant  = BinAnt::rng_and_pp(PP::IterationExpRand, FakeRng);
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


     group.bench_function("bin ant", |b| b.iter(|| bench_bin_ant(black_box(ant.clone()), &ss, &mut start_pheromone)));


    let ant = BinAnt::rng_and_pp(PP::IterationExpRand, FakeRng);
    let mut start_pheromone = (0..5).map(|_| FMatrix::repeat(size_count, size_count, 1.0)).collect_vec();
    group.bench_function("bin ant 2d", |b| b.iter(|| bench_bin_ant2d(black_box(ant.clone()), &ss, &mut start_pheromone)));
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);