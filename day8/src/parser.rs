use crate::op::Op::{Acc, Jmp, NoOp};

pub fn parse_program(input: &str) -> Vec<crate::op::Op> {
    input.lines().filter(|line| !line.is_empty()).map(parse_line).collect()
}

fn parse_line(input: &str) -> crate::op::Op {
    let splitted: Vec<&str> = input.split_whitespace().collect();
    match splitted[..] {
        ["nop", num] => NoOp(num.parse().expect(&format!("could not parse {} for acc", num))),
        ["acc", num] => Acc(num.parse().expect(&format!("could not parse {} for acc", num))),
        ["jmp", num] => Jmp(num.parse().expect(&format!("could not parse {} for jmp", num))),
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::op::Op::{Acc, Jmp, NoOp};

    use super::*;

    #[test]
    fn should_parse_test_input_day7() {
        let input = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let vec = parse_program(input);

        assert_eq!(vec, vec![
            NoOp(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6)
        ])
    }
}