use crate::math::Point3;
use crate::renderer::hittable::{Hit, Hittable};
use crate::{Ray, Vec3};
use type_uuid::TypeUuid;
use uuid::Uuid;

#[derive(TypeUuid)]
#[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
    pub max_radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            max_radius: 5.0 * radius,
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
            let hit_record = Hit {
                point: hit_point,
                normal: (hit_point - self.center) / self.radius,
                t: root,
            };
            return Some(hit_record);
        };
    }

    fn uid(&self) -> Uuid {
        Uuid::from_bytes(Sphere::UUID)
    }

    fn name(&self) -> String {
        format!("Spehere {}", 1)
    }
}
