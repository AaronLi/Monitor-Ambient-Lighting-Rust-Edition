use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub enum Side{
    TOP,
    LEFT,
    BOTTOM,
    RIGHT,
    ERROR
}

impl From<char> for Side{
    fn from(c: char) -> Self {
        match c{
            't' => Side::TOP,
            'b' => Side::BOTTOM,
            'l' => Side::LEFT,
            'r' => Side::RIGHT,
            _ => Side::ERROR
        }
    }
}

pub struct SideDirection{
    pub side: Side,
    pub direction: Side
}

impl Display for Side{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Side::TOP => {"Top"},
                Side::LEFT => {"Left"},
                Side::BOTTOM => {"Bottom"},
                Side::RIGHT => {"Right"},
                Side::ERROR => {"Error"}
            }
        )
    }
}