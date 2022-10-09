use std::fmt::{Display, Formatter};
use std::ops::Mul;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Resolution {
    x: i32,
    y: i32,
}

impl Resolution {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub(crate) fn available() -> Vec<Resolution> {
        let mut res = vec![];
        for i in -2..3 {
            let multiplier = (2.0 as f64).powi(i);
            res.push(Resolution::new(1600, 1200) * multiplier);
            res.push(Resolution::new(1920, 1080) * multiplier);
        }
        res.sort_by(|a, b| a.x.cmp(&b.x));
        res
    }
}

impl Mul<f64> for Resolution {
    type Output = Resolution;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new((self.x as f64 * rhs) as i32, (self.y as f64 * rhs) as i32)
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl From<Resolution> for [usize; 2] {
    fn from(resolution: Resolution) -> Self {
        [resolution.x as usize, resolution.y as usize]
    }
}
