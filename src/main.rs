// #![feature(trait_alias)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate core;


use eframe::{egui, HardwareAcceleration};
use egui::Vec2;
use crate::app::MyApp;
use crate::math::{Color3, Ray, Vec3};

mod renderer;
mod math;
mod app;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.hardware_acceleration = HardwareAcceleration::Required;
    options.initial_window_size = Some(Vec2::new(1600.0, 1200.0));
    options.vsync = false;


    eframe::run_native(
        "Ray tracer in one weekend",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

