use crate::math::Point3;
use crate::math::{Vec3};


pub struct Ray
{
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }

    pub(crate) fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
}