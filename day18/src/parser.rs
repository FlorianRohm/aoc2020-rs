use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::calculator::{Argument, Operand};

#[derive(Parser)]
#[grammar = "parser.pest"]
struct CalcParser;

impl From<&str> for Operand {
    fn from(input: &str) -> Self {
        match input.trim() {
            "+" => Operand::PLUS,
            "*" => Operand::TIMES,
            _ => panic!("operand not supported")
        }
    }
}

lazy_static::lazy_static! {
    pub static ref PREC_CLIMBER_PART_2: pest::prec_climber::PrecClimber<Rule> = {
        use Rule::{Add, Multiply};
        use pest::prec_climber::{Operator, Assoc};
        use pest::prec_climber::PrecClimber;

        PrecClimber::new(vec![
            Operator::new(Multiply, Assoc::Left),
            Operator::new(Add, Assoc::Left)
        ])
    };

    pub static ref PREC_CLIMBER_PART_1: pest::prec_climber::PrecClimber<Rule> = {
        use Rule::{Add, Multiply};
        use pest::prec_climber::{Operator, Assoc};
        use pest::prec_climber::PrecClimber;

        PrecClimber::new(vec![
            Operator::new(Multiply, Assoc::Left) | Operator::new(Add, Assoc::Left),

        ])
    };
}

pub trait AocParser {
    fn eval(&self, expression: Pairs<Rule>) -> Argument;
    fn parse(&self, input: &str) -> Argument;
}

impl AocParser for pest::prec_climber::PrecClimber<Rule> {
    fn eval(&self, expression: Pairs<Rule>) -> Argument {
        self.climb(
            expression,
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::Digit => Argument::Literal(pair.as_str().trim().parse::<i64>().unwrap()),
                Rule::SubExpr => self.eval(pair.into_inner()),
                Rule::Expr => self.eval(pair.into_inner()),
                _ => unreachable!(),
            },
            |lhs: Argument, op: Pair<Rule>, rhs: Argument| match op.as_rule() {
                Rule::Add | Rule::Multiply => Argument::Expression {
                    arg_1: Box::from(lhs),
                    operand: op.as_str().into(),
                    arg_2: Box::from(rhs),
                },
                _ => unreachable!(),
            },
        )
    }

    fn parse(&self, input: &str) -> Argument {
        let calculation = CalcParser::parse(Rule::Calculation, input).unwrap_or_else(|e| panic!("{}", e));

        self.eval(calculation.into_iter().next().unwrap().into_inner())
    }
}


#[cfg(test)]
mod tests {
    use crate::calculator::Argument;
    use crate::calculator::Operand;

    use super::*;

    #[test]
    fn should_parse_simple_2() {
        let parsed = PREC_CLIMBER_PART_2.parse("2 + 4");
        assert_eq!(parsed,
                   Argument::Expression {
                       operand: Operand::PLUS,
                       arg_1: Box::new(Argument::Literal(2)),
                       arg_2: Box::new(Argument::Literal(4)),
                   });
    }

    #[test]
    fn should_parse_simple() {
        let parsed = PREC_CLIMBER_PART_1.parse("2 + 4");
        assert_eq!(parsed,
                   Argument::Expression {
                       operand: Operand::PLUS,
                       arg_1: Box::new(Argument::Literal(2)),
                       arg_2: Box::new(Argument::Literal(4)),
                   });
    }

    #[test]
    fn should_parse_sub_term() {
        let parsed = PREC_CLIMBER_PART_1.parse("2 + (4*2)");
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
        let parsed = PREC_CLIMBER_PART_1.parse("2 * 3 + (4 * 5)");
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