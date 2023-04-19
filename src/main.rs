use std::time::Instant;
use ecrs::aco;
use ecrs::aco::FMatrix;
use ecrs::aco::pheromone::{AntSystemPU};
use ecrs::aco::termination_condition::IterationCond;
use itertools::Itertools;
use crate::colony::{BinColony, BinSharedState};
use crate::colony::ant::BinAnt;

mod probe;
mod fitness;
mod util;
mod colony;

const ANTS: usize = 50;
//const ITEMS: [usize; 120] = [98, 98, 98, 96, 96, 94, 93, 93, 92, 91, 91, 90, 87, 86, 85, 85, 84, 84, 84, 84, 84, 83, 83, 82, 82, 81, 80, 80, 80, 79, 79, 78, 78, 78, 78, 76, 74, 74, 73, 73, 73, 73, 72, 71, 70, 70, 70, 69, 69, 69, 67, 66, 64, 62, 62, 60, 60, 59, 58, 58, 58, 57, 57, 57, 57, 55, 55, 55, 50, 49, 49, 49, 47, 46, 46, 45, 45, 44, 44, 43, 43, 43, 43, 42, 42, 42, 42, 42, 41, 41, 41, 39, 39, 38, 38, 38, 37, 36, 36, 36, 35, 33, 33, 33, 32, 32, 30, 30, 30, 29, 28, 27, 27, 26, 25, 25, 24, 23, 23, 20, ];
//const ITEMS: [usize; 1000] = [100,100,100,100,100,100,100,100,100,99,99,99,99,99,99,99,99,98,98,98,98,98,98,98,98,98,97,97,97,97,97,97,97,97,97,97,97,97,97,96,96,96,96,96,96,96,96,96,96,96,96,96,96,95,95,95,95,95,94,94,94,94,94,94,94,94,94,94,94,94,94,94,93,93,93,93,93,93,93,93,93,93,93,93,93,93,92,92,92,92,92,92,92,92,92,92,92,91,91,91,91,91,91,91,91,91,91,91,91,91,91,90,90,90,90,90,90,90,90,90,90,90,90,89,89,89,89,89,89,89,89,89,89,89,89,89,89,89,88,88,88,88,88,88,88,88,88,88,88,87,87,87,87,87,87,87,87,87,87,87,87,87,87,87,87,87,86,86,86,86,86,86,86,86,86,86,86,86,86,85,85,85,85,85,85,85,85,85,85,85,85,85,84,84,84,84,84,84,84,84,84,84,84,84,83,83,83,83,83,83,83,83,83,83,83,83,82,82,82,82,82,82,82,82,82,82,82,82,82,82,82,81,81,81,81,81,81,81,81,81,81,81,81,81,81,81,81,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,80,79,79,79,79,79,79,79,79,79,79,79,79,79,79,79,78,78,78,78,78,78,78,78,78,78,78,78,78,78,77,77,77,77,77,77,77,77,77,76,76,76,76,76,76,76,76,76,76,76,76,76,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,74,74,74,74,74,74,74,74,74,74,74,74,74,74,74,74,73,73,73,73,73,73,73,73,73,73,73,73,73,72,72,72,72,72,72,72,72,72,71,71,71,71,71,71,71,71,71,71,71,71,71,71,71,70,70,70,70,70,70,70,70,70,69,69,69,69,69,69,69,69,69,68,68,68,68,68,68,68,68,68,67,67,67,67,67,67,67,67,67,67,67,67,67,67,66,66,66,66,66,66,66,66,66,66,66,66,65,65,65,65,65,65,65,65,65,65,65,64,64,64,64,64,64,64,64,64,64,63,63,63,63,63,63,63,63,63,63,63,63,62,62,62,62,62,62,62,62,61,61,61,61,61,61,61,61,61,61,61,61,61,60,60,60,60,60,60,60,60,60,60,60,59,59,59,59,59,59,59,59,58,58,58,58,58,58,58,58,58,58,58,58,58,57,57,57,57,57,57,57,57,57,57,57,57,57,57,57,57,57,57,56,56,56,56,56,56,56,56,56,56,55,55,55,55,55,55,55,55,55,55,55,55,55,55,55,55,55,54,54,54,54,54,54,54,54,54,54,54,54,54,53,53,53,53,53,53,53,53,53,53,53,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,52,51,51,51,51,51,51,51,51,51,51,51,51,51,51,51,50,50,50,50,50,50,50,50,50,50,50,50,50,50,49,49,49,49,49,49,49,49,48,48,48,48,48,48,48,48,48,48,48,47,47,47,47,47,47,47,47,47,47,47,47,47,47,47,47,46,46,46,46,46,46,46,46,46,46,46,46,46,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,45,44,44,44,44,44,44,44,44,44,43,43,43,43,43,43,43,43,43,42,42,42,42,42,42,42,42,42,42,42,41,41,41,41,41,41,41,41,41,41,41,41,40,40,40,40,40,40,40,40,39,39,39,39,39,39,39,38,38,38,38,38,38,38,38,38,37,37,37,37,37,37,37,37,37,37,37,36,36,36,36,36,36,36,36,36,36,36,35,35,35,35,35,35,35,35,35,35,35,35,35,35,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,31,31,31,31,31,31,31,31,31,31,31,31,31,30,30,30,30,30,30,30,30,30,30,30,30,29,29,29,29,29,29,28,28,28,28,28,28,28,28,28,28,28,28,28,28,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,26,26,26,26,26,26,26,26,26,26,25,25,25,25,25,25,25,25,25,25,25,25,24,24,24,24,24,24,24,24,24,24,24,24,23,23,23,23,23,23,23,23,23,23,23,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,21,21,21,21,21,21,21,21,21,20,20,20,20,20,20,20,20,20,20,20,20,20,20];
const ITEMS: [usize; 501] = [499,499,499,498,495,494,494,494,492,492,492,492,491,490,489,489,488,488,488,487,487,485,484,484,482,482,482,481,481,481,480,479,479,478,478,477,477,476,476,475,475,471,471,470,470,469,469,468,466,466,465,464,464,462,462,462,462,462,461,460,459,457,455,455,454,454,453,451,449,449,447,447,445,443,443,442,441,437,436,434,434,432,432,431,431,430,429,429,429,429,429,426,426,425,424,423,421,421,420,418,418,416,416,415,414,413,412,412,412,411,411,411,410,409,409,406,405,404,403,401,400,400,398,398,397,397,396,396,396,395,394,391,389,389,389,389,386,385,383,383,381,379,379,378,377,377,376,376,375,375,375,373,373,372,371,370,369,368,367,367,365,364,363,363,361,360,359,359,358,358,357,356,356,356,354,354,353,352,352,351,351,350,350,348,347,347,344,343,342,341,341,340,340,340,339,338,337,337,337,336,336,335,334,333,333,333,330,328,328,327,325,325,324,324,324,323,323,322,321,320,319,319,319,318,318,318,317,317,316,316,316,316,315,315,312,312,312,312,311,311,310,310,309,309,309,309,309,308,308,307,306,306,304,304,304,304,304,304,303,303,302,299,299,299,299,298,298,297,296,296,296,296,295,295,294,294,292,292,291,290,290,289,289,289,289,288,288,288,287,286,285,285,285,283,283,283,283,282,282,282,282,281,281,280,280,279,279,279,279,278,278,277,277,277,277,277,275,275,274,274,274,274,274,274,273,273,273,273,272,272,272,272,272,272,272,272,271,271,271,271,271,270,269,269,269,269,268,268,268,268,268,267,267,267,267,267,267,267,266,266,266,265,265,265,265,265,265,265,265,265,265,264,264,264,264,264,264,264,264,264,264,264,263,263,263,263,263,263,263,263,263,262,262,261,261,261,261,261,261,260,260,260,260,260,259,259,259,259,259,259,259,258,258,258,258,258,258,258,258,258,257,257,257,257,257,257,257,257,256,256,256,256,256,256,255,255,255,255,255,255,255,255,255,255,255,254,254,254,254,254,254,254,254,254,254,254,254,254,253,253,253,253,253,253,252,252,252,252,252,252,252,252,252,252,252,251,251,251,251,251,251,251,251,251,251,251,251,251,251,250,250,250,250,250,250,250,250,250,250,250,250];
const BIN_CAP: usize = 1000;


fn main() {
    let (size_count, size_to_index, index_to_size) = util::process_items(&ITEMS);
    let mut i2count = vec![0; size_count];
    for i in ITEMS {
        i2count[size_to_index[&i]] += 1;
    }

    let ants = (0..ANTS).map(|_| BinAnt::new()).collect_vec();
    let ss = BinSharedState {
        alpha: 1.0,
        beta: 2.0,
        i2size: index_to_size.clone(),
        solution_size: ITEMS.len(),
        bin_cap: BIN_CAP,
        i2count
    };

    let colony = BinColony::new(ss.clone(), ants);

    let start_pheromone = FMatrix::repeat(size_count, size_count, 1.0);

    let fitness = fitness::BinFitness{
        stress_factor: 2.0,
        i2size: index_to_size.clone(),
        bin_cap: BIN_CAP,
    };

    let probe = probe::CsvProbe::new(index_to_size.clone(), "as" , BIN_CAP);

    let algo = aco::Builder::new(ITEMS.len())
      .set_colony(colony)
      .set_start_pheromone(start_pheromone)
      .set_pheromone_update(AntSystemPU)
      .set_fitness(fitness)
      .set_probe(probe)
      .set_termination_condition(IterationCond::new(200))
      .build();

    let start = Instant::now();
    algo.run();
    println!("Time: {}", start.elapsed().as_millis());
}


