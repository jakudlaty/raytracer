use crate::renderer::hittable::Hittable;
use crate::renderer::scene::{Scene, SceneObject};
use crate::renderer::{RenderParams, Renderer};
use crate::MyApp;
use egui::{Color32, ColorImage, Response, TextureFilter, TextureHandle, Ui};

pub struct RenderBox {
    tex_handle: Option<TextureHandle>,
    render_image: ColorImage,
    renderer: Renderer,
    scene: Scene,
}

impl RenderBox {
    pub fn new() -> RenderBox {
        let image_data = ColorImage::new([400, 300], Color32::default());
        Self {
            tex_handle: None,
            render_image: image_data,
            renderer: Renderer::create(),
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
            .render(&mut self.render_image, params.clone(), &self.scene);

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
                ui.push_id(id, |ui| match object {
                    SceneObject::Sphere(sphere) => {
                        ui.collapsing(sphere.name(), |ui2| {
                            ui2.add(
                                egui::Slider::new(&mut sphere.radius, 0.0..=sphere.max_radius)
                                    .text("Sphere radius"),
                            );
                        });
                    }
                });
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_box.render(ui, &self.params);
        });
        ctx.request_repaint();
    }
}
