use std::collections::HashMap;

pub fn solve() -> (usize, i64) {
    let input = include_str!("./input");
    let input: Vec<i64> = input.lines().filter(|line| !line.is_empty()).map(|l| l.parse::<i64>().unwrap()).collect();
    let differences = get_differences(input);

    let joltage = solve_joltage(&differences);

    println!("solution is {}", joltage);

    let count = count_combinations(&differences);
    println!("there are {} possibilities", count);

    (joltage, count)
}

fn solve_joltage(input: &Vec<i64>) -> usize {
    let by: HashMap<i64, usize> = input.iter().fold(HashMap::new(), |mut acc, new| {
        *acc.entry(*new).or_insert(0) += 1;
        acc
    });


    by.get(&1).unwrap() * (by.get(&3).unwrap())
}

fn get_differences(mut input: Vec<i64>) -> Vec<i64> {
    use itertools::Itertools;

    let max = *input.iter().max().unwrap();
    input.append(&mut vec![0, max + 3]);
    input.sort();

    let map: Vec<i64> = input.iter().tuple_windows()
        .map(|(a, b)| b - a).collect();
    map
}

fn count_combinations(differences: &[i64]) -> i64 {
    differences.split(|a| a == &3).map(count_combinations_three_free).fold(1, |acc, new| acc * new)
}

fn count_combinations_three_free(differences: &[i64]) -> i64 {
    /*
    1,1,1 => (1,1 + rest) + (2 + rest) + (1, rest) + (3 + rest)
    2,1 => (2 + rest) + (3 + rest)
    1,2 => (1 + rest) + (2 + rest)
    2, 2/3 => rest
    3 => rest
    */

    match differences {
        [] | [_] => 1,
        [1, 1] | [1, 2] | [2, 1] => 2,
        [1, 1, 1] => 4,
        [1, 1, 2] | [1, 2, 1] | [2, 1, 1] => 3,
        [2, 2, 1] | [1, 2, 2] => 2,
        [2, 1, 2] => 3,
        [2, 2, 2] => 1,
        [1, 1, 1, 1] => 7,
        _ => panic!(format!("no matching pattern for {:?}", differences))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (1984, 3543369523456));
    }

    #[test]
    fn should_count_first() {
        let vec1 = vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3, ];

        let i = count_combinations(&vec1);

        assert_eq!(i, 8)
    }

    #[test]
    fn should_count_second() {
        let vec1 = vec![1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 3, 1, 1, 1, 1, 3, 1, 3, 3, 1, 1, 1, 1, 3];

        let i = count_combinations(&vec1);

        assert_eq!(i, 19208)
    }

    #[test]
    fn should_solve_first() {
        let vec1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let map = get_differences(vec1);

        let i = solve_joltage(&map);

        assert_eq!(i, 7 * 5)
    }


    #[test]
    fn should_count_differences() {
        assert_eq!(count_combinations(&vec![3]), 1, "3");
        assert_eq!(count_combinations(&vec![1, 3]), 1, "13");
        assert_eq!(count_combinations(&vec![1, 1, 3]), 2, "113");
    }
}