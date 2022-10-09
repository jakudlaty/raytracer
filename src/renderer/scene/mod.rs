use crate::math::Point3;
use crate::renderer::hittable::{Hit, Hittable};
use crate::renderer::scene::sphere::Sphere;
use crate::Ray;

pub(crate) mod sphere;

#[derive(Clone)]
pub enum SceneObject {
    Sphere(Sphere),
}

pub struct Scene {
    pub(crate) contents: Vec<SceneObject>,
}

impl Clone for Scene {
    fn clone(&self) -> Self {
        Self {
            contents: self.contents.iter().cloned().collect(),
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut closest_hit: Option<Hit> = None;

        for object in &self.contents {
            match object {
                SceneObject::Sphere(sphere) => {
                    if let Some(hit) = sphere.hit(ray, t_min, closest_so_far) {
                        closest_so_far = hit.t;
                        closest_hit = Some(hit)
                    }
                }
            }
        }
        return closest_hit;
    }

    fn name(&self) -> String {
        "scene".to_string()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            contents: vec![
                SceneObject::Sphere(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
                SceneObject::Sphere(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
            ],
        }
    }
}
