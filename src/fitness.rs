use ecrs::aco::fitness::Fitness;

#[derive(Clone)]
pub struct BinFitness {
  pub stress_factor: f64,
  pub i2size: Vec<usize>,
  pub bin_cap: usize
}

impl Fitness for BinFitness{
  fn apply(&mut self, path: &[usize]) -> f64 {
    let mut bins_content: Vec<usize> = Vec::new();

    let mut curr_content = 0;
    for i in path.iter().cloned() {
      if curr_content + self.i2size[i] > self.bin_cap {
        bins_content.push(curr_content);
        curr_content = 0;
      }
      curr_content += self.i2size[i];
    }
    if curr_content > 0 {
      bins_content.push(curr_content);
    }

    let fitness : f64 =  bins_content.iter()
      .map(|x| *x as f64 / self.bin_cap as f64)
      .map(|x| x.powf(self.stress_factor))
      .sum();
    fitness / bins_content.len() as f64

  }

}