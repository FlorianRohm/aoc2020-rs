fn main() {
    let input: Vec<&str> = include_str!("./input").lines().collect();
    let arrival: CalculationType = input[0].parse().expect("not a number");
    let lines: Vec<(BusIndex, BusId)> = input[1].split(",").enumerate()
        .filter(|&(_, x)| x.ne("x"))
        .map(|(index, x)| (index as CalculationType, x.parse().expect("not a number")))
        .collect();

    let (bus, wait) = get_line_and_departure(arrival, &*lines);

    println!("We're taking bus line {} and have to wait {}. Solution: {}", bus, wait, bus * wait);

    println!("remainder theorem: {}", solve_remainder_theorem(&lines));
}

type CalculationType = i64;
type BusId = CalculationType;
type BusIndex = CalculationType;

fn get_line_and_departure(arrival: CalculationType, lines: &[(BusIndex, BusId)]) -> (BusId, CalculationType) {
    lines.iter().map(|&(_, bus)| (bus, bus - arrival % bus)).min_by_key(|(_bus, wait)| *wait).expect("empty")
}

/// to get the time of the first departure, we have to solve
/// t = 0 mod 7
/// t = 1 mod 13
/// t = ai mod mi
/// ...
/// which is the chinese remainder theorem.
/// Manual inspection of the input proves that all BusIds are prime.
#[allow(non_snake_case)]
fn solve_remainder_theorem(lines: &[(BusIndex, BusId)]) -> CalculationType {
    let M: CalculationType = lines.iter().map(|&(_, id)| id).product();

    let one_solution: CalculationType = lines.iter().map(|&(ai, mi)| {
        use num_integer::ExtendedGcd;
        use num_integer::Integer;

        let Mi = M / mi;

        let ExtendedGcd { gcd: g, x: si, .. } = Mi.extended_gcd(&mi);

        let ei = si * Mi;
        debug_assert_eq!(g, 1);
        ai * ei
    }).sum();

    one_solution.abs() % M
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_part1() {
        let (bus, wait) = get_line_and_departure(939, &vec![(1, 7), (2, 13), (4, 59), (5, 31), (4, 19)]);

        assert_eq!(bus, 59);
        assert_eq!(wait, 5);
    }

    #[test]
    fn should_solve_remainder_theorem() {
        assert_eq!(solve_remainder_theorem(&vec![(0, 17), (2, 13), (3, 19)]), 3417)
    }

    #[test]
    fn should_solve_remainder_theorem_2() {
        assert_eq!(solve_remainder_theorem(&vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]), 1068781)
    }

    #[test]
    fn should_solve_remainder_theorem_3() {
        assert_eq!(solve_remainder_theorem(&vec![(0, 67), (1, 7), (2, 59), (3, 61)]), 754018);
    }


    #[test]
    fn should_solve_remainder_theorem_4() {
        assert_eq!(solve_remainder_theorem(&vec![(0, 67), (2, 7), (3, 59), (4, 61)]), 779210);
    }

    #[test]
    fn should_solve_remainder_theorem_5() {
        assert_eq!(solve_remainder_theorem(&vec![(0, 67), (1, 7), (3, 59), (4, 61)]), 1261476);
    }

    #[test]
    fn should_solve_remainder_theorem_6() {
        assert_eq!(solve_remainder_theorem(&vec![(0, 1789), (1, 37), (2, 47), (3, 1889)]), 1202161486);
    }
}
