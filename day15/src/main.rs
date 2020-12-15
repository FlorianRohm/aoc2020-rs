use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = vec![7, 12, 1, 0, 16, 2];

    println!("the last spoken number is {}", speak_n_times(input, 2020));
    let input = vec![7, 12, 1, 0, 16, 2];

    let now = Instant::now();
    println!("the last spoken number is {}", speak_n_times(input, 30000000));
    println!("{}", now.elapsed().as_millis());
}

fn speak_n_times(input: Vec<i32>, n: usize) -> i32 {
    let (mut speaker, mut status) = Speaker::new(vec![7, 12, 1, 0, 16, 2]);
    for _ in input.len()..n {
        status = speaker.speak_next_number(status);
    }

    match status {
        Status::New(i) => i,
        Status::Old(i) => i
    }
}

struct Speaker {
    store: HashMap<i32, i32>,
    numbers_spoken: i32,
}

#[derive(Eq, PartialEq, Debug)]
enum Status {
    New(i32),
    Old(i32),
}

impl Speaker {
    fn speak_next_number(&mut self, last_spoken: Status) -> Status {
        let new_status: Status = match last_spoken {
            Status::New(entry) => {
                self.store.insert(entry, self.numbers_spoken - 1);

                Status::Old(0)
            }
            Status::Old(entry) => {
                let spoken_before_index = *self.store.get(&entry).unwrap();

                self.store.insert(entry, self.numbers_spoken - 1);
                let new_index = self.numbers_spoken - spoken_before_index - 1;

                match self.store.contains_key(&new_index) {
                    true => Status::Old(new_index),
                    false => Status::New(new_index)
                }
            }
        };
        self.numbers_spoken += 1;
        new_status
    }

    fn new(input: Vec<i32>) -> (Self, Status) {
        let mut store = HashMap::new();
        let mut last_spoken = 0;
        for (index, input) in input.iter().enumerate() {
            store.insert(*input, index as i32);
            last_spoken = *input;
        }
        let numbers_spoken = store.values().len() as i32;
        (Self {
            store,
            numbers_spoken,
        }, Status::New(last_spoken))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_speak_next_number() {
        let input = vec![0, 3, 6];
        let (mut speaker, status) = Speaker::new(input);

        let spoken = speaker.speak_next_number(status);
        assert_eq!(spoken, Status::Old(0));

        let spoken = speaker.speak_next_number(spoken);
        assert_eq!(spoken, Status::Old(3));

        let spoken = speaker.speak_next_number(spoken);
        assert_eq!(spoken, Status::Old(3));

        let spoken = speaker.speak_next_number(spoken);
        assert_eq!(spoken, Status::New(1));
    }

    #[test]
    fn should_speak_2020_times_in() {
        let input = vec![7, 12, 1, 0, 16, 2];
        let i = speak_n_times(input, 2020);

        assert_eq!(i, 410)
    }
}