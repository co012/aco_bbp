pub mod ant;

use std::marker::PhantomData;
use ecrs::aco::colony::Colony;
use ecrs::aco::pheromone::Pheromone;
use itertools::Itertools;
use rayon::prelude::*;
use crate::colony::ant::MyAnt;


#[derive(Clone)]
pub struct BinSharedState {
  pub alpha: f64,
  pub beta: f64,
  pub i2size: Vec<usize>,
  pub i2count: Vec<usize>,
  pub solution_size: usize,
  pub bin_cap: usize,
}

unsafe impl Send for BinSharedState {}
unsafe impl Sync for BinSharedState {}

#[derive(Clone)]
pub struct BinColony<P: Pheromone, A: MyAnt<P>> {
  shared_state: BinSharedState,
  ants: Vec<A>,
  _phantom: PhantomData<P>
}

impl<P: Pheromone, A: MyAnt<P>> BinColony<P, A> {
  pub fn new(shared_state: BinSharedState, ants: Vec<A>) -> Self {
    Self { shared_state, ants, _phantom: Default::default() }
  }
}

impl<P: Pheromone + Send + Sync,A: MyAnt<P> + Sync + Send> Colony<P> for BinColony<P, A> {
  fn build_solutions(&mut self, pheromone: &mut P) -> Vec<Vec<usize>> {
    self.ants.iter_mut()
      .map(|x| x.build_solution(pheromone, &self.shared_state))
      .collect()
  }
}