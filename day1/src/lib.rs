pub fn solve() -> ((i32, i32, i32), (i32, i32, i32, i32)) {
    let input = include_str!("./input");
    let lines: Vec<i32> = input.lines()
        .map(|line| line.parse::<i32>().expect("input should be ints"))
        .collect();

    let (s1, s2) = solve_1(&lines).expect("no matching input found");

    let (s3, s4, s5) = solve_2(&lines).expect("no matching input found");
    ((s1 * s2, s1, s2), (s3 * s4 * s5, s3, s4, s5))
}

fn solve_1(input: &[i32]) -> Option<(i32, i32)> {
    use itertools::Itertools;
    input.iter().cartesian_product(input.iter())
        .find(|(&line1, &line2)| line1 + line2 == 2020)
        .map(|(&line1, &line2)| (line1, line2))
}

fn solve_2(input: &[i32]) -> Option<(i32, i32, i32)> {
    use itertools::Itertools;
    input.iter()
        .cartesian_product(input.iter())
        .cartesian_product(input.iter())
        .find(|((&line1, &line2), &line3)| line1 + line2 + line3 == 2020)
        .map(|((&line1, &line2), &line3)| (line1, line2, line3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day() {
        let solve1 = solve();
        assert_eq!(solve1.0.0, 73371);
        assert_eq!(solve1.1.0, 127642310);
    }

    #[test]
    fn should_solve_test_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let (s1, s2) = solve_1(&input).unwrap();

        assert_eq!(s1 * s2, 514579);
    }

    #[test]
    fn should_solve_test_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let (s1, s2, s3) = solve_2(&input).unwrap();

        assert_eq!(s1 * s2 * s3, 241861950);
    }
}