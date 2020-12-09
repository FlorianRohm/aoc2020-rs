pub struct XmasCode {
    pub preamble_length: i64,
    pub code: Vec<i64>,
}

impl XmasCode {
    pub fn find_corrupt_data(&self) -> Option<(usize, i64)> {
        let slice = &self.code[self.preamble_length as usize..];
        for (i, wanted) in slice.iter().enumerate() {
            let end = i + self.preamble_length as usize;
            match get_valid_combination(&self.code[i..end], *wanted) {
                None => return Some((end, *wanted)),
                Some(_) => continue
            }
        }
        None
    }

    pub fn find_weak_slice(&self, corrupt_location: usize, corrupt_value: i64) -> Option<&[i64]> {
        let start = corrupt_location - 1;

        for start_index in (0..start).rev() {
            match get_summation_slice(&self.code[0..start_index], corrupt_value) {
                None => {}
                Some(slice) => return Some(slice)
            }
        }

        None
    }

    pub fn find_encryption_weakness(&self, corrupt_location: usize, corrupt_value: i64) -> Option<i64> {
        self.find_weak_slice(corrupt_location, corrupt_value).map(|weak_slice| {
            weak_slice.iter().max().expect("empty vec") + weak_slice.iter().min().expect("empty vec")
        })
    }
}

fn get_summation_slice(input: &[i64], wanted_sum: i64) -> Option<&[i64]> {
    let mut acc = wanted_sum;
    for (index, x) in input.iter().enumerate().rev() {
        acc -= x;
        match acc {
            _ if acc == 0 => return Some(&input[index..]),
            _ if acc < 0 => return None,
            _ => continue
        }
    }

    None
}

fn get_valid_combination(input: &[i64], wanted_sum: i64) -> Option<(i64, i64)> {
    use itertools::Itertools;
    input.iter().enumerate().cartesian_product(input.iter().enumerate())
        .find(|((i1, &line1), (i2, &line2))| i1 != i2 && line1 + line2 == wanted_sum)
        .map(|((_, l1), (_, l2))| (*l1, *l2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_data_in_ok_slice() {
        let code = vec![35, 20, 15, 25, 47];

        let check = get_valid_combination(&code, 40).unwrap();

        assert_eq!(check, (15, 25))
    }

    #[test]
    fn should_check_data_in_corrupt_slice() {
        let code = vec![95, 102, 117, 150, 182];

        let check = get_valid_combination(&code, 127);

        assert_eq!(check, None)
    }

    #[test]
    fn should_find_corrupt_data_in_test() {
        let code = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        let xmas_code = XmasCode { code, preamble_length: 5 };

        let corrupt_data = xmas_code.find_corrupt_data().expect("test has corrupt data");

        assert_eq!(corrupt_data, (14, 127))
    }

    #[test]
    fn should_find_weakness_in_code() {
        let code = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        let xmas_code = XmasCode { code, preamble_length: 5 };

        let weakness = xmas_code.find_weak_slice(14, 127).unwrap();

        let vec1 = vec![15_i64, 25, 47, 40];
        assert_eq!(weakness, vec1.as_slice());

        let weakness = xmas_code.find_encryption_weakness(14, 127).unwrap();
        assert_eq!(weakness, 62);
    }
}