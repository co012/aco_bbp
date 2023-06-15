use std::fs::OpenOptions;
use std::io::Write;

use ecrs::aco::fitness::Fitness;
use ecrs::aco::pheromone::Pheromone;
use ecrs::aco::probe::Probe;
use ecrs::aco::Solution;

use crate::fitness::BinFitness;

#[derive(Clone)]
pub struct CsvProbe {
    i2size: Vec<usize>,
    fitness: Vec<f64>,
    bins: Vec<usize>,
    label: String,
    pub file_post: String,
    bin_cap: usize,
    fit_op: BinFitness,
}

impl CsvProbe {
    pub fn new(i2size: Vec<usize>, label: String, bin_cap: usize) -> Self {
        let fit_op = BinFitness {
            stress_factor: 2.0,
            i2size: i2size.clone(),
            bin_cap,
        };
        Self {
            i2size,
            label,
            bins: vec![],
            file_post: String::from(""),
            bin_cap,
            fitness: vec![],
            fit_op,
        }
    }

    pub fn clone_exchange(&self, label: String) -> Self {
        let mut c = self.clone();
        c.label = label;
        c
    }

    fn flush(&mut self) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("results/bpp_results_{}.csv", self.file_post))
            .expect("Could not open file");

        for ((i, f), b) in self.fitness.iter().enumerate().zip(self.bins.iter()) {
            writeln!(file, "{},{},{},{}", i, f, b, self.label).expect("Error while writing to file");
        }
        file.flush().expect("Could not flush");
        println!("Completed {}", self.label)
    }
}

impl<P: Pheromone> Probe<P> for CsvProbe {
    fn on_current_best(&mut self, best: &Solution) {
        let mut bins_content: Vec<usize> = Vec::new();

        let mut curr_content = 0;
        for i in best.path.iter().cloned() {
            if curr_content + self.i2size[i] > self.bin_cap {
                bins_content.push(curr_content);
                curr_content = 0;
            }
            curr_content += self.i2size[i];
        }
        if curr_content > 0 {
            bins_content.push(curr_content);
        }

        let f = self.fit_op.apply(&best.path);
        self.fitness.push(f);
        self.bins.push(bins_content.len());
    }

    fn on_end(&mut self) {
        self.flush();
    }
}