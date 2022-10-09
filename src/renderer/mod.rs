use egui::{ColorImage};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::{Color3, Ray, Vec3};

use crate::renderer::camera::Camera;
use crate::renderer::hittable::Hittable;
use crate::renderer::scene::Scene;


pub mod hittable;
pub mod camera;
mod random;
pub mod scene;

pub struct RenderParams {
    pub(crate) focal_length: f64,
    pub(crate) samples: i16,
    pub min_ray_distance: f64
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            focal_length: 1.0,
            samples: 10,
            min_ray_distance: 0.001
        }
    }
}


pub struct Renderer {
    random: ThreadRng,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Self {
            random: thread_rng()
        }
    }

    pub fn render(&mut self, image: &mut ColorImage, params: &RenderParams, scene: &Scene) {
        let image_width = image.size[0] as f64;
        let camera = Camera::new(image.size, params.focal_length);
        let scale = camera.viewport_width / image_width;

        for y in 0..image.size[1] {
            for x in 0..image.size[0] {
                let mut cumulated_color = Color3::splat(0.0);

                for _sample in 1..params.samples {
                    let u = (x as f64 + self.random.gen::<f64>()) * scale;
                    let v = (y as f64 + self.random.gen::<f64>()) * scale;


                    let ray = camera.cast_ray(u, v);
                    let color = self.ray_color(&ray, scene, params, 0);
                    cumulated_color = cumulated_color + color;
                }

                Self::set_pixel(image, x, y, cumulated_color, params.samples);
            }
        }
    }

    fn ray_color(&mut self, ray: &Ray, scene: &Scene, params: &RenderParams, depth: i32) -> Color3 {
        if depth > 50 {
            return Color3::splat(0.0);
        }
        let hit = scene.hit(ray, 0.001, f64::INFINITY);

        // let hit_distance = Self::hit_sphere(&center, radius, ray);
        if let Some(the_hit) = hit {
            let random_bounce = the_hit.point + the_hit.normal + self.random_in_unit_sphere();
            // return (the_hit.normal + Color3::splat(1.0)) * 0.5;
            let new_ray = Ray::new(the_hit.point, random_bounce);
            return self.ray_color(&new_ray, &scene, params, depth + 1) * 0.5;
        }


        let unit_direction = ray.direction() / ray.direction().length();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Color3::splat(1.0).lerp(1.0 - t, &BG_COLOR);
    }

    fn set_pixel(render_image: &mut ColorImage, x: usize, y: usize, color: Color3, i: i16) {
        let size = render_image.size;
        let dest = &mut render_image.pixels[(size[1] - y - 1) * size[0] + x];

        let scale = 1.0 / (i as f64);

        let r = (color.x() * scale).sqrt().clamp(0.0, 1.0);
        let g = (color.y() * scale).sqrt().clamp(0.0, 1.0);
        let b = (color.z() * scale).sqrt().clamp(0.0, 1.0);


        dest[0] = fast_round(r * ALMOST_256);
        dest[1] = fast_round(g * ALMOST_256);
        dest[2] = fast_round(b * ALMOST_256);
    }

    fn random_in_unit_sphere(&mut self) -> Vec3 {
        Vec3::new(
            self.random.gen::<f64>(),
            self.random.gen::<f64>(),
            self.random.gen::<f64>(),
        )
    }
}

fn fast_round(r: f64) -> u8 {
    (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
}

const ALMOST_256: f64 = 255.999;
static BG_COLOR: Vec3 = Color3 { data: [0.5, 0.7, 1.0] };
