use crate::renderer::camera::Camera;
use crate::renderer::hittable::Hittable;
use crate::renderer::scene::Scene;
use crate::renderer::RenderParams;
use crate::Ray;
use crate::{Color3, Vec3};
use egui::{Color32, ColorImage};
use rand::thread_rng;
use rand::Rng;
use std::sync::mpsc::{Receiver, RecvError, Sender};

pub enum RenderThreadCommand {
    //TODO: if the scene grows it should be shared between UI and renderer in RWMutex to prevent copying scene on each frame
    UpdateScene(Scene),
    UpdateRenderParams(RenderParams),
    RequestFrame,
}

pub enum RenderThreadResponse {
    FrameRendered(ColorImage),
    ProgressUpdate(f64),
}

pub struct RenderThread {
    pub(crate) sender: Sender<RenderThreadResponse>,
    pub(crate) receiver: Receiver<RenderThreadCommand>,
    pub(crate) scene: Option<Scene>,
    pub(crate) params: RenderParams,
}

impl RenderThread {
    pub(crate) fn run(&mut self) -> Result<(), RecvError> {
        loop {
            let command = self.receiver.recv()?;
            match command {
                RenderThreadCommand::UpdateScene(scene) => self.scene = Some(scene),
                RenderThreadCommand::UpdateRenderParams(params) => self.params = params,
                RenderThreadCommand::RequestFrame => {
                    if let Some(scene) = &self.scene {
                        let render_params = &self.params;
                        let mut image =
                            ColorImage::new(render_params.resolution.into(), Color32::BLACK);
                        self.render(&mut image, render_params, scene);
                        self.sender
                            .send(RenderThreadResponse::FrameRendered(image))
                            .expect("Unable to send response")
                    }
                }
            }
        }
    }

    pub fn render(&self, image: &mut ColorImage, params: &RenderParams, scene: &Scene) {
        let mut rng = thread_rng();
        let image_width = image.size[0] as f64;
        let image_height = image.size[1] as f64;
        let camera = Camera::new(image.size, params.focal_length);
        let scale = camera.viewport_width / image_width;

        for y in 0..image.size[1] {
            for x in 0..image.size[0] {
                let mut cumulated_color = Color3::splat(0.0);

                for _sample in 1..params.samples {
                    let u = (x as f64 + rng.gen::<f64>()) * scale;
                    let v = (y as f64 + rng.gen::<f64>()) * scale;

                    let ray = camera.cast_ray(u, v);
                    let color = Self::ray_color(&ray, scene, params, 0);
                    cumulated_color = cumulated_color + color;
                }

                Self::set_pixel(image, x, y, cumulated_color, params.samples);
            }
            if y % (image.size[1] / 50) == 0 {
                self.sender
                    .send(RenderThreadResponse::ProgressUpdate(
                        y as f64 / image_height,
                    ))
                    .expect("Unable to comunicate with UI");
            }
        }
    }

    fn ray_color(ray: &Ray, scene: &Scene, params: &RenderParams, depth: i32) -> Color3 {
        if depth > 50 {
            return Color3::splat(0.0);
        }
        let hit = scene.hit(ray, 0.001, f64::INFINITY);

        // let hit_distance = Self::hit_sphere(&center, radius, ray);
        if let Some(the_hit) = hit {
            let random_bounce = the_hit.point + the_hit.normal + Self::random_in_unit_sphere();
            // return (the_hit.normal + Color3::splat(1.0)) * 0.5;
            let new_ray = Ray::new(the_hit.point, random_bounce);
            return Self::ray_color(&new_ray, scene, params, depth + 1) * 0.5 * the_hit.surface;
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

    fn random_in_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }
}

const ALMOST_256: f64 = 255.999;
static BG_COLOR: Vec3 = Color3 {
    data: [0.5, 0.7, 1.0],
};

#[inline]
fn fast_round(r: f64) -> u8 {
    (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
}
