use std::ops::RangeInclusive;

pub struct Rule(pub String, pub Vec<RangeInclusive<i32>>);

pub struct Ticket(pub Vec<i32>);