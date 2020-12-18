use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::calculator::{Argument, Operand};
use crate::calculator::Argument::Literal;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct CalcParser;

impl From<&str> for Operand {
    fn from(input: &str) -> Self {
        match input {
            "+" => Operand::PLUS,
            "*" => Operand::TIMES,
            _ => panic!("operand not supported")
        }
    }
}

pub fn parse(input: &str) -> Argument {
    let mut calculation = CalcParser::parse(Rule::Calculation, input).unwrap_or_else(|e| panic!("{}", e));

    parse_rules(calculation.into_iter().next().unwrap().into_inner())
}

fn parse_rules(mut input: pest::iterators::Pairs<Rule>) -> Argument {
    let last = input.next_back();
    let second_to_last = input.next_back();
    match (last, second_to_last) {
        (Some(last), Some(second)) => inner_to_argument(input, second, last),
        (Some(l), None) => rule_to_argument(l),

        _ => unimplemented!()
    }
}

fn rule_to_argument(input: Pair<Rule>) -> Argument {
    match input.as_rule() {
        Rule::Digit => Argument::Literal(input.as_str().trim().parse::<i64>().unwrap()),
        Rule::SubExpr => parse_rules(input.into_inner()),
        _ => unreachable!()
    }
}

fn inner_to_argument(remainder: Pairs<Rule>, operand: Pair<Rule>, last: Pair<Rule>) -> Argument {
    let arg_1 = parse_rules(remainder);
    let arg_2 = rule_to_argument(last);
    let operand = operand.as_str().trim().into();

    Argument::Expression {
        arg_1: Box::new(arg_1),
        operand,
        arg_2: Box::new(arg_2),
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::Argument;
    use crate::calculator::Operand;

    use super::*;

    #[test]
    fn should_parse_simple() {
        let parsed = parse("2 + 4");
        assert_eq!(parsed,
                   Argument::Expression {
                       operand: Operand::PLUS,
                       arg_1: Box::new(Argument::Literal(2)),
                       arg_2: Box::new(Argument::Literal(4)),
                   });
    }

    #[test]
    fn should_parse_sub_term() {
        let parsed = parse("2 + (4*2)");
        assert_eq!(parsed,
                   Argument::Expression {
                       operand: Operand::PLUS,
                       arg_1: Box::new(Argument::Literal(2)),
                       arg_2: Box::new(Argument::Expression {
                           operand: Operand::TIMES,
                           arg_1: Box::new(Argument::Literal(4)),
                           arg_2: Box::new(Argument::Literal(2)),
                       }),
                   });
    }

    #[test]
    fn should_parse_multiple_term() {
        let parsed = parse("2 * 3 + (4 * 5)");
        assert_eq!(parsed,
                   Argument::Expression {
                       operand: Operand::PLUS,
                       arg_1: Box::new(Argument::Expression {
                           operand: Operand::TIMES,
                           arg_1: Box::new(Argument::Literal(2)),
                           arg_2: Box::new(Argument::Literal(3)),
                       }),
                       arg_2: Box::new(Argument::Expression {
                           operand: Operand::TIMES,
                           arg_1: Box::new(Argument::Literal(4)),
                           arg_2: Box::new(Argument::Literal(5)),
                       }),
                   });
    }
}