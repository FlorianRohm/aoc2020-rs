#[macro_use]
extern crate pest_derive;


use parser::AocParser;

mod calculator;
mod parser;

fn main() {
    let input = include_str!("./input");
    let all_evaluations: i64 = input.lines().map(|line| parser::PREC_CLIMBER_PART_1.parse(line)).map(|argument| argument.eval()).sum();

    println!("All things evaluated are {}", all_evaluations);

    let all_evaluations: i64 = input.lines().map(|line| parser::PREC_CLIMBER_PART_2.parse(line)).map(|argument| argument.eval()).sum();

    println!("All things evaluated with the right arithmetic are {}", all_evaluations);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_part1() {
        assert_eq!(parser::PREC_CLIMBER_PART_1.parse("1 + (2 * 3) + (4 * (5 + 6))").eval(), 51);
        assert_eq!(parser::PREC_CLIMBER_PART_1.parse("2 * 3 + (4 * 5)").eval(), 26);
        assert_eq!(parser::PREC_CLIMBER_PART_1.parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").eval(), 437);
        assert_eq!(parser::PREC_CLIMBER_PART_1.parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").eval(), 12240);
        assert_eq!(parser::PREC_CLIMBER_PART_1.parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").eval(), 13632);
    }

    #[test]
    fn should_calculate_part2() {
        assert_eq!(parser::PREC_CLIMBER_PART_2.parse("1 + (2 * 3) + (4 * (5 + 6)) ").eval(), 51);
        assert_eq!(parser::PREC_CLIMBER_PART_2.parse("2 * 3 + (4 * 5) ").eval(), 46);
        assert_eq!(parser::PREC_CLIMBER_PART_2.parse("5 + (8 * 3 + 9 + 3 * 4 * 3) ").eval(), 1445);
        assert_eq!(parser::PREC_CLIMBER_PART_2.parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) ").eval(), 669060);
        assert_eq!(parser::PREC_CLIMBER_PART_2.parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 ").eval(), 23340);
    }
}