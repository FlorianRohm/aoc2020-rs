use crate::assembly::Assembly;

mod assembly;
mod parser;
mod op;
mod permute;

fn main() {
    let input = include_str!("./input");
    let input = parser::parse_program(input);
    let (i, bool) = Assembly::new(&input).run();

    println!("The program starts to repeat itself with a value of {}", i);

    let (i, at) = permute::find_permuted_assembly(&input).expect("no valid input found");

    println!("The program does not repeat if one switches {} and then produces {}", at, i)
}
