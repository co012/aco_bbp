use std::fs::OpenOptions;
use std::io::Write;
use ecrs::aco::pheromone::Pheromone;
use ecrs::aco::probe::Probe;
use ecrs::aco::Solution;

#[derive(Clone)]
pub struct CsvProbe {
  i2size: Vec<usize>,
  fitness: Vec<usize>,
  label: &'static str,
  bin_cap: usize
}

impl CsvProbe {

  pub fn new(i2size: Vec<usize>, label:&'static str, bin_cap: usize) -> Self {
    Self {
      i2size,
      label,
      bin_cap,
      fitness: vec![]
    }
  }

  pub fn clone_exchange(&self, label: &'static str) -> Self {
    let mut c = self.clone();
    c.label = label;
    c
  }

  fn flush(&mut self) {
    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("bpp_results.csv")
      .expect("Could not open file");

    for (i, f) in self.fitness.iter().enumerate() {
      writeln!(file, "{},{},{}", i, f, self.label).expect("Error while writing to file");
    }
    file.flush().expect("Could not flush")
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

    println!("{} {}", bins_content.len(), best.fitness);
    self.fitness.push(bins_content.len());
  }

  fn on_end(&mut self) {
    self.flush();
  }
}