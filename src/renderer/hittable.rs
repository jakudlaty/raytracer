use crate::math::Point3;
use crate::{Ray, Vec3};

pub struct Hit {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync + Clone {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn name(&self) -> String;
}
