use std::time::Instant;
use egui::{Color32, ColorImage, Response, TextureFilter, TextureHandle, Ui};
use crate::{Color3, MyApp, Ray, Vec3};

use crate::renderer::{Renderer, RenderParams};

pub struct RenderBox {
    tex_handle: Option<TextureHandle>,
    render_image: ColorImage,
    renderer: Renderer
}


impl RenderBox {
    pub fn new() -> RenderBox {
        let image_data = ColorImage::new([800, 600], Color32::default());
        Self {
            tex_handle: None,
            render_image: image_data,
            renderer: Renderer {  }
        }
    }



    pub fn render(&mut self, ui: &mut Ui, params: &RenderParams) -> Response {

        let texture: &mut TextureHandle = self.tex_handle.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                self.render_image.clone(),
                TextureFilter::Linear,
            )
        });

        self.renderer.render(&mut self.render_image, params);
        texture.set(self.render_image.clone(), TextureFilter::Linear);
        ui.image(texture, ui.available_size())
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Render parameters ");
            ui.add(
                egui::Slider::new(&mut self.params.focal_length, 0.0..=1.0)
                    .text("Focal length")
            );
            ui.add(
                egui::Slider::new(&mut self.params.radius, 0.0..=1.0)
                    .text("Sphere radius")
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let delta = self.last_frame_time.elapsed().as_nanos();
            self.last_frame_time = Instant::now();

            let fps = 1_000_000_000 / delta;
            ui.heading(format!("Render fps: {}", fps));
            self.render_box.render(ui, &self.params);
        });
        ctx.request_repaint();
    }
}
