use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Bag {
    adjective: String,
    color: String,
}

impl Bag {
    pub fn new(adjective: String, color: String) -> Bag {
        Bag { adjective, color }
    }
}


lazy_static::lazy_static! {
    static ref FIRST_BAG_MATCH: Regex = Regex::new("^(\\w*)\\s*(\\w*)").unwrap();
    static ref OTHER_BAG_MATCH: Regex = Regex::new("(\\d)+[,\\s]*(\\w*)[,\\s]*(\\w*)[,\\s]*bags?").unwrap();
}

pub fn parse_bag_line(input: &str) -> (Bag, Vec<(i32, Bag)>) {
    let first_bag = FIRST_BAG_MATCH.captures(input).expect("no first bag found");

    let first_bag = Bag::new((&first_bag[1]).to_owned(), (&first_bag[2]).to_owned());

    let including_bags: Vec<(i32, Bag)> = OTHER_BAG_MATCH.captures_iter(input)
        .map(|capture| ((&capture[1]).parse().expect("not parsable"), Bag::new((&capture[2]).to_owned(), (&capture[3]).to_owned())))
        .collect();

    (first_bag, including_bags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_bags() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let (original, inserts) = parse_bag_line(input);

        assert_eq!(original, Bag::new("light".to_owned(), "red".to_owned()));
        assert_eq!(inserts, vec![
            (1, Bag::new("bright".to_owned(), "white".to_owned())),
            (2, Bag::new("muted".to_owned(), "yellow".to_owned()))
        ]);
    }
}