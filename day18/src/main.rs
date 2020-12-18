#[macro_use]
extern crate pest_derive;

use parser::parse;

mod calculator;
mod parser;

fn main() {
    let input = include_str!("./input");
    let all_evaluations: i64 = input.lines().map(|line| parse(line)).map(|argument| argument.eval()).sum();

    println!("All things evaluated are {}", all_evaluations);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate() {
        assert_eq!(parser::parse("1 + (2 * 3) + (4 * (5 + 6))").eval(), 51);
        assert_eq!(parser::parse("2 * 3 + (4 * 5)").eval(), 26);
        assert_eq!(parser::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").eval(), 437);
        assert_eq!(parser::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").eval(), 12240);
        assert_eq!(parser::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").eval(), 13632);
    }
}