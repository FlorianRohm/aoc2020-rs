use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    IResult,
    sequence::tuple,
};
use nom::bytes::complete::take_while;
use nom::character::{is_digit, is_space};
use nom::character::complete::{anychar, char, one_of};
use nom::combinator::recognize;
use nom::error::{Error, ParseError};
use nom::lib::std::str::FromStr;
use nom::multi::{many0, many1};
use nom::sequence::terminated;

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy<'a> {
    pub min: usize,
    pub max: usize,
    pub occurrence: char,
    pub password: &'a str,
}

fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(|a| is_space(a as u8))(i)
}


fn parse_password(input: &str) -> IResult<&str, PasswordPolicy> {
    use nom::character::{is_space, is_alphanumeric};
    use nom::number::complete::u8;
    use std::str::FromStr;

    let (input, min) = map_res(take_while(|a| is_digit(a as u8)), usize::from_str)(input)?;

    let (input, _) = tuple((space, tag("-"), space))(input)?;
    let (input, max) = map_res(take_while(|a| is_digit(a as u8)), usize::from_str)(input)?;
    let (input, _) = space(input)?;
    let (input, occurrence) = anychar(input)?;
    let (input, _) = tuple((space, tag(":"), space))(input)?;
    let (input, password) = take_while(|a| is_alphanumeric(a as u8))(input)?;

    Ok((input, PasswordPolicy { min, max, occurrence, password }))
}

pub fn parse_pw_unwrap(input: &str) -> PasswordPolicy {
    match parse_password(input) {
        Ok((_remainder, policy)) => { policy }
        Err(_) => { panic!("could not parse {}", input) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_pw() {
        assert_eq!(parse_password("12-34 b: cdefg"), Ok(("", PasswordPolicy {
            min: 12,
            max: 34,
            occurrence: 'b',
            password: "cdefg",
        })));
    }
}