#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Op {
    NoOp(i32),
    Acc(i32),
    Jmp(i32),
}