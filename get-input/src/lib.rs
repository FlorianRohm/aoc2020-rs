use std::fs;

pub fn get_raw_input(day: i32, section:i32) -> String {
    fs::read_to_string(format!("get-input/resources/day{}_{}", day, section))
        .expect("Something went wrong reading the file")
}
