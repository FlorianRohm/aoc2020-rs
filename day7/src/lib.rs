use std::collections::HashSet;

use multimap::MultiMap;

use crate::parser::Bag;

mod parser;

pub fn solve() -> (usize, i32) {
    let input = include_str!("./input");

    let luggage = get_number_of_possibilities_for(input, Bag::new("shiny".to_owned(), "gold".to_owned()));
    println!("There are {} possibilities to pack the luggage", luggage);

    let bags = get_number_of_bags_for(input, Bag::new("shiny".to_owned(), "gold".to_owned()));
    println!("There are {} bags needed", bags);
    (luggage, bags)
}

fn get_number_of_possibilities_for(input: &str, bag: Bag) -> usize {
    let mut map = get_reverse_map(input);

    let mut possible_baggages = HashSet::<Bag>::new();

    let mut baggages_to_check = map.get_vec(&bag).unwrap().to_owned();

    for index in 0.. {
        match baggages_to_check.get(index) {
            None => break,
            Some((_, new_bag)) => {
                possible_baggages.insert(new_bag.to_owned());
                if let Some(new) = map.get_vec_mut(new_bag) {
                    baggages_to_check.append(new);
                };
            }
        }
    }

    possible_baggages.len()
}

fn get_number_of_bags_for(input: &str, bag: Bag) -> i32 {
    let map = get_forward_map(input);

    get_number_of_bags_times(&map, &bag, 1) - 1
}

fn get_number_of_bags_times(input_map: &MultiMap<Bag, (i32, Bag)>, bag: &Bag, multiplicity: i32) -> i32 {
    match input_map.get_vec(bag) {
        Some(baggage) => {
            let x: i32 = baggage.iter()
                .map(|(new_multiplicity, new_bag)| new_multiplicity * get_number_of_bags_times(input_map, new_bag, multiplicity * new_multiplicity))
                .sum();
            1 + x
        }
        None => 1
    }
}


fn get_reverse_map(input: &str) -> MultiMap<Bag, (i32, Bag)> {
    let rules = parse_rules(input);

    let mut map = MultiMap::<Bag, (i32, Bag)>::new();

    for (in_rule, contain_rule) in rules {
        for (multiplicity, baggage) in contain_rule {
            map.insert(baggage, (multiplicity, in_rule.clone()));
        }
    }
    map
}

fn get_forward_map(input: &str) -> MultiMap<Bag, (i32, Bag)> {
    let rules = parse_rules(input);

    let mut map = MultiMap::<Bag, (i32, Bag)>::new();

    for (in_rule, contain_rule) in rules {
        for (multiplicity, baggage) in contain_rule {
            map.insert(in_rule.clone(), (multiplicity, baggage.clone()));
        }
    }
    map
}

fn parse_rules(input: &str) -> Vec<(Bag, Vec<(i32, Bag)>)> {
    input.lines().filter(|line| !line.is_empty()).map(parser::parse_bag_line)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (226, 9569));
    }

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    const INPUT_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn should_give_correct_answer_for_our_bag() {
        let possibilities = get_number_of_possibilities_for(INPUT, Bag::new("shiny".to_owned(), "gold".to_owned()));
        assert_eq!(possibilities, 4)
    }

    #[test]
    fn should_count_total_bags_needed() {
        let possibilities = get_number_of_bags_for(INPUT, Bag::new("shiny".to_owned(), "gold".to_owned()));
        assert_eq!(possibilities, 32)
    }

    #[test]
    fn should_count_total_bags_needed_2() {
        let possibilities = get_number_of_bags_for(INPUT_2, Bag::new("shiny".to_owned(), "gold".to_owned()));
        assert_eq!(possibilities, 126)
    }
}