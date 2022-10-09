use std::io::Error;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::task::ready;
use std::thread;
use crate::{Color3, Ray, Vec3};
use egui::{Color32, ColorImage};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use crate::renderer::camera::Camera;
use crate::renderer::hittable::Hittable;
use crate::renderer::scene::Scene;

pub mod camera;
pub mod hittable;
pub mod scene;

#[derive(Copy, Clone)]
pub struct RenderParams {
    pub(crate) focal_length: f64,
    pub(crate) samples: i16,
    pub min_ray_distance: f64,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            focal_length: 1.0,
            samples: 2,
            min_ray_distance: 0.001,
        }
    }
}

enum RenderThreadCommand {
    //TODO: if the scene grows it should be shared between UI and renderer in RWMutex to prevent copying scene on each frame
    UpdateScene(Box<dyn Hittable>),
    UpdateRenderParams(RenderParams),
    RequestFrame,
}

enum RenderThreadResponse {
    FrameRendered(ColorImage)
}

pub struct RenderThread {
    sender: Sender<RenderThreadResponse>,
    receiver: Receiver<RenderThreadCommand>,
    scene: Option<Box<dyn Hittable>>,
    params: RenderParams,
}

impl RenderThread {
    pub(crate) fn run(&mut self) -> Result<(), RecvError> {
        loop {
            let command = self.receiver.recv()?;
            match command {
                RenderThreadCommand::UpdateScene(scene) => {
                    self.scene = Some(scene)
                }
                RenderThreadCommand::UpdateRenderParams(params) => {
                    self.params = params
                }
                RenderThreadCommand::RequestFrame => {
                    if let Some(scene) = &self.scene {
                        let mut image = ColorImage::new([800, 600], Color32::BLACK);
                        let render_params = &self.params;
                        self.render(&mut image, render_params, scene.clone_box());
                        self.sender.send(RenderThreadResponse::FrameRendered(image))
                            .expect("Unable to send response")
                    }
                }
            }
        }
    }


    pub fn render(&self, image: &mut ColorImage, params: &RenderParams, scene: Box<dyn Hittable>) {
        let mut rng = thread_rng();
        let image_width = image.size[0] as f64;
        let camera = Camera::new(image.size, params.focal_length);
        let scale = camera.viewport_width / image_width;

        for y in 0..image.size[1] {
            for x in 0..image.size[0] {
                let mut cumulated_color = Color3::splat(0.0);

                for _sample in 1..params.samples {
                    let u = (x as f64 + rng.gen::<f64>()) * scale;
                    let v = (y as f64 + rng.gen::<f64>()) * scale;

                    let ray = camera.cast_ray(u, v);
                    let color = Self::ray_color(&ray, scene.clone(), params, 0);
                    cumulated_color = cumulated_color + color;
                }

                Self::set_pixel(image, x, y, cumulated_color, params.samples);
            }
        }
    }

    fn ray_color(ray: &Ray, scene: Box<dyn Hittable>, params: &RenderParams, depth: i32) -> Color3 {
        if depth > 50 {
            return Color3::splat(0.0);
        }
        let hit = scene.hit(ray, 0.001, f64::INFINITY);

        // let hit_distance = Self::hit_sphere(&center, radius, ray);
        if let Some(the_hit) = hit {
            let random_bounce = the_hit.point + the_hit.normal + Self::random_in_unit_sphere();
            // return (the_hit.normal + Color3::splat(1.0)) * 0.5;
            let new_ray = Ray::new(the_hit.point, random_bounce);
            return Self::ray_color(&new_ray, scene, params, depth + 1) * 0.5;
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
        Vec3::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>(),
        )
    }
}

pub struct Renderer {
    sender: Sender<RenderThreadCommand>,
    receiver: Receiver<RenderThreadResponse>,
    waiting_for_next_frame: bool
}

impl Renderer {
    pub(crate) fn create() -> Self {
        let (command_sender, command_revceiver) = channel();
        let (response_sender, response_receiver) = channel();


        let handle = thread::spawn(|| {
            let mut thread = RenderThread {
                sender: response_sender,
                receiver: command_revceiver,
                scene: None,
                params: RenderParams::default(),
            };

            thread.run()
        });

        Self {
            sender: command_sender,
            receiver: response_receiver,
            waiting_for_next_frame: false
        }
    }

    fn send_command(&self, command: RenderThreadCommand) {
        self.sender.send(command)
            .expect("Unable to comunicate with renderer");
    }

    pub fn render(&mut self, image: &mut ColorImage, params: RenderParams, scene: Box<dyn Hittable>){
        if ! self.waiting_for_next_frame {
            self.send_command(RenderThreadCommand::UpdateScene(scene));
            self.send_command(RenderThreadCommand::UpdateRenderParams(params));
            self.send_command(RenderThreadCommand::RequestFrame);
            self.waiting_for_next_frame = true
        } else {
            if let Ok(f) = self.receiver.try_recv() {
                match f {
                    RenderThreadResponse::FrameRendered(im) => {
                        *image = im
                    }
                }
                self.waiting_for_next_frame = false
            }

        }


    }
}

fn fast_round(r: f64) -> u8 {
    (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
}

const ALMOST_256: f64 = 255.999;
static BG_COLOR: Vec3 = Color3 {
    data: [0.5, 0.7, 1.0],
};
