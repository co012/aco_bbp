use std::collections::HashMap;
use std::marker::PhantomData;

use ecrs::aco::pheromone::{Pheromone, PheromoneUpdate};
use ecrs::aco::Solution;

pub fn process_items(items: &[usize]) -> (usize, HashMap<usize, usize>, Vec<usize>) {
    let mut size_count = 1;
    let mut size_to_index: HashMap<usize, usize> = HashMap::new();
    let mut index_to_size: Vec<usize> = Vec::new();
    size_to_index.insert(items[0], 0);
    index_to_size.push(items[0]);
    for i in 0..(items.len() - 1) {
        if items[i] != items[i + 1] {
            size_to_index.insert(items[i + 1], size_count);
            index_to_size.push(items[i + 1]);
            size_count += 1;
        }
    }
    (size_count, size_to_index, index_to_size)
}

pub struct TimedPU<Pher: Pheromone,P: PheromoneUpdate<Pher>> {
    pu: P,
    _phantom: PhantomData<Pher>
}

impl<Pher: Pheromone, P: PheromoneUpdate<Pher>> TimedPU<Pher, P> {
    pub fn new(pu: P) -> Self {
        Self { pu, _phantom: Default::default() }
    }
}


impl<Pher: Pheromone,P: PheromoneUpdate<Pher>> PheromoneUpdate<Pher> for TimedPU<Pher, P> {
    #[time_graph::instrument]
    fn apply(&mut self, pheromone: &mut Pher, solutions: &[Solution], evaporation_rate: f64) {
        self.pu.apply(pheromone, solutions,evaporation_rate)
    }
}