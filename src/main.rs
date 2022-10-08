#![feature(trait_alias)]
#![feature(portable_simd)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate core;

use std::time::{Instant};
use eframe::{egui, HardwareAcceleration};
use egui::Vec2;
use crate::math::{Color3, Ray, Vec3};
use crate::renderer::ui::{RenderBox, RenderParams};

mod renderer;
mod math;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.hardware_acceleration = HardwareAcceleration::Required;
    options.initial_window_size = Some(Vec2::new(1600.0, 1200.0));
    options.vsync = false;


    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    render_box: Option<RenderBox>,
    last_frame_time: Instant,
    render_params: RenderParams
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            render_box: None,
            last_frame_time: Instant::now(),
            render_params: RenderParams {
                focal_length: 0.2,
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.render_box.is_none() {
            self.render_box = Some(RenderBox::new(ctx))
        }

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Render parameters ");
            ui.add(egui::Slider::new(&mut self.render_params.focal_length, 0.0..=0.2).text("Focal length"));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let delta = self.last_frame_time.elapsed().as_nanos();
            self.last_frame_time = Instant::now();

            let fps = 1_000_000_000 / delta;
            ui.heading(format!("Render fps: {}", fps));

            self.render_box.as_mut().map(|rb| {
                rb.render(ui, &self.render_params);
            })
        });
        ctx.request_repaint();
    }
}
