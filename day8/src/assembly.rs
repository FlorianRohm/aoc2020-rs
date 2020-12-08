use crate::op::Op;

type EndedNominally = bool;
type AlreadyVisited = bool;

pub struct Assembly {
    instructions: Vec<(Op, AlreadyVisited)>,
    pointer: usize,
    accumulator: i32,
}

impl Assembly {
    pub fn new(input: &Vec<Op>) -> Assembly {
        Assembly {
            instructions: input.iter().map(|op| (op.clone(), false)).collect(),
            pointer: 0,
            accumulator: 0,
        }
    }

    pub fn run(&mut self) -> (i32, EndedNominally) {
        loop {
            if self.pointer >= self.instructions.len() {
                return (self.accumulator, true);
            }
            let (current_instruction, was_already_hit) = self.instructions.get_mut(self.pointer).expect("We left the band");

            if *was_already_hit { return (self.accumulator, false); }

            *was_already_hit = true;

            match current_instruction {
                Op::NoOp(_) => {
                    self.pointer += 1
                }
                Op::Acc(num) => {
                    self.accumulator += *num;
                    self.pointer += 1;
                }
                Op::Jmp(num) => {
                    self.pointer = (self.pointer as i32 + *num) as usize
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Op::*;

    #[test]
    fn should_compute_test_input() {
        let input = vec![
            NoOp(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6)
        ];
        let (i, nominal) = Assembly::new(&input).run();

        assert_eq!(i, 5);
        assert_eq!(nominal, false);
    }
}