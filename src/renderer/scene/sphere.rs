use crate::math::Point3;
use crate::renderer::hittable::{Hit, Hittable};
use crate::{Color3, Ray, Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
    pub max_radius: f64,
    pub color: Color3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            max_radius: 2.0 * radius,
            color: Color3::splat(1.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        return if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let hit_point = ray.at(root);
            let outward_normal = (hit_point - self.center) / self.radius;

            let front_face = Vec3::dot(ray.direction(), &outward_normal) < 0.0;
            let normal = match front_face {
                true => outward_normal,
                false => -outward_normal,
            };

            let hit_record = Hit {
                point: hit_point,
                normal,
                front_face,
                t: root,
                surface: self.color,
            };
            return Some(hit_record);
        };
    }

    fn name(&self) -> String {
        format!("Spehere at {}", self.center)
    }
}
