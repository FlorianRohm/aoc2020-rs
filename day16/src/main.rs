use crate::structs::{Rule, Ticket};

mod structs;

lazy_static::lazy_static! {
    static ref RULES: Vec<structs::Rule> = vec![
        Rule("departure location".into(), vec![47..=874, 885..=960]),
        Rule("departure station".into(), vec![25..=616, 622..=964]),
        Rule("departure platform".into(), vec![42..=807, 825..=966]),
        Rule("departure track".into(), vec![36..=560, 583..=965]),
        Rule("departure date".into(), vec![37..=264, 289..=968]),
        Rule("departure time".into(), vec![27..=325, 346..=954]),
        Rule("arrival location".into(), vec![37..=384, 391..=950]),
        Rule("arrival station".into(), vec![35..=233, 244..=963]),
        Rule("arrival platform".into(), vec![26..=652, 675..=949]),
        Rule("arrival track".into(), vec![41..=689, 710..=954]),
        Rule("class".into(), vec![27..=75, 81..=952]),
        Rule("duration".into(), vec![45..=784, 807..=967]),
        Rule("price".into(), vec![40..=350, 374..=970]),
        Rule("route".into(), vec![30..=892, 904..=968]),
        Rule("row".into(), vec![47..=144, 151..=957]),
        Rule("seat".into(), vec![28..=750, 773..=973]),
        Rule("train".into(), vec![30..=456, 475..=950]),
        Rule("type".into(), vec![34..=642, 648..=968]),
        Rule("wagon".into(), vec![42..=486, 498..=970]),
        Rule("zone".into(), vec![37..=152, 167..=973]),
    ];

    static ref OUR_TICKET: Ticket = Ticket(vec![83,137,101,73,67,61,103,131,151,127,113,107,109,89,71,139,167,97,59,53]);
}

fn main() {
    let input = include_str!("./input");
    let tickets: Vec<Ticket> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| structs::Ticket(line.split(',')
            .map(|num| num.parse().unwrap()).collect()))
        .collect();

    let invalid_sum: i32 = get_invalid_fields(&tickets, &RULES).iter().sum();

    println!("The sum of invalid fields is {}", invalid_sum);

    let valid_tickets = get_valid_tickets(tickets, &RULES);

    let vec = solve_rules(&RULES, &valid_tickets);

    let solution: i64 = vec.iter()
        .filter(|(rule, _)| rule.0.contains("departure"))
        .map(|(_, column)| dbg!(OUR_TICKET.0[*column]))
        .map(|a| a as i64)
        .product();

    println!("Solution to this day is {}", solution);
}

fn get_invalid_fields(tickets: &[Ticket], rules: &[Rule]) -> Vec<i32> {
    tickets.iter().flat_map(|ticket| ticket.get_invalid_fields(rules)).collect()
}

fn get_valid_tickets(tickets: Vec<Ticket>, rules: &[Rule]) -> Vec<Ticket> {
    tickets.into_iter().filter(|ticket| ticket.get_invalid_fields(rules).is_empty()).collect()
}

fn adheres_to_any_rule(field: i32, rules: &[Rule]) -> bool {
    rules.iter()
        .any(|rule| rule.value_adheres_to_rule(field))
}

impl Ticket {
    fn get_invalid_fields(&self, rules: &[Rule]) -> Vec<i32> {
        self.0.iter().filter(|&field| !adheres_to_any_rule(*field, rules)).map(|f| *f).collect()
    }
}

impl Rule {
    fn get_valid_columns(&self, tickets: &[Ticket]) -> Vec<usize> {
        let column_range = tickets[0].0.len();
        (0..column_range).filter(|&column| self.column_valid_in_tickets(tickets, column)).collect()
    }

    fn column_valid_in_tickets(&self, tickets: &[Ticket], column: usize) -> bool {
        tickets.iter().all(|ticket| self.value_adheres_to_rule(ticket.0[column]))
    }

    fn value_adheres_to_rule(&self, field: i32) -> bool {
        self.1.iter()
            .any(|range| range.contains(&field))
    }
}

fn solve_rules<'a>(rules: &'a [Rule], valid_tickets: &[Ticket]) -> Vec<(&'a Rule, usize)> {
    let mut still_to_solve: Vec<(&Rule, Vec<usize>)> = rules.iter().map(|rule| (rule, rule.get_valid_columns(&valid_tickets))).collect();

    let mut solution = vec![];

    while let Some(new_solution) = get_next_single_rule(&mut still_to_solve) {
        solution.push(new_solution);
    }

    assert_eq!(still_to_solve.len(), 0, "there are unsolved rules. New strategy needed");
    solution
}

fn get_next_single_rule<'a>(input: &mut Vec<(&'a Rule, Vec<usize>)>) -> Option<(&'a Rule, usize)> {
    let option = input.iter().position(|(_rule, size)| size.len() == 1);
    return match option {
        None => None,
        Some(pos) => {
            let (rule, col_vec) = input.remove(pos);
            let found_solution = *col_vec.iter().next().unwrap();

            input.iter_mut().for_each(|(_rule, columns)| columns.retain(|d| *d != found_solution));
            Some((rule, found_solution))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_test() {
        let rules: Vec<structs::Rule> = vec![
            Rule("class".into(), vec![1..=3, 5..=7]),
            Rule("row".into(), vec![6..=11, 33..=44]),
            Rule("seat".into(), vec![13..=40, 45..=50]),
        ];
        let tickets: Vec<Ticket> = vec![
            Ticket(vec![7, 3, 47]),
            Ticket(vec![40, 4, 50]),
            Ticket(vec![55, 2, 20]),
            Ticket(vec![38, 6, 12]),
        ];
        let invalids = get_invalid_fields(&tickets, &rules);

        assert_eq!(invalids, vec![4, 55, 12])
    }

    #[test]
    fn should_compute_test_v2() {
        let rules: Vec<structs::Rule> = vec![
            Rule("class".into(), vec![0..=1, 4..=19]),
            Rule("row".into(), vec![0..=5, 8..=19]),
            Rule("seat".into(), vec![0..=13, 16..=19]),
        ];
        let tickets: Vec<Ticket> = vec![
            Ticket(vec![3, 9, 18]),
            Ticket(vec![15, 1, 5]),
            Ticket(vec![5, 14, 9])
        ];
        for rule in rules {
            let valid_columns = rule.get_valid_columns(&tickets);
            println!("{:?}", valid_columns);
        }
    }
}