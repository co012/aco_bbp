use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Problem {
  pub items: Vec<usize>,
  pub bin_cap: usize,
}


pub struct ProblemLoader {
  root: &'static str,
  uniform: bool,
  problem_size: usize,
}

impl ProblemLoader {
  pub fn new() -> Self {
    Self { root: "data/Falkenauer", uniform: true, problem_size: 120 }
  }

  pub fn pick_uniform(mut self, uniform: bool) -> Self {
    self.uniform = uniform;
    self
  }

  pub fn problem_size(mut self, problem_size: usize) -> Self {
    self.problem_size = problem_size;
    self
  }

  pub fn load_problem(&self, problem_num: usize) -> Problem {
    let type_path = if self.uniform { "Falkenauer U" } else { "Falkenauer_T" };
    let problem_prefix = if self.uniform { "Falkenauer_u" } else { "Falkenauer_t" };

    let path = format!("{}/{}/{}{}_{:02}.txt", self.root, type_path, problem_prefix, self.problem_size, problem_num);

    let file = File::open(path).expect("File does not exists");
    let mut lines = io::BufReader::new(file).lines();
    let problem_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let bin_cap: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut items: Vec<usize> = Vec::with_capacity(problem_size);
    for line in lines.flatten() {
      items.push(line.parse().unwrap());

    }

    Problem {
      items,
      bin_cap,
    }
  }
}