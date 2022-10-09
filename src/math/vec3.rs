use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3 {
    pub(crate) data: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }

    //    #[inline(always)]
    pub fn from(data: [f64; 3]) -> Self {
        Self { data }
    }

    pub fn splat(value: f64) -> Self {
        Self {
            data: [value, value, value],
        }
    }

    //    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.data[0]
    }

    //    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.data[1]
    }

    //    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.data[2]
    }

    //    #[inline(always)]
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    //    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        let e = &self.data;
        return (e[0] * e[0]) + (e[1] * e[1]) + (e[2] * e[2]);
    }

    //    #[inline(always)]
    pub fn normalized(&self) -> Self {
        self / self.length()
    }

    //    #[inline(always)]
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        return (u.data[0] * v.data[0]) + (u.data[1] * v.data[1]) + (u.data[2] * v.data[2]);
    }

    //    #[inline(always)]
    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        return Self::new(
            u.data[1] * v.data[2] - u.data[2] * v.data[1],
            u.data[2] * v.data[0] - u.data[0] * v.data[2],
            u.data[0] * v.data[1] - u.data[1] * v.data[0],
        );
    }

    pub fn lerp(&self, alpha: f64, dest: &Vec3) -> Self {
        let beta = 1.0 - alpha;
        return Self::new(
            self.x() * alpha + dest.x() * beta,
            self.y() * alpha + dest.y() * beta,
            self.z() * alpha + dest.z() * beta,
        );
    }
}

// Math on references

impl Sub for &Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

//Math on values
impl Sub for Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

//scalar math
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    //    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;

impl Color3 {
    pub const WHITE: Color3 = Color3 {
        data: [1.0, 1.0, 1.0],
    };
}
