use crate::renderer::scene::sphere::Sphere;
use crate::renderer::scene::Scene;
use crate::renderer::{RenderParams, Renderer};
use crate::{MyApp};
use egui::{Color32, ColorImage, Response, TextureFilter, TextureHandle, Ui};
use std::mem;
use std::time::Instant;
use type_uuid::TypeUuid;
use uuid::Uuid;

pub struct RenderBox {
    tex_handle: Option<TextureHandle>,
    render_image: ColorImage,
    renderer: Renderer,
    scene: Scene,
}

impl RenderBox {
    pub fn new() -> RenderBox {
        let image_data = ColorImage::new([800, 600], Color32::default());
        Self {
            tex_handle: None,
            render_image: image_data,
            renderer: Renderer::new(),
            scene: Scene::default(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui, params: &RenderParams) -> Response {
        let texture: &mut TextureHandle = self.tex_handle.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx()
                .load_texture("my-image", self.render_image.clone(), TextureFilter::Linear)
        });

        self.renderer
            .render(&mut self.render_image, params, &self.scene);
        texture.set(self.render_image.clone(), TextureFilter::Linear);
        ui.image(texture, ui.available_size())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Render parameters ");
            ui.add(
                egui::Slider::new(&mut self.params.focal_length, 0.0..=1.0).text("Focal length"),
            );
            ui.add(egui::Slider::new(&mut self.params.samples, 0..=100).text("Number of samples"));

            ui.add(
                egui::Slider::new(&mut self.params.min_ray_distance, 0.0001..=0.1)
                    .text("Min ray distance"),
            );

            ui.heading("Scene contents ");
            let mut id = 1;
            for object in &mut self.render_box.scene.contents {
                id += 1;
                ui.push_id(id, |ui| {
                    if object.uid() == Uuid::from_bytes(Sphere::UUID) {
                        //this is sphere
                        ui.collapsing("Sphere", |ui2| {
                            //TODO: Make it safe
                            unsafe {
                                let sphere: &mut Box<Sphere> = mem::transmute(object);
                                ui2.add(
                                    egui::Slider::new(&mut sphere.radius, 0.0..=100.0)
                                        .text("Sphere radius"),
                                );
                            }
                        });
                    };
                });
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let delta = self.last_frame_time.elapsed().as_micros() as f64;
            self.last_frame_time = Instant::now();

            ui.heading(format!("Render ms: {:.2}", delta / 1000.0));
            self.render_box.render(ui, &self.params);
        });
        ctx.request_repaint();
    }
}
