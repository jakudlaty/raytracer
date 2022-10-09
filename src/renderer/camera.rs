use crate::math::Point3;
use crate::{Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Vec3,
    pub(crate) viewport_width: f64,
    pub(crate) viewport_height: f64,
}

impl Camera {
    pub fn new(viewport_size: [usize; 2], focal_length: f64) -> Self {
        let image_width = viewport_size[0] as f64;
        let image_height = viewport_size[1] as f64;
        let aspect_ratio = image_width / image_height;

        let viewport_height: f64 = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Point3::new(0.0, 0.0, 0.0);

        // let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = Vec3::new(
            origin.x() - (viewport_width / 2.0),
            origin.y() - (viewport_height / 2.0),
            origin.z() - focal_length,
        );

        Self {
            origin,
            lower_left_corner,
            viewport_width,
            viewport_height,
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        let ray = Ray::new(
            self.origin,
            self.lower_left_corner + Vec3::new(u, v, 0.0) - self.origin,
        );
        ray
    }
}
