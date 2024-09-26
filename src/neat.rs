use crate::gene::Genome;

pub(crate) struct Neat {
    population: Vec<Genome>,
    input_size: u32,
    output_size: u32,
}

impl Neat {
    pub fn new(population_size: u32, input_size: u32, output_size: u32) -> Neat {
        let mut population = Vec::new();
        for _ in 0..population_size {
            population.push(Genome::new(input_size, output_size));
        }
        Neat {
            population,
            input_size,
            output_size,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut best_genome = None;
        let mut best_fitness = 0.0;
        for genome in &self.population {
            let fitness = genome.calculate_fitness();
            if fitness > best_fitness {
                best_fitness = fitness;
                best_genome = Some(genome);
            }
        }
        println!("Best fitness: {}", best_fitness);
    }
}