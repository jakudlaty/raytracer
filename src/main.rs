// #![feature(trait_alias)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate core;

use crate::app::MyApp;
use crate::math::{Color3, Ray, Vec3};
use eframe::{egui, HardwareAcceleration};
use egui::{Color32, Rounding, Vec2};

mod app;
mod math;
mod renderer;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.hardware_acceleration = HardwareAcceleration::Required;
    options.initial_window_size = Some(Vec2::new(1600.0, 1200.0));
    options.vsync = true;

    eframe::run_native(
        "Ray tracer in one weekend",
        options,
        Box::new(|cc| {
            let ctx = &cc.egui_ctx;
            let mut style: egui::Style = (*ctx.style()).clone();
            style.visuals.button_frame = true;
            style.visuals.dark_mode = true;
            style.spacing.button_padding = Vec2::new(8.0, 5.0);

            style.spacing.item_spacing = egui::vec2(10.0, 7.0);
            ctx.set_style(style);


            Box::new(MyApp::default())
        }),
    );
}
