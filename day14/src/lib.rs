use std::collections::HashMap;

use crate::definitions::{BitmaskData, Input, MemoryData};

mod parse;
mod definitions;

pub fn solve() -> (i64, i64) {
    let input = include_str!("./input");
    let state = solve_v1(input);

    let sum_1: i64 = state.data.values().sum();

    println!("the sum of all instructions v1 is {}", sum_1);
    let state = solve_v2(input);

    let sum_2: i64 = state.data.values().sum();

    println!("the sum of all instructions v2 is {}", sum_2);
    (sum_1, sum_2)
}

fn solve_v1(input: &str) -> State {
    let instructions: Vec<Input> = input.lines().filter(|line| !line.is_empty()).map(parse::parse_line).collect();

    let mut state = State::default();
    for instruction in instructions {
        state.update_v1(instruction);
    }
    state
}

fn solve_v2(input: &str) -> State {
    let instructions: Vec<Input> = input.lines().filter(|line| !line.is_empty()).map(parse::parse_line).collect();

    let mut state = State::default();
    for instruction in instructions {
        state.update_v2(instruction);
    }
    state
}

#[derive(Default, Debug)]
struct State {
    current_bitmask: Bitmask,
    data: HashMap<i64, i64>,
}

type Bitmask = Vec<BitmaskData>;

impl State {
    fn update_v2(&mut self, instruction: Input) {
        match instruction {
            Input::Bitmask(bitmask) => self.current_bitmask = bitmask,
            Input::Memory(update) => self.update_memory_at_locations(update)
        }
    }

    fn update_v1(&mut self, instruction: Input) {
        match instruction {
            Input::Bitmask(bitmask) => self.current_bitmask = bitmask,
            Input::Memory(update) => self.update_memory(update)
        }
    }

    fn update_memory_at_locations(&mut self, new_memory: MemoryData) {
        let radix_string = format!("{}", radix_fmt::radix(new_memory.location, 2));
        let location = format!("{:0>36}", radix_string);

        debug_assert_eq!(self.current_bitmask.len(), 36);

        let new_mask: Bitmask = location.chars().into_iter().zip(self.current_bitmask.iter()).map(|(char, bitmask_data)| {
            match bitmask_data {
                BitmaskData::NewValue(new) if *new == '0' => BitmaskData::NewValue(char),
                BitmaskData::NewValue(new) if *new == '1' => BitmaskData::NewValue(*new),
                BitmaskData::Ignore => BitmaskData::Ignore,
                _ => unreachable!()
            }
        }).collect();

        let all_masks = State::explode_recursion(vec![new_mask]);
        all_masks.iter()
            .map(|mask| {
                let x: String = mask.iter()
                    .map(|data| match data {
                        BitmaskData::NewValue(c) => *c,
                        BitmaskData::Ignore => unreachable!()
                    }).collect();
                x
            })
            .map(|string| i64::from_str_radix(&string, 2).unwrap())
            .for_each(|address| { self.data.insert(address, new_memory.value); });
    }

    fn explode_address(mut input: Bitmask) -> Vec<Bitmask> {
        let has_ignore = input.iter().position(|p| *p == BitmaskData::Ignore);
        match has_ignore {
            None => vec![input],
            Some(location) => {
                let mut vec = input.clone();
                *vec.get_mut(location).unwrap() = BitmaskData::NewValue('0');
                *input.get_mut(location).unwrap() = BitmaskData::NewValue('1');

                vec![input, vec]
            }
        }
    }

    fn explode_recursion(input: Vec<Bitmask>) -> Vec<Bitmask> {
        let input_len = input.len();
        let new_vec: Vec<Bitmask> = input.into_iter().flat_map(State::explode_address).collect();
        if input_len == new_vec.len() {
            new_vec
        } else {
            State::explode_recursion(new_vec)
        }
    }

    fn update_memory(&mut self, new_memory: MemoryData) {
        let radix_string = format!("{}", radix_fmt::radix(new_memory.value, 2));
        let string = format!("{:0>36}", radix_string);

        debug_assert_eq!(self.current_bitmask.len(), 36);

        let new_string: String = string.chars().into_iter().zip(self.current_bitmask.iter()).map(|(char, bitmask_data)| {
            match bitmask_data {
                BitmaskData::NewValue(new) => *new,
                BitmaskData::Ignore => char
            }
        }).collect();

        let new_value = i64::from_str_radix(&new_string, 2).unwrap();

        self.data.insert(new_memory.location, new_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (15403588588538, 3260587250457));
    }

    #[test]
    fn should_solve_test() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let state = solve_v1(input);

        let mut solution = HashMap::new();
        solution.insert(7i64, 101i64);
        solution.insert(8, 64);

        assert_eq!(state.data, solution)
    }

    #[test]
    fn should_solve_test_2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let state = solve_v2(input);

        let values_sum: i64 = state.data.values().into_iter().sum();

        assert_eq!(values_sum, 208)
    }
}