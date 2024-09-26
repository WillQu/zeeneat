use crate::gene::Genome;

const GENERATIONS: u32 = 10;
pub fn run(population_size: u32, input_size: u32, output_size: u32) {
    let mut population = Vec::new();
    for _ in 0..population_size {
        population.push(Genome::new(input_size, output_size));
    }
    for _ in 0..GENERATIONS {
        population = generation(population_size, &population);
    }
}

fn generation(population_size: u32, population: &[Genome]) -> Vec<Genome> {
    let mut best_genome = None;
    let mut best_fitness = 0.0;
    for genome in population {
        let fitness = genome.calculate_fitness();
        if fitness > best_fitness {
            best_fitness = fitness;
            best_genome = Some(genome);
        }
    }
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Best fitness: {}", best_fitness);
    // println!("Best genome: {:?}", best_genome);
    println!("Node count: {}", best_genome.unwrap().node_count());
    println!("Connection count: {}", best_genome.unwrap().connection_count());
    println!("0 xor 0 = {}", best_genome.unwrap().calculate_output(0.0, 0.0));
    println!("0 xor 1 = {}", best_genome.unwrap().calculate_output(0.0, 1.0));
    println!("1 xor 0 = {}", best_genome.unwrap().calculate_output(1.0, 0.0));
    println!("1 xor 1 = {}", best_genome.unwrap().calculate_output(1.0, 1.0));
    (0..population_size).map(|_| best_genome.unwrap().mutate()).collect()
}