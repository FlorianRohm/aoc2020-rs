use get_input::get_raw_input;

fn main() {
    let input = get_raw_input(1, 1);
    let lines: Vec<i32> = input.lines()
        .map(|line| line.parse::<i32>().expect("input should be ints"))
        .collect();
    let solution = solve(lines);

    println!("the solution is {}", solution.expect("no matching input found"));
}

fn solve(input: Vec<i32>) -> Option<i32> {
    use itertools::Itertools;
    let x: Option<(&i32, &i32)> = input.iter().cartesian_product(input.iter())
        .find(|(&line1, &line2)| line1 + line2 == 2020);

    x.map(|(line1, line2)| line1 * line2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_test_input() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let solution = solve(input);

        assert_eq!(solution.unwrap(), 514579);
    }
}