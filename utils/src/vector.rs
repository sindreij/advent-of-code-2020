use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector(pub i64, pub i64);

fn sini(deg: i64) -> i64 {
    if deg.abs() == 90 {
        deg.signum()
    } else if deg.abs() == 180 {
        0
    } else if deg == 0 {
        0
    } else if deg.abs() == 360 {
        0
    } else if deg.abs() == 270 {
        -deg.signum()
    } else {
        panic!("Cannot do integer-sin of deg {}", deg)
    }
}

fn cosi(deg: i64) -> i64 {
    if deg.abs() == 90 {
        0
    } else if deg.abs() == 180 {
        -1
    } else if deg == 0 {
        1
    } else if deg.abs() == 360 {
        1
    } else if deg.abs() == 270 {
        0
    } else {
        panic!("Cannot do integer-cos of deg {}", deg)
    }
}

impl Vector {
    pub const NORTH: Self = Vector(0, -1);
    pub const WEST: Self = Vector(-1, 0);
    pub const EAST: Self = Vector(1, 0);
    pub const SOUTH: Self = Vector(0, 1);

    pub fn rotate(&self, deg: i64) -> Self {
        let Vector(x, y) = self;
        Vector(x * cosi(deg) - y * sini(deg), x * sini(deg) + y * cosi(deg))
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }
}

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

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(Vector::NORTH.rotate(90), Vector::EAST);
        assert_eq!(Vector::EAST.rotate(90), Vector::SOUTH);
        assert_eq!(Vector::SOUTH.rotate(90), Vector::WEST);
        assert_eq!(Vector::WEST.rotate(90), Vector::NORTH);
    }

    #[test]
    fn test_sini() {
        assert_eq!(sini(90), 1);
        assert_eq!(sini(-90), -1);
    }
}
