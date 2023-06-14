use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Problem {
    pub items: Vec<usize>,
    pub bin_cap: usize,
}

pub enum ProblemSet {
    Falkenauer,
    Hard28,
}

impl ProblemSet {
    const fn root(&self) -> &str {
        match self {
            ProblemSet::Falkenauer => "data/Falkenauer",
            ProblemSet::Hard28 => "data/Hard28",
        }
    }
}


pub struct ProblemLoader {
    problem_set: ProblemSet,
    uniform: bool,
    problem_size: usize,
}

impl ProblemLoader {
    pub fn new() -> Self {
        Self { problem_set: ProblemSet::Falkenauer, uniform: true, problem_size: 120 }
    }

    pub fn from(problem_set: ProblemSet) -> Self {
        Self { problem_set, uniform: true, problem_size: 120 }
    }


    pub fn pick_uniform(mut self, uniform: bool) -> Self {
        self.uniform = uniform;
        self
    }

    pub fn problem_size(mut self, problem_size: usize) -> Self {
        self.problem_size = problem_size;
        self
    }

    fn create_path(&self, problem_num: usize) -> String {
        match self.problem_set {
            ProblemSet::Falkenauer => {
                let type_path = if self.uniform { "Falkenauer U" } else { "Falkenauer_T" };
                let problem_prefix = if self.uniform { "Falkenauer_u" } else { "Falkenauer_t" };

                format!("{}/{}/{}{}_{:02}.txt", self.problem_set.root(), type_path, problem_prefix, self.problem_size, problem_num)
            }

            ProblemSet::Hard28 => {
                format!("{}/Hard28_BPP{}.txt", self.problem_set.root(), problem_num)
            }
        }
    }

    pub fn load_problem(&self, problem_num: usize) -> Problem {
        let path = self.create_path(problem_num);
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