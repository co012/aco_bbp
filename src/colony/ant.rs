use ecrs::aco::FMatrix;
use ecrs::aco::pheromone::Pheromone;
use itertools::Itertools;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::colony::BinSharedState;

pub trait MyAnt<P: Pheromone> {
  fn build_solution(&mut self, pheromone: &P, ss: &BinSharedState) -> Vec<usize>;
}

#[derive(Clone)]
pub struct BinAnt {
  pub i2count: Vec<usize>,
  pub path: Vec<usize>,
  pub rng: ThreadRng,
  pub inside_bin: Vec<usize>,
}


impl BinAnt {
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

  fn perceived_pheromone(&self, pheromone: &FMatrix, possible_destinations: &[usize]) -> Vec<f64> {
    if self.inside_bin.is_empty() {
      return vec![1.0; possible_destinations.len()]
    }
    let mut pher = vec![];
    for i in possible_destinations.iter().cloned() {
      let mut p: f64 = self.inside_bin.iter().cloned().map(|j| pheromone[(j, i)]).sum();
      p /= self.inside_bin.len() as f64;
      pher.push(p)
    }
    pher
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

  pub fn new() -> Self {
    Self { i2count: vec![], path: vec![], rng: Default::default(), inside_bin: vec![] }
  }
}

impl MyAnt<FMatrix> for BinAnt {
  fn build_solution(&mut self, pheromone: &FMatrix, ss: &BinSharedState) -> Vec<usize> {
    self.clear(ss);
    let start = self.chose_staring_place();
    self.go_to(start);

    for _ in 1..ss.solution_size {
      let tmp = self.find_destinations(ss);
      let fitting_items = if tmp.is_empty() {
        self.inside_bin.clear();
        self.find_destinations(ss)
      } else { tmp };

      let pher = self.perceived_pheromone(pheromone, &fitting_items);

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