
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            e: [x, y, z],
        }
    }

    // Practical accessors.
    pub fn x(self) -> f64 {
        self[0]
    }
    pub fn y(self) -> f64 {
        self[1]
    }
    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn lenght(self) -> f64 {
        self.lenght_squared().sqrt()
    }

    pub fn lenght_squared(self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }
}

use std::ops::Neg;
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self[0], -self[1], -self[2]]
        }
    }
}

use std::ops::{Index, IndexMut};
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, vec_index: usize) -> &f64 {
        match vec_index {
            0 => &self.e[0],
            1 => &self.e[1],
            2 => &self.e[2],
            _ => panic!("Index greater than 2!"),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, vec_index: usize) -> &mut f64 {
        match vec_index {
            0 => &mut self.e[0],
            1 => &mut self.e[1],
            2 => &mut self.e[2],
            _ => panic!("Index greater than 2!"),
        }
    }
}

use std::ops::Add;
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        }
    }
}

use std::ops::Sub;
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
        }
    }
}

use std::ops::Mul;
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs[0], self * rhs[1], self * rhs[2]]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Self {
            e: [t * self[0], t * self[1], t * self[2]]
        }
    }
}

use std::ops::Div;
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        Vec3 {
            e: [
                (1.0 / t) * self[0],
                (1.0 / t) * self[1],
                (1.0 / t) * self[2],
                ]

        }
    }
}

use std::ops::AddAssign;
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
        }
    }
}

use std::ops::MulAssign;
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

use std::ops::DivAssign;
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] *= 1.0 / rhs;
        self[1] *= 1.0 / rhs;
        self[2] *= 1.0 / rhs;
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        e: [
            lhs[1] * rhs[2] - lhs[2] * rhs[1],
            lhs[2] * rhs[0] - lhs[0] * rhs[2],
            lhs[0] * rhs[1] - lhs[1] * rhs[0]
        ]
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.lenght()
}