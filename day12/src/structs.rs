#[derive(Copy, Clone)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

pub enum Instruction {
    Translation((Heading, i64)),
    Right(i64),
    Left(i64),
    Forward(i64),
}
