use crate::assembly::Assembly;
use crate::op::{Op, Op::*};

pub fn find_permuted_assembly(input: &Vec<Op>) -> Option<(i32, usize)> {
    for i in 0..input.len() {
        if let Some(working_copy) = get_working_copy(input, i) {
            let (output, success) = Assembly::new(&working_copy).run();
            if success {
                return Some((output, i));
            }
        }
    }

    None
}

fn get_working_copy(input: &Vec<Op>, i: usize) -> Option<Vec<Op>> {
    let op_to_change = input.get(i).unwrap();
    let new_op = match op_to_change {
        NoOp(num) => Jmp(*num),
        Acc(_) => return None,
        Jmp(num) => NoOp(*num),
    };
    let mut working_copy = input.clone();

    let x = working_copy.get_mut(i).unwrap();
    *x = new_op;

    Some(working_copy)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let (i, index) = find_permuted_assembly(&input).expect("test should compute");

        assert_eq!(i, 8);
        assert_eq!(index, 7);
    }
}