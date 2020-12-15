#[derive(Debug)]
pub enum Input {
    Bitmask(Vec<BitmaskData>),
    Memory(MemoryData),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BitmaskData {
    NewValue(char),
    Ignore,
}

#[derive(Debug)]
pub struct MemoryData {
    pub location: i64,
    pub value: i64,
}