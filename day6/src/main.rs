use std::collections::HashSet;

fn main() {
    let input = include_str!("./input");
    let puzzle_answer_1 = count_answers(input, crate_set_from_single_group_any);
    println!("The sum of customs to declare is {}", puzzle_answer_1);

    let puzzle_answer_2 = count_answers(input, crate_set_from_single_group_all);
    println!("The REAL sum of customs to declare is {}", puzzle_answer_2);
}

pub fn count_answers(input: &str, create_set: impl Fn(&str) -> HashSet<char>) -> usize {
    create_sets_from_all_groups(input, create_set).map(|hashset| hashset.len()).sum()
}

fn create_sets_from_all_groups<'a>(input: &'a str, create_set: impl Fn(&'a str) -> HashSet<char> + 'a) -> impl Iterator<Item=HashSet<char>> + 'a {
    input.split("\n\n").map(create_set)
}

fn crate_set_from_single_group_any(input: &str) -> HashSet<char> {
    input.lines().into_iter().flat_map(|line| line.chars().into_iter()).collect()
}

fn crate_set_from_single_group_all(input: &str) -> HashSet<char> {
    use reduce::Reduce;

    input.lines().into_iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap_or(HashSet::new())
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b
";

    #[test]
    fn should_count_sets_1() {
        assert_eq!(count_answers(INPUT, crate_set_from_single_group_any), 11);
    }


    #[test]
    fn should_count_sets_2() {
        assert_eq!(count_answers(INPUT, crate_set_from_single_group_all), 6);
    }

    macro_rules! set {
        ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
            {
                let mut temp_set = HashSet::new();  // Create a mutable HashSet
                $(
                    temp_set.insert($x); // Insert each item matched into the HashSet
                )*
                temp_set // Return the populated HashSet
            }
        };
    }


    #[test]
    fn should_create_set_from_inputs() {
        let sets: Vec<HashSet<char>> = create_sets_from_all_groups(INPUT, crate_set_from_single_group_any).collect();
        assert_eq!(sets, vec![
            set!['a', 'b', 'c'],
            set!['a', 'b', 'c'],
            set!['a', 'b', 'c'],
            set!['a'],
            set!['b']]
        );
    }

    #[test]
    fn should_count_one_group() {
        let set = crate_set_from_single_group_any("abc");

        assert_eq!(set, set!['a', 'b', 'c'])
    }

    #[test]
    fn should_count_one_group_new_lines() {
        let input = "a
b
c";
        let set = crate_set_from_single_group_any(input);

        assert_eq!(set, set!['a', 'b', 'c'])
    }

    #[test]
    fn should_count_one_group_second_part() {
        let set = crate_set_from_single_group_all("abc");

        assert_eq!(set, set!['a', 'b', 'c'])
    }

    #[test]
    fn should_count_one_group_new_lines_second_part() {
        let input = "ac
ac
ab";
        let set = crate_set_from_single_group_all(input);

        assert_eq!(set, set!['a'])
    }
}