mod bin_ant;
mod bin_ant_2d;
mod ant_dna;

pub use bin_ant::BinAnt;
pub use bin_ant_2d::BinAnt2D;
pub use bin_ant_2d::PerceivedPherStrat;
pub use ant_dna::DnaAnt;

use ecrs::aco::pheromone::Pheromone;
use crate::colony::BinSharedState;

pub trait MyAnt<P: Pheromone> {
  fn build_solution(&mut self, pheromone: &P, ss: &BinSharedState) -> Vec<usize>;
}

