use crate::math::Point3;
use crate::{Ray, Vec3};
use uuid::Uuid;

pub struct Hit {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn uid(&self) -> Uuid;
    fn name(&self) -> String;
}
