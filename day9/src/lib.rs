mod xmas;

pub fn solve() -> ((usize, i64), i64) {
    let input = include_str!("./input");
    let input: Vec<i64> = input.lines().filter(|line| !line.is_empty()).map(|l| l.parse::<i64>().unwrap()).collect();

    let code = xmas::XmasCode { preamble_length: 25, code: input };

    let (index, corrupt_data) = code.find_corrupt_data().expect("code not corrupt");
    println!("the corrupt data is {} at {}", corrupt_data, index);

    let x = code.find_encryption_weakness(index, corrupt_data).expect("no weakness found");

    println!("the weakness is {}", x);

    ((index, corrupt_data), x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), ((548, 57195069), 7409241));
    }
}
