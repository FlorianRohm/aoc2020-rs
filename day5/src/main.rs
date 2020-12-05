use itertools::Itertools;

fn main() {
    let input = include_str!("./input");

    let mut seat_ids: Vec<usize> = input.lines().filter(|line| line.len() == 10)
        .map(parse_seat)
        .map(|seat| seat.get_seat_id()).collect();

    let maximum_id = seat_ids.iter()
        .max().unwrap_or(&0);

    println!("The biggest id is {}", maximum_id);

    seat_ids.sort();
    let our_seat = seat_ids.iter().tuple_windows()
        .find(|(a, b)| **a + 2 == **b)
        .map(|(a, _b)| *a + 1)
        .expect("No fitting seat found");

    println!("Our Seat id is {}", our_seat);
}


struct Seat {
    row: usize,
    seat: usize,
}


impl Seat {
    fn get_seat_id(&self) -> usize {
        self.row * 8 + self.seat
    }
}

fn parse_seat(input: &str) -> Seat {
    let (row, seat) = input.split_at(7);

    let row = row.replace('B', "1").replace('F', "0");
    let seat = seat.replace('R', "1").replace('L', "0");

    let row = usize::from_str_radix(&row, 2).expect(&format!("String {} is no binary", row));
    let seat = usize::from_str_radix(&seat, 2).expect(&format!("String {} is no binary", seat));

    Seat { row, seat }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_translate_test_input_1() {
        let id = parse_seat("BFFFBBFRRR").get_seat_id();

        assert_eq!(id, 567)
    }

    #[test]
    fn should_translate_test_input_2() {
        let id = parse_seat("FFFBBBFRRR").get_seat_id();

        assert_eq!(id, 119)
    }

    #[test]
    fn should_translate_test_input_3() {
        let id = parse_seat("BBFFBBFRLL").get_seat_id();

        assert_eq!(id, 820)
    }
}