use crate::simulation_3d::Simulation3d;
use crate::simulation_4d::Simulation4d;

mod parse_3d;
mod cell;
mod simulation_3d;
mod simulation_4d;
mod parse_4d;

pub fn solve() -> (usize, usize) {
    let input = include_str!("./input");
    let simulation = Simulation3d::new(parse_3d::parse(input, 7));

    let active = simulation.propagate().propagate().propagate().propagate().propagate().propagate().count_active();

    println!("there are {} active cubes", active);

    let simulation = Simulation4d::new(parse_4d::parse(input, 7));

    let active_hyper = simulation.propagate().propagate().propagate().propagate().propagate().propagate().count_active();

    println!("there are {} active hyper cubes", active_hyper);

    (active, active_hyper)
}

#[cfg(test)]
mod tests {
    use crate::simulation_3d::Simulation3d;

    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (213, 1624));
    }

    #[test]
    fn test_input_3d() {
        let input = ".#.
..#
###";
        let base = parse_3d::parse(input, 7);
        let simulation = Simulation3d::new(base);
        let active = simulation.propagate().propagate().propagate().propagate().propagate().propagate().count_active();

        assert_eq!(active, 112)
    }

    #[test]
    fn test_input_4d() {
        let input = ".#.
..#
###";
        let base = parse_4d::parse(input, 7);
        let simulation = Simulation4d::new(base);
        let active = simulation.propagate().propagate().propagate().propagate().propagate().propagate().count_active();

        assert_eq!(active, 848)
    }
}