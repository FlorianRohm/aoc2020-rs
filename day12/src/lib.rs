use crate::structs::Instruction;

mod logic_old;
mod logic_real;
mod structs;

pub fn solve() -> (i64, i64) {
    let input = include_str!("./input");
    let instructions: Vec<Instruction> = input.lines().filter(|line| !line.is_empty()).map(parse_line).collect();

    let state = logic_old::ShipState::new().execute_all_instructions(instructions);

    let distance = state.get_distance();
    println!("our ship is {} away from the start", distance);

    let instructions: Vec<Instruction> = input.lines().filter(|line| !line.is_empty()).map(parse_line).collect();

    let state = logic_real::ShipState::new().execute_all_instructions(instructions);

    let real_distance = state.get_distance();
    println!("our ship is really {} away from the start", real_distance);

    (distance, real_distance)
}


fn parse_line(input: &str) -> structs::Instruction {
    use structs::Instruction::*;
    use structs::Heading::*;
    let (dir, amount) = input.split_at(1);
    let amount: i64 = amount.parse().expect(&format!("could not parse '{}'", amount));

    match dir {
        "N" => Translation((North, amount)),
        "S" => Translation((South, amount)),
        "E" => Translation((East, amount)),
        "W" => Translation((West, amount)),
        "L" => Left(amount),
        "R" => Right(amount),
        "F" => Forward(amount),
        _ => panic!(format!("dir {} not supported", dir))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (1007, 41212));
    }
}