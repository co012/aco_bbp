use ecrs::aco;
use ecrs::aco::FMatrix;
use ecrs::aco::pheromone::{AntColonySystemPU, AntSystemPU, PartFromEvalPU, PheromoneUpdate};
use ecrs::aco::pheromone::best_policy::IterationBest;
use ecrs::aco::termination_condition::{IterationCond};
use itertools::Itertools;
use rand::{Rng, thread_rng};
use rayon::prelude::*;

use crate::colony::{BinColony, BinSharedState};
use crate::colony::ant::{BinAnt, BinAnt2D, DnaAnt, PerceivedPherStrat};
use crate::fitness::BinFitness;
use crate::probe::CsvProbe;
use crate::problem::{Problem, ProblemLoader};

mod probe;
mod fitness;
mod util;
mod colony;
mod problem;

const ANTS: usize = 50;
const ITERS: usize = 1000;
const PHER_LEVELS: usize = 5;

fn check_parms() {
  for p_num in 0..20 {
    let problem = ProblemLoader::new()
      .pick_uniform(true)
      .problem_size(500)
      .load_problem(p_num);


    let (size_count, size_to_index, index_to_size) = util::process_items(&problem.items);
    let mut i2count = vec![0; size_count];
    for i in problem.items.clone() {
      i2count[size_to_index[&i]] += 1;

    }
    (0..20).into_par_iter().for_each(|i| {
      let fitness = BinFitness {
        stress_factor: 2.0,
        i2size: index_to_size.clone(),
        bin_cap: problem.bin_cap,
      };
      let mut probe = CsvProbe::new(index_to_size.clone(), String::new(), problem.bin_cap);
      probe.file_post = format!("{}", i);


      let ss = BinSharedState {
        alpha: 2.5,
        beta: 2.5,
        i2size: index_to_size.clone(),
        solution_size: problem.items.len(),
        bin_cap: problem.bin_cap,
        i2count: i2count.clone(),
      };

      // alpha = 2.5 beta = 2.5
      run_as(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("as", &ss, p_num)));
      run_as_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("as2d_ei", &ss, p_num)), ss.clone(), PerceivedPherStrat::EveryItem);
      run_acs_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("acs2d_ei", &ss, p_num)), ss.clone(), PerceivedPherStrat::EveryItem);
      run_acs_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("acs2d_io", &ss, p_num)), ss.clone(), PerceivedPherStrat::IterOnce);


      let ss = BinSharedState {
        alpha: 2.0,
        beta: 2.5,
        i2size: index_to_size.clone(),
        solution_size: problem.items.len(),
        bin_cap: problem.bin_cap,
        i2count: i2count.clone(),
      };
      // alpha = 2.0 beta = 2.5
      run_dna(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("dna", &ss, p_num)));
      run_dna_rand(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("dna_rand", &ss, p_num)));
      run_as(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("as", &ss, p_num)));
      run_acs_pu(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("acs", &ss, p_num)));
      run_acs_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("acs2d_ei", &ss, p_num)), ss.clone(), PerceivedPherStrat::EveryItem);
      run_as_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("as2d_io", &ss, p_num)), ss.clone(), PerceivedPherStrat::IterOnce);
      run_as_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("as2d_ei", &ss, p_num)), ss.clone(), PerceivedPherStrat::EveryItem);


      let ss = BinSharedState {
        alpha: 2.0,
        beta: 2.0,
        i2size: index_to_size.clone(),
        solution_size: problem.items.len(),
        bin_cap: problem.bin_cap,
        i2count: i2count.clone(),
      };
      // alpha = 2.0 beta = 2.0
      run_acs_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("acs2d_io", &ss, p_num)), ss.clone(), PerceivedPherStrat::IterOnce);
      run_as_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("as2d_io", &ss, p_num)), ss.clone(), PerceivedPherStrat::IterOnce);
    });
  }
}


fn main() {
  let problem = ProblemLoader::new()
    .pick_uniform(true)
    .problem_size(1000)
    .load_problem(0);

  let (size_count, size_to_index, index_to_size) = util::process_items(&problem.items);
  let mut i2count = vec![0; size_count];
  for i in problem.items.clone() {
    i2count[size_to_index[&i]] += 1;

  }

  (0..10).into_par_iter().for_each(|i| {


    let fitness = BinFitness {
      stress_factor: 2.0,
      i2size: index_to_size.clone(),
      bin_cap: problem.bin_cap,
    };
    let mut probe = CsvProbe::new(index_to_size.clone(), String::new(), problem.bin_cap);
    probe.file_post = format!("{}", i);



    for alpha in [2.5, 3.0] {
      for beta in [2.5, 10.0] {
        let ss = BinSharedState {
          alpha,
          beta,
          i2size: index_to_size.clone(),
          solution_size: problem.items.len(),
          bin_cap: problem.bin_cap,
          i2count: i2count.clone(),
        };
        run_as(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("as", &ss, 0)));
        run_dna(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("dna", &ss, 0)));
        run_dna_rand(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("dna_rand", &ss, 0)));
        run_acs_pu(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("acs", &ss,0)));
        run_acs_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("acs2d_ei", &ss, 0)), ss.clone(), PerceivedPherStrat::EveryItem);
        run_as_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("as2d_io", &ss, 0)), ss.clone(), PerceivedPherStrat::IterOnce);
        run_as_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("as2d_ei", &ss,0)), ss.clone(), PerceivedPherStrat::EveryItem);
        run_acs_2d(&problem, size_count, fitness.clone(), probe.clone_exchange(make_label("acs2d_io", &ss, 0)), ss.clone(), PerceivedPherStrat::IterOnce);
        run_dna_bias(&problem, ss.clone(), size_count, &fitness, probe.clone_exchange(make_label("dna_bias", &ss, 0)));
      }
    }




  })

}

fn make_label(l: &'static str, ss: &BinSharedState, p_num: usize) -> String {
  format!("{},{},{},{},{}",p_num, l, ss.alpha, ss.beta, ANTS)
}

fn run_as_2d(problem: &Problem, size_count: usize, fitness: BinFitness, probe: CsvProbe, ss: BinSharedState, pp: PerceivedPherStrat) {
  let ants = (0..ANTS).map(|_| BinAnt2D::new(pp.clone())).collect_vec();
  let colony = BinColony::new(ss, ants);
  let start_pheromone = (0..PHER_LEVELS).map(|_| FMatrix::repeat(size_count, size_count, 1.0)).collect_vec();
  let pus = (0..PHER_LEVELS)
    .map(|_| Box::new(AntSystemPU) as Box<dyn PheromoneUpdate<FMatrix>>)
    .collect_vec();
  let pu = PartFromEvalPU::new(pus);

  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(pu)
    .set_fitness(fitness)
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();


  algo.run()
}

fn run_acs_2d(problem: &Problem, size_count: usize, fitness: BinFitness, probe: CsvProbe, ss: BinSharedState, pp: PerceivedPherStrat) {
  let ants = (0..ANTS).map(|_| BinAnt2D::new(pp.clone())).collect_vec();
  let colony = BinColony::new(ss, ants);
  let start_pheromone = (0..PHER_LEVELS).map(|_| FMatrix::repeat(size_count, size_count, 1.0)).collect_vec();
  let pus = (0..PHER_LEVELS)
    .map(|_| Box::new(AntColonySystemPU::with_policy(IterationBest::new())) as Box<dyn PheromoneUpdate<FMatrix>>)
    .collect_vec();
  let pu = PartFromEvalPU::new(pus);

  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(pu)
    .set_fitness(fitness)
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();


  algo.run()
}

fn run_as(problem: &Problem, ss: BinSharedState, size_count: usize, fitness: &BinFitness, probe: CsvProbe) {
  let ants = (0..ANTS).map(|_| BinAnt::new()).collect_vec();
  let colony = BinColony::new(ss.clone(), ants);
  let start_pheromone = FMatrix::repeat(size_count, size_count, 1.0);
  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(AntSystemPU)
    .set_fitness(fitness.clone())
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();
  algo.run()
}

fn run_dna(problem: &Problem, ss: BinSharedState, size_count: usize, fitness: &BinFitness, probe: CsvProbe) {
  let dna = (0..PHER_LEVELS).map(|i| 0.5f64.powi(i as i32)).rev().collect_vec();

  let start_pheromone = (0..PHER_LEVELS).map(|_| FMatrix::repeat(size_count, size_count, 1.0)).collect_vec();
  let ants = (0..ANTS).map(|_| DnaAnt::new(dna.clone())).collect_vec();
  let pus = (0..PHER_LEVELS)
    .map(|_| Box::new(AntSystemPU) as Box<dyn PheromoneUpdate<FMatrix>>)
    .collect_vec();
  let pu = PartFromEvalPU::new(pus);

  let colony = BinColony::new(ss.clone(), ants);
  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(pu)
    .set_fitness(fitness.clone())
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();

  algo.run()
}

fn run_dna_bias(problem: &Problem, ss: BinSharedState, size_count: usize, fitness: &BinFitness, probe: CsvProbe) {
  let dna = (0..PHER_LEVELS).map(|i| 0.5f64.powi(i as i32) - 0.125).rev().collect_vec();

  let start_pheromone = (0..PHER_LEVELS).map(|_| FMatrix::repeat(size_count, size_count, 1.0)).collect_vec();
  let ants = (0..ANTS).map(|_| DnaAnt::new(dna.clone())).collect_vec();
  let pus = (0..PHER_LEVELS)
    .map(|_| Box::new(AntSystemPU) as Box<dyn PheromoneUpdate<FMatrix>>)
    .collect_vec();
  let pu = PartFromEvalPU::new(pus);

  let colony = BinColony::new(ss.clone(), ants);
  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(pu)
    .set_fitness(fitness.clone())
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();

  algo.run()
}

fn run_dna_rand(problem: &Problem, ss: BinSharedState, size_count: usize, fitness: &BinFitness, probe: CsvProbe) {
  let mut r = thread_rng();
  let dnas = (0..ANTS).map(|_| (0..PHER_LEVELS).map(|_| r.gen::<f64>()).collect_vec());

  let start_pheromone = (0..PHER_LEVELS).map(|_| FMatrix::repeat(size_count, size_count, 1.0)).collect_vec();
  let ants = dnas.map(|dna| DnaAnt::new(dna)).collect_vec();
  let pus = (0..PHER_LEVELS)
    .map(|_| Box::new(AntSystemPU) as Box<dyn PheromoneUpdate<FMatrix>>)
    .collect_vec();
  let pu = PartFromEvalPU::new(pus);

  let colony = BinColony::new(ss.clone(), ants);
  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(pu)
    .set_fitness(fitness.clone())
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();

  algo.run()
}

fn run_acs_pu(problem: &Problem, ss: BinSharedState, size_count: usize, fitness: &BinFitness, probe: CsvProbe) {
  let ants = (0..ANTS).map(|_| BinAnt::new()).collect_vec();
  let colony = BinColony::new(ss.clone(), ants);
  let start_pheromone = FMatrix::repeat(size_count, size_count, 1.0);
  let algo = aco::Builder::new(problem.items.len())
    .set_colony(colony)
    .set_start_pheromone(start_pheromone)
    .set_pheromone_update(AntColonySystemPU::new())
    .set_fitness(fitness.clone())
    .set_probe(probe)
    .set_termination_condition(IterationCond::new(ITERS))
    .build();

  algo.run()
}


