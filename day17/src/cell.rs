use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Copy, Clone)]
pub enum Cell {
    #[display("#")]
    Active,
    #[display(".")]
    Inactive,
}
