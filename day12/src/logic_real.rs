use crate::structs::{Heading, Instruction};

pub struct ShipState {
    loc: Location,
    waypoint: Location,
}

pub struct Location {
    loc_east: i64,
    loc_north: i64,
}

impl Location {
    pub fn rotate(&mut self, amount_right: i64) {
        match amount_right {
            0 => {}
            90 => {
                std::mem::swap(&mut self.loc_north, &mut self.loc_east);
                self.loc_north *= -1;
            }
            180 => {
                self.loc_east *= -1;
                self.loc_north *= -1;
            }
            270 => {
                self.rotate(90);
                self.rotate(180);
            }
            _ => panic!(format!("amount {} not supported", amount_right))
        }
    }
}

impl ShipState {
    pub fn new() -> Self {
        ShipState {
            loc: Location { loc_east: 0, loc_north: 0 },
            waypoint: Location { loc_east: 10, loc_north: 1 },
        }
    }

    pub fn get_distance(&self) -> i64 {
        self.loc.loc_north.abs() + self.loc.loc_east.abs()
    }

    pub fn move_to_waypoint(&mut self) {
        self.loc.loc_north += self.waypoint.loc_north;
        self.loc.loc_east += self.waypoint.loc_east;
    }

    pub fn execute_instruction(mut self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Translation((heading, value)) => match heading {
                Heading::North => self.waypoint.loc_north += value,
                Heading::South => self.waypoint.loc_north -= value,
                Heading::East => self.waypoint.loc_east += value,
                Heading::West => self.waypoint.loc_east -= value,
            }
            Instruction::Right(value) => self.waypoint.rotate(value),
            Instruction::Left(value) => self.waypoint.rotate(360 - value),
            Instruction::Forward(value) => (0..value).for_each(|_| self.move_to_waypoint()),
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

        assert_eq!(state.get_distance(), 286)
    }
}