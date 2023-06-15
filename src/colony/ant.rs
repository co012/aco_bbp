use ecrs::aco::pheromone::Pheromone;

pub use ant_dna::DnaAnt;
pub use bin_ant::BinAnt;
pub use bin_ant_2d::BinAnt2D;
pub use bin_ant_2d::PerceivedPherStrat;

use crate::colony::BinSharedState;

mod bin_ant;
mod bin_ant_2d;
mod ant_dna;

pub trait MyAnt<P: Pheromone> {
    fn build_solution(&mut self, pheromone: &P, ss: &BinSharedState) -> Vec<usize>;
}

