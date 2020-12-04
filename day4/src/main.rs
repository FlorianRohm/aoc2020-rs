#[macro_use]
extern crate lazy_static;

use get_input::get_raw_input;

mod lib;

fn main() {
    let input = get_raw_input(4, 1);

    let valid_passports = lib::get_lenient_valid_pws_from_input(&input);
    println!("There are {} valid passports", valid_passports.count());
    let strict_valid_passports = lib::get_strict_valid_pws_from_input(&input);
    println!("There are {} strict valid passports", strict_valid_passports.count())
}
