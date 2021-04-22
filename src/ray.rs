use crate::vec3::Vec3;

use super::vec3;

pub type Point3 = vec3::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: vec3::Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}