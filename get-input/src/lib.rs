use std::fs;

pub fn get_raw_input(day: i32) -> String {
    fs::read_to_string(format!("get-input/resources/day{}", day))
        .expect("Something went wrong reading the file")
}
