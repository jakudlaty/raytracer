use egui::Key::N;
use uuid::Uuid;
use crate::math::Point3;
use crate::Ray;
use crate::renderer::hittable::{Hit, Hittable};
use crate::renderer::scene::sphere::Sphere;
use type_uuid::TypeUuid;
pub mod sphere;

#[derive(TypeUuid)]
#[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
pub struct Scene {
    pub(crate) contents: Vec<Box<dyn Hittable>>,
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut closest_hit : Option<Hit> = None;

        for object in &self.contents {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit)
            }
        }
        return closest_hit;
    }

    fn uid(&self) -> Uuid {
        return Uuid::from_bytes(Scene::UUID)
    }

    fn name(&self) -> String {
        "scene".to_string()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            contents: vec![
                Box::new(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 }),
                Box::new(Sphere { center:Point3::new(0.0,-100.5,-1.0), radius: 100.0})

            ]
        }
    }
}