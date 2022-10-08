use std::time::Instant;
use crate::app::ui::{RenderBox};
use crate::renderer::RenderParams;

mod ui;

pub struct MyApp {
    render_box: RenderBox,
    last_frame_time: Instant,
    render_params: RenderParams,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            render_box: RenderBox::new(),
            last_frame_time: Instant::now(),
            render_params: RenderParams {
                focal_length: 0.2,
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Render parameters ");
            ui.add(egui::Slider::new(&mut self.render_params.focal_length, 0.0..=0.2).text("Focal length"));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let delta = self.last_frame_time.elapsed().as_nanos();
            self.last_frame_time = Instant::now();

            let fps = 1_000_000_000 / delta;
            ui.heading(format!("Render fps: {}", fps));
            self.render_box.render(ui, &self.render_params);
        });
        ctx.request_repaint();
    }
}
