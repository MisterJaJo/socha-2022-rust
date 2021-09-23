use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

pub type Coordinates = Vec2<i32>;

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Self) -> Self::Output {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, other: Self) -> Self::Output {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
