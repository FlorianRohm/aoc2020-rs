use crate::definitions::{BitmaskData, Input, MemoryData};

lazy_static::lazy_static! {
    static ref MATCHER: regex::Regex = regex::Regex::new("^mem\\[(\\d*)\\] = (\\d*)$").unwrap();
}

pub fn parse_line(input: &str) -> Input {
    if input.starts_with("mask = ") {
        let data = (&input[7..]).chars().map(|c| match c {
            'X' | 'x' => BitmaskData::Ignore,
            '1' => BitmaskData::NewValue('1'),
            '0' => BitmaskData::NewValue('0'),
            _ => unreachable!()
        }).collect();
        Input::Bitmask(data)
    } else {
        let memory_match = MATCHER.captures(input).expect(&format!("input '{}' does not match", input));

        let location: i64 = memory_match[1].parse().unwrap();
        let value: i64 = memory_match[2].parse().unwrap();
        Input::Memory(MemoryData { location, value })
    }
}
