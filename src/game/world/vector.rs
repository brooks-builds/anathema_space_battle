use std::ops::Sub;

use anathema::geometry::LocalPos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone, Copy, Deserialize, PartialEq)]
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

    pub fn all_around(&self) -> Vec<Self> {
        vec![
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x + 1, self.y),
            Self::new(self.x + 1, self.y + 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x - 1, self.y - 1),
        ]
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

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn should_create_vectors_around() {
        let center = Vector::new(5, 5);
        let expected = vec![
            Vector::new(5, 4),
            Vector::new(6, 4),
            Vector::new(6, 5),
            Vector::new(6, 6),
            Vector::new(5, 6),
            Vector::new(4, 6),
            Vector::new(4, 5),
            Vector::new(4, 4),
        ];
        let result = center.all_around();

        assert_eq!(result, expected);
    }
}
