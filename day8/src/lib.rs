use crate::assembly::Assembly;

mod assembly;
mod parser;
mod op;
mod permute;

pub fn solve() -> (i32, (usize, i32)) {
    let input = include_str!("./input");
    let input = parser::parse_program(input);
    let (i, _) = Assembly::new(&input).run();

    println!("The program starts to repeat itself with a value of {}", i);

    let (i2, at) = permute::find_permuted_assembly(&input).expect("no valid input found");

    println!("The program does not repeat if one switches {} and then produces {}", at, i2);
    (i, (at, i2))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (11563, (498, 767)));
    }
}