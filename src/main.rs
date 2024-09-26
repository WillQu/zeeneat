mod gene;
mod neat;
mod sigmoid;

use neat::Neat;

fn main() {
    let mut neat = Neat::new(100, 2, 1);
    neat.run();
}
