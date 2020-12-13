use crate::structs::Instruction;

mod logic_old;
mod logic_real;
mod structs;

fn main() {
    let input = include_str!("./input");
    let instructions: Vec<Instruction> = input.lines().filter(|line| !line.is_empty()).map(parse_line).collect();

    let state = logic_old::ShipState::new().execute_all_instructions(instructions);

    println!("our ship is {} away from the start", state.get_distance());

    let instructions: Vec<Instruction> = input.lines().filter(|line| !line.is_empty()).map(parse_line).collect();

    let state = logic_real::ShipState::new().execute_all_instructions(instructions);

    println!("our ship is really {} away from the start", state.get_distance());
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