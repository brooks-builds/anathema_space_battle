use std::ops::Sub;

use anathema::geometry::LocalPos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone, Copy, Deserialize)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Self) -> i32 {
        let difference = self - other;
        let mag = difference.x.pow(2) + difference.y.pow(2);

        mag.isqrt()
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Vector> for LocalPos {
    fn from(val: Vector) -> Self {
        LocalPos {
            x: val.x as u16,
            y: val.y as u16,
        }
    }
}
