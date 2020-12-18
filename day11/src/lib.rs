use crate::grid::Simulation;

mod grid;
mod parse;

pub fn solve() -> (usize, usize) {
    let input = include_str!("./input");

    let sim = Simulation::new(parse::parse(input));

    let sim = sim.propagate_until_hold();

    let first_occupations = sim.count_seats();
    println!("there are {} seats occupied", first_occupations);
    let sim = Simulation::new(parse::parse(input));
    let sim = sim.propagate_wide_hold();

    let second_occupations = sim.count_seats();
    println!("there are {} seats occupied with new rules", second_occupations);

    (first_occupations, second_occupations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (2113, 1865));
    }
}
