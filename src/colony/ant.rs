use ecrs::aco::pheromone::Pheromone;
pub use bin_ant::BinAnt;
pub use bin_ant::PP;


use crate::colony::BinSharedState;

mod bin_ant;

pub trait MyAnt<P: Pheromone> {
    fn build_solution(&mut self, pheromone: &P, ss: &BinSharedState) -> Vec<usize>;
}



