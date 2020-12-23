use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub i64, pub i64);

impl Mul<i64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i64) -> Self::Output {
        let Vector(x, y) = self;
        Vector(x * rhs, y * rhs)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = rhs;

        Vector(x1 + x2, y1 + y2)
    }
}
