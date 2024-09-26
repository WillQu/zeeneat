use crate::neat::run;

mod gene;
mod neat;
mod sigmoid;

fn main() {
    run(100, 2, 1);
}
