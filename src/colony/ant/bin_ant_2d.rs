use crate::colony::ant::MyAnt;
use crate::colony::BinSharedState;
use ecrs::aco::FMatrix;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone)]
pub enum PerceivedPherStrat {
  IterOnce,
  EveryItem
}

impl PerceivedPherStrat {
  fn perceived_pheromone_every_item(&self, ant: &mut BinAnt2D, pheromone: &Vec<FMatrix>, possible_destinations: &[usize]) -> Vec<f64> {
    if ant.inside_bin.is_empty() {
      return vec![1.0; possible_destinations.len()]
    }
    let mut pher = vec![];
    for i in possible_destinations.iter().cloned() {
      let pheromone_level = exp_less(&mut ant.rng, pheromone.len());
      let mut p: f64 = ant.inside_bin.iter().cloned().map(|j| pheromone[pheromone_level][(j, i)]).sum();
      p /= ant.inside_bin.len() as f64;
      pher.push(p)
    }
    pher
  }

  fn perceived_pheromone_once(&self, ant: &mut BinAnt2D, pheromone: &Vec<FMatrix>, possible_destinations: &[usize]) -> Vec<f64> {
    if ant.inside_bin.is_empty() {
      return vec![1.0; possible_destinations.len()]
    }
    let mut pher = vec![];
    let pheromone_level = exp_less(&mut ant.rng, pheromone.len());
    for i in possible_destinations.iter().cloned() {

      let mut p: f64 = ant.inside_bin.iter().cloned().map(|j| pheromone[pheromone_level][(j, i)]).sum();
      p /= ant.inside_bin.len() as f64;
      pher.push(p)
    }
    pher
  }

  fn perceived_pher(&self, ant: &mut BinAnt2D, pheromone: &Vec<FMatrix>, possible_destinations: &[usize]) -> Vec<f64> {
    match self {
      PerceivedPherStrat::IterOnce => self.perceived_pheromone_once(ant, pheromone, possible_destinations),
      PerceivedPherStrat::EveryItem => self.perceived_pheromone_every_item(ant, pheromone, possible_destinations)
    }
  }
}

#[derive(Clone)]
pub struct BinAnt2D {
  pub i2count: Vec<usize>,
  pub path: Vec<usize>,
  pub rng: StdRng,
  pub inside_bin: Vec<usize>,
  pub pp: PerceivedPherStrat
}


unsafe impl Send for BinAnt2D {}
unsafe impl Sync for BinAnt2D {}



impl BinAnt2D {
  fn find_destinations(&self, ss: &BinSharedState) -> Vec<usize> {
    let place_taken: usize = self.inside_bin.iter()
      .cloned()
      .map(|x| ss.i2size[x])
      .sum();

    let place_left = ss.bin_cap - place_taken;

    let mut fit_items = Vec::<usize>::new();
    for i in (0..self.i2count.len()).rev() {
      if ss.i2size[i] > place_left {
        break;
      }

      if self.i2count[i] > 0 {
        fit_items.push(i);
      }
    }

    fit_items
  }


  fn choose_next(&mut self, fitting_items: Vec<usize>, goodness: Vec<f64>) -> Option<usize> {
    let sum = goodness.iter().sum();
    let mut random: f64 = self.rng.gen_range(0.0..=sum);
    for (v, g) in fitting_items.iter().zip(goodness) {
      random -= g;
      if random <= 0.0 {
        return  Some(*v);
      }
    }

    fitting_items.last().map(Clone::clone)
  }

  /// Clears iteration specific data like visited vertices or path.
  fn clear(&mut self, ss: &BinSharedState) {
    self.i2count = ss.i2count.clone();
    self.path.clear();
    self.inside_bin.clear();
  }
  /// Selects an vertex to start from
  fn chose_staring_place(&mut self) -> usize {
    self.rng.gen_range(0..self.i2count.len())
  }

  fn go_to(&mut self, v: usize) {
    self.i2count[v] -= 1;
    self.path.push(v);
    self.inside_bin.push(v);
  }

  pub fn new(pp: PerceivedPherStrat) -> Self {
    Self { i2count: vec![], path: vec![], rng: StdRng::from_entropy(), inside_bin: vec![], pp }
  }
}

impl MyAnt<Vec<FMatrix>> for BinAnt2D {
  #[time_graph::instrument]
  fn build_solution(&mut self, pheromone: &Vec<FMatrix>, ss: &BinSharedState) -> Vec<usize> {
    self.clear(ss);
    let start = self.chose_staring_place();
    self.go_to(start);

    for _ in 1..ss.solution_size {
      let tmp = self.find_destinations(ss);
      let fitting_items = if tmp.is_empty() {
        self.inside_bin.clear();
        self.find_destinations(ss)
      } else { tmp };

      let pher = self.pp.clone().perceived_pher(self, pheromone, &fitting_items);

      let goodness = fitting_items.iter()
        .map(|x| ss.i2size[*x] as f64 / ss.bin_cap as f64)
        .zip(pher.iter())
        .map(|(h, p)| p.powf(ss.alpha) * h.powf(ss.beta))
        .collect_vec();

      let next = self.choose_next(fitting_items, goodness).expect("Ant is stuck");


      self.go_to(next);
    }

    self.path.clone()
  }
}

fn exp_less(rng: &mut StdRng, end: usize) -> usize {
  for i in 1..end {
    if rng.gen::<bool>() {
      return end - i;
    }
  }

  0
}
