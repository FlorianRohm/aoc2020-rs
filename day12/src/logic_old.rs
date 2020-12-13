use crate::structs::{Heading, Instruction};

impl Heading {
    fn cycle_to_next(&mut self, amount_right: i64, next_headings: Vec<Heading>) {
        match amount_right {
            0 => {}
            90 => *self = next_headings[0],
            180 => *self = next_headings[1],
            270 => *self = next_headings[2],
            _ => panic!(format!("amount {} not supported", amount_right))
        }
    }
    pub fn cycle(&mut self, amount_right: i64) {
        use Heading::*;
        match self {
            Heading::North => self.cycle_to_next(amount_right, vec![East, South, West]),
            Heading::East => self.cycle_to_next(amount_right, vec![South, West, North]),
            Heading::South => self.cycle_to_next(amount_right, vec![West, North, East]),
            Heading::West => self.cycle_to_next(amount_right, vec![North, East, South]),
        }
    }
}

pub struct ShipState {
    loc_east: i64,
    loc_north: i64,
    heading: Heading,
}

impl ShipState {
    pub fn new() -> Self {
        ShipState {
            loc_east: 0,
            loc_north: 0,
            heading: Heading::East,
        }
    }
    pub fn get_distance(&self) -> i64 {
        self.loc_north.abs() + self.loc_east.abs()
    }

    pub fn execute_instruction(mut self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Translation((heading, value)) => match heading {
                Heading::North => self.loc_north += value,
                Heading::South => self.loc_north -= value,
                Heading::East => self.loc_east += value,
                Heading::West => self.loc_east -= value,
            }
            Instruction::Right(value) => self.heading.cycle(value),
            Instruction::Left(value) => self.heading.cycle(360 - value),
            Instruction::Forward(value) => match self.heading {
                Heading::North => self.loc_north += value,
                Heading::South => self.loc_north -= value,
                Heading::East => self.loc_east += value,
                Heading::West => self.loc_east -= value,
            }
        }

        self
    }
    pub fn execute_all_instructions(self, instructions: Vec<Instruction>) -> Self {
        let mut state = self;
        for instruction in instructions {
            state = state.execute_instruction(instruction);
        }
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_test_input() {
        use Instruction::*;
        let instructions = vec![Forward(10), Translation((Heading::North, 3)), Forward(7), Right(90), Forward(11)];

        let state = ShipState::new().execute_all_instructions(instructions);

        assert_eq!(state.get_distance(), 25)
    }
}