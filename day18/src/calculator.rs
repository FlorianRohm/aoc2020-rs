#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operand {
    PLUS,
    TIMES,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Argument {
    Expression {
        arg_1: Box<Argument>,
        operand: Operand,
        arg_2: Box<Argument>,
    },
    Literal(i64),
}

impl Argument {
    pub fn eval(self) -> i64 {
        match self {
            Argument::Expression { arg_1, arg_2, operand } => eval_arg(arg_1, operand, arg_2),
            Argument::Literal(i) => i
        }
    }
}

pub fn eval_arg(arg_1: Box<Argument>,
                operand: Operand,
                arg_2: Box<Argument>) -> i64 {
    use Operand::*;
    match (arg_1.eval(), operand, arg_2.eval()) {
        (a, PLUS, b) => a + b,
        (a, TIMES, b) => a * b,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_case() {
        assert_eq!(6, Argument::Expression {
            operand: Operand::PLUS,
            arg_1: Box::new(Argument::Literal(2)),
            arg_2: Box::new(Argument::Literal(4)),
        }.eval());

        assert_eq!(8, Argument::Expression {
            operand: Operand::TIMES,
            arg_1: Box::new(Argument::Literal(2)),
            arg_2: Box::new(Argument::Literal(4)),
        }.eval());
    }

    #[test]
    fn recursion() {
        assert_eq!(12, Argument::Expression {
            operand: Operand::PLUS,
            arg_1: Box::new(Argument::Expression {
                operand: Operand::TIMES,
                arg_1: Box::new(Argument::Literal(2)),
                arg_2: Box::new(Argument::Literal(4)),
            }),
            arg_2: Box::new(Argument::Literal(4)),
        }.eval());
    }
}
