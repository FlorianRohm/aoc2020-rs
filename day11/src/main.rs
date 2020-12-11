use crate::grid::Simulation;

mod grid;
mod parse;

fn main() {
    let input = include_str!("./input");

    let sim = Simulation::new(parse::parse(input));

    let sim = sim.propagate_until_hold();

    println!("there are {} seats occupied", sim.count_seats());
    let sim = Simulation::new(parse::parse(input));
    let sim = sim.propagate_wide_hold();

    println!("there are {} seats occupied with new rules", sim.count_seats())
}
