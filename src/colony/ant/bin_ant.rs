use ecrs::aco::FMatrix;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::prelude::StdRng;

use crate::colony::ant::MyAnt;
use crate::colony::BinSharedState;

#[derive(Clone)]
pub struct BinAnt {
    pub i2count: Vec<usize>,
    pub rng: StdRng,
    pub inside_bin: Vec<usize>,
}

unsafe impl Send for BinAnt {}

unsafe impl Sync for BinAnt {}


impl BinAnt {
    #[time_graph::instrument]
    fn try_finding_fitting_items(&self, ss: &BinSharedState) -> Vec<usize> {
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
    #[time_graph::instrument]
    fn perceived_pheromone(&self, pheromone: &FMatrix, possible_destinations: &[usize]) -> Vec<f64> {
        if self.inside_bin.is_empty() {
            return vec![1.0; possible_destinations.len()];
        }
        let mut pher = Vec::<f64>::with_capacity(possible_destinations.len());
        for i in possible_destinations.iter().cloned() {
            let mut p: f64 = self.inside_bin.iter().cloned().map(|j| pheromone[(j, i)]).sum();
            p /= self.inside_bin.len() as f64;
            pher.push(p)
        }
        pher
    }
    #[time_graph::instrument]
    fn choose_next(&mut self, fitting_items: Vec<usize>, goodness: Vec<f64>) -> Option<usize> {
        let sum = goodness.iter().sum();
        let mut random: f64 = self.rng.gen_range(0.0..=sum);
        for (v, g) in fitting_items.iter().zip(goodness) {
            random -= g;
            if random <= 0.0 {
                return Some(*v);
            }
        }

        fitting_items.last().map(Clone::clone)
    }

    /// Clears iteration specific data like visited vertices or path.
    #[time_graph::instrument]
    fn clear(&mut self, ss: &BinSharedState) {
        self.i2count.clone_from(&ss.i2count);
        self.inside_bin.clear();
    }

    fn chose_staring_place(&mut self) -> usize {
        self.rng.gen_range(0..self.i2count.len())
    }
    #[time_graph::instrument]
    fn go_to(&mut self, v: usize, path: &mut Vec<usize>) {
        self.i2count[v] -= 1;
        path.push(v);
        self.inside_bin.push(v);
    }

    pub fn new() -> Self {
        Self { i2count: vec![], rng: StdRng::from_entropy(), inside_bin: vec![] }
    }

    fn find_fitting_items(&mut self, ss: &BinSharedState) -> Vec<usize> {
        let fitting_items = self.try_finding_fitting_items(ss);
        if fitting_items.is_empty() {
            self.inside_bin.clear();
            return self.try_finding_fitting_items(ss)
        }
        fitting_items
    }
}

impl MyAnt<FMatrix> for BinAnt {
    #[time_graph::instrument]
    fn build_solution(&mut self, pheromone: &FMatrix, ss: &BinSharedState) -> Vec<usize> {

        let mut path = Vec::<usize>::with_capacity(ss.solution_size);
        self.clear(ss);
        let start = self.chose_staring_place();
        self.go_to(start,&mut path);

        for _ in 1..ss.solution_size {
            let fitting_items = self.find_fitting_items(ss);

            let pher = self.perceived_pheromone(pheromone, &fitting_items);

            let goodness = fitting_items.iter()
                .map(|x| ss.heuristic[*x])
                .zip(pher.iter())
                .map(|(h, p)| p.powf(ss.alpha) * h)
                .collect_vec();

            let next = self.choose_next(fitting_items, goodness).expect("Ant is stuck");

            self.go_to(next, &mut path);
        }

       path
    }
}
