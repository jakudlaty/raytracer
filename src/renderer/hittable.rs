use crate::math::Point3;
use crate::{Ray, Vec3};
use uuid::Uuid;

pub struct Hit {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync   {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn uid(&self) -> Uuid;
    fn name(&self) -> String;
    fn clone_box(&self) -> Box<dyn Hittable>;
}

//its workaround because CLone requires Self::Sized
impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

