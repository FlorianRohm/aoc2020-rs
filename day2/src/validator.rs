use std::ops::BitXor;

use crate::parser::PasswordPolicy;

pub fn is_valid(pw: &PasswordPolicy) -> bool {
    let count = pw.password.chars().into_iter().filter(|c| pw.occurrence == *c).count();

    count >= pw.min && count <= pw.max
}


pub fn is_valid_new(pw: &PasswordPolicy) -> bool {
    let char1 = pw.password.chars().nth(pw.min - 1);
    let char2 = pw.password.chars().nth(pw.max - 1);

    let found1 = char1.map(|c| c == pw.occurrence).unwrap_or(false);
    let found2 = char2.map(|c| c == pw.occurrence).unwrap_or(false);

    found1.bitxor(found2)
}
