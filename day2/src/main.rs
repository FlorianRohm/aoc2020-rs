use crate::parser::PasswordPolicy;

mod parser;
mod validator;

fn main() {
    let input = get_input::get_raw_input(2);
    let policies: Vec<PasswordPolicy> = input.lines()
        .map(parser::parse_pw_unwrap).collect();
    let valid_pws = policies.iter()
        .filter(|&pw| validator::is_valid(pw))
        .count();

    let valid_pws_new = policies.iter()
        .filter(|&pw| validator::is_valid_new(pw))
        .count();

    println!("there are {} valid passwords", valid_pws);
    println!("there are {} valid passwords new", valid_pws_new);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_provided() {
        let input1 = "1-3 a: abcde";
        let input2 = "1-3 b: cdefg";
        let input3 = "2-9 c: ccccccccc";

        let policy1 = parser::parse_pw_unwrap(input1);
        let policy2 = parser::parse_pw_unwrap(input2);
        let policy3 = parser::parse_pw_unwrap(input3);

        assert_eq!(validator::is_valid(&policy1), true);
        assert_eq!(validator::is_valid(&policy2), false);
        assert_eq!(validator::is_valid(&policy3), true);
    }

    #[test]
    fn check_provided_2() {
        let input1 = "1-3 a: abcde";
        let input2 = "1-3 b: cdefg";
        let input3 = "2-9 c: ccccccccc";

        let policy1 = parser::parse_pw_unwrap(input1);
        let policy2 = parser::parse_pw_unwrap(input2);
        let policy3 = parser::parse_pw_unwrap(input3);

        assert_eq!(validator::is_valid_new(&policy1), true);
        assert_eq!(validator::is_valid_new(&policy2), false);
        assert_eq!(validator::is_valid_new(&policy3), false);
    }
}