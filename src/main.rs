use std::time::Instant;
use ecrs::aco;
use ecrs::aco::FMatrix;
use ecrs::aco::pheromone::{AntSystemPU};
use ecrs::aco::termination_condition::IterationCond;
use itertools::Itertools;
use crate::colony::{BinColony, BinSharedState};
use crate::colony::ant::BinAnt;
use crate::problem::ProblemLoader;

mod probe;
mod fitness;
mod util;
mod colony;
mod problem;

const ANTS: usize = 50;


fn main() {
    let problem = ProblemLoader::new()
      .problem_size(1000)
      .load_problem(2);


    let (size_count, size_to_index, index_to_size) = util::process_items(&problem.items);
    let mut i2count = vec![0; size_count];
    for i in problem.items.clone() {
        i2count[size_to_index[&i]] += 1;
    }

    let ants = (0..ANTS).map(|_| BinAnt::new()).collect_vec();
    let ss = BinSharedState {
        alpha: 1.0,
        beta: 2.0,
        i2size: index_to_size.clone(),
        solution_size: problem.items.len(),
        bin_cap: problem.bin_cap,
        i2count
    };

    let colony = BinColony::new(ss.clone(), ants);

    let start_pheromone = FMatrix::repeat(size_count, size_count, 1.0);

    let fitness = fitness::BinFitness{
        stress_factor: 2.0,
        i2size: index_to_size.clone(),
        bin_cap: problem.bin_cap,
    };

    let probe = probe::CsvProbe::new(index_to_size.clone(), "as" , problem.bin_cap);

    let algo = aco::Builder::new(problem.items.len())
      .set_colony(colony)
      .set_start_pheromone(start_pheromone)
      .set_pheromone_update(AntSystemPU)
      .set_fitness(fitness)
      .set_probe(probe)
      .set_termination_condition(IterationCond::new(200))
      .build();

    let start = Instant::now();
    algo.run();
    println!("Time: {}", start.elapsed().as_millis());
}


