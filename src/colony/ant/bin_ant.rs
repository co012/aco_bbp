use ecrs::aco::FMatrix;
use ecrs::aco::pheromone::Pheromone;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::prelude::StdRng;

use crate::colony::ant::MyAnt;
use crate::colony::BinSharedState;

trait PPApply<P: Pheromone> {
    fn apply(&self, inside_bin: &[usize], pheromone: &P, fitting_items: &[usize], rand: &mut impl Rng) -> Vec<f64>;
}
#[derive(Clone)]
pub enum PP {
    Dna(Vec<f64>),
    IterationExpRand,
    ItemExpRand,
}

impl PPApply<FMatrix> for PP {
    fn apply(&self, inside_bin: &[usize], pheromone: &FMatrix, fitting_items: &[usize], _ : &mut impl Rng) -> Vec<f64>{
        if inside_bin.is_empty() {
            return vec![1.0; fitting_items.len()];
        }
        let mut pher = Vec::<f64>::with_capacity(fitting_items.len());
        for i in fitting_items.iter().cloned() {
            let mut p: f64 = inside_bin.iter().cloned().map(|j| pheromone[(j, i)]).sum();
            p /= inside_bin.len() as f64;
            pher.push(p)
        }
        pher
    }
}

impl PP {

    fn apply_dna(inside_bin: &[usize], pheromone: &[FMatrix], fitting_items: &[usize], dna: &[f64]) -> Vec<f64> {
        if inside_bin.is_empty() {
            return vec![1.0; fitting_items.len()];
        }
        let mut pher = Vec::with_capacity(fitting_items.len());
        for u in fitting_items.iter().cloned() {
            let mut p = 0f64;
            for (pheromone_level, weight) in pheromone.iter().zip(dna.iter().cloned()) {
                for v in inside_bin.iter().cloned() {
                    p += pheromone_level[(u, v)] * weight
                }
            }
            p = p.max(0.0);
            pher.push(p / inside_bin.len() as f64)
        }
        pher
    }

    fn apply_iter_exp_rand(inside_bin: &[usize], pheromone: &[FMatrix], fitting_items: &[usize], rand: &mut impl Rng) -> Vec<f64> {
        if inside_bin.is_empty() {
            return vec![1.0; fitting_items.len()];
        }
        let mut pher = Vec::with_capacity(fitting_items.len());
        let pheromone_level = exp_less(rand, pheromone.len());
        for i in fitting_items.iter().cloned() {
            let mut p: f64 = inside_bin.iter().cloned().map(|j| pheromone[pheromone_level][(j, i)]).sum();
            p /= inside_bin.len() as f64;
            pher.push(p)
        }
        pher
    }

    fn apply_item_exp_rand(inside_bin: &[usize], pheromone: &[FMatrix], fitting_items: &[usize], rand: &mut impl Rng) -> Vec<f64> {
        if inside_bin.is_empty() {
            return vec![1.0; fitting_items.len()];
        }
        let mut pher = Vec::with_capacity(fitting_items.len());
        for i in fitting_items.iter().cloned() {
            let pheromone_level = exp_less(rand, pheromone.len());
            let mut p: f64 = inside_bin.iter().cloned().map(|j| pheromone[pheromone_level][(j, i)]).sum();
            p /= inside_bin.len() as f64;
            pher.push(p)
        }
        pher
    }

}


impl PPApply<Vec<FMatrix>> for PP {
    fn apply(&self, inside_bin: &[usize], pheromone: &Vec<FMatrix>, fitting_items: &[usize], rand: &mut impl Rng) -> Vec<f64> {

        match self {
            PP::Dna(dna) => PP::apply_dna(inside_bin, pheromone, fitting_items, dna),
            PP::IterationExpRand=> PP::apply_iter_exp_rand(inside_bin, pheromone, fitting_items, rand),
            PP::ItemExpRand => PP::apply_item_exp_rand(inside_bin, pheromone, fitting_items, rand)

        }
    }
}

fn exp_less(rng: &mut impl Rng, end: usize) -> usize {
    for i in 1..end {
        if rng.gen::<bool>() {
            return end - i;
        }
    }
    0
}

#[derive(Clone)]
pub struct BinAnt<R: Rng + Clone = StdRng> {
    pub i2count: Vec<usize>,
    pub rng: R,
    pub inside_bin: Vec<usize>,
    pub pp: PP
}



impl<R: Rng + Clone> BinAnt<R> {
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



    pub fn rng_and_pp(pp: PP, rng: R) -> Self {
        Self{ i2count: vec![], rng, inside_bin: vec![], pp }
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

impl BinAnt<StdRng> {
    pub fn new() -> Self {
        Self::with_pp(PP::ItemExpRand)
    }

    pub fn with_pp(pp: PP) -> Self {
        Self::rng_and_pp(pp, StdRng::from_entropy())
    }
}

impl<R: Rng + Clone> MyAnt<FMatrix> for BinAnt<R> {
    #[time_graph::instrument]
    fn build_solution(&mut self, pheromone: &FMatrix, ss: &BinSharedState) -> Vec<usize> {

        let mut path = Vec::<usize>::with_capacity(ss.solution_size);
        self.clear(ss);
        let start = self.chose_staring_place();
        self.go_to(start,&mut path);

        for _ in 1..ss.solution_size {
            let fitting_items = self.find_fitting_items(ss);

            let pher = self.pp.apply(&self.inside_bin, pheromone, &fitting_items, &mut self.rng);

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

impl<R: Rng + Clone> MyAnt<Vec<FMatrix>> for BinAnt<R> {
    #[time_graph::instrument]
    fn build_solution(&mut self, pheromone: &Vec<FMatrix>, ss: &BinSharedState) -> Vec<usize> {

        let mut path = Vec::<usize>::with_capacity(ss.solution_size);
        self.clear(ss);
        let start = self.chose_staring_place();
        self.go_to(start,&mut path);

        for _ in 1..ss.solution_size {
            let fitting_items = self.find_fitting_items(ss);

            let pher = self.pp.apply(&self.inside_bin, pheromone, &fitting_items, &mut self.rng);

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
