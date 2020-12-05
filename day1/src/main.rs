
fn main() {
    let input = get_input::get_raw_input(1);
    let lines: Vec<i32> = input.lines()
        .map(|line| line.parse::<i32>().expect("input should be ints"))
        .collect();

    let solution = solve_1(&lines);
    println!("the first solution is {}", solution.expect("no matching input found"));


    let solution = solve_2(&lines);
    println!("the second solution is {}", solution.expect("no matching input found"));
}

fn solve_1(input: &[i32]) -> Option<i32> {
    use itertools::Itertools;
    let x: Option<(&i32, &i32)> = input.iter().cartesian_product(input.iter())
        .find(|(&line1, &line2)| line1 + line2 == 2020);

    x.map(|(line1, line2)| line1 * line2)
}

fn solve_2(input: &[i32]) -> Option<i32> {
    use itertools::Itertools;
    let x =
        input.iter()
        .cartesian_product(input.iter())
            .cartesian_product(input.iter())
        .find(|((&line1, &line2), &line3)| line1 + line2 + line3 == 2020);

    x.map(|((line1, line2), line3)| line1 * line2 * line3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_test_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let solution = solve_1(&input);

        assert_eq!(solution.unwrap(), 514579);
    }

    #[test]
    fn should_solve_test_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let solution = solve_2(&input);

        assert_eq!(solution.unwrap(), 241861950);
    }
}