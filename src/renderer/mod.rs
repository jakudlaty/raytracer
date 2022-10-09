use crate::{Color3, Ray, Vec3};
use egui::{Color32, ColorImage};
use std::fmt::{Display, Formatter};
use std::ops::Mul;

use rand::Rng;

use crate::renderer::resolution::Resolution;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};

use crate::renderer::scene::Scene;
use crate::renderer::thread::{RenderThread, RenderThreadCommand, RenderThreadResponse};

pub mod camera;
pub mod hittable;
mod resolution;
pub mod scene;
mod thread;

#[derive(Clone)]
pub struct RenderParams {
    pub(crate) focal_length: f64,
    pub(crate) samples: i16,
    pub min_ray_distance: f64,
    pub resolution: Resolution,
    pub available_resolutions: Vec<Resolution>,
}

impl Default for RenderParams {
    fn default() -> Self {
        let resolutions = Resolution::available();
        Self {
            focal_length: 1.0,
            samples: 100,
            min_ray_distance: 0.001,
            resolution: resolutions[0],
            available_resolutions: resolutions,
        }
    }
}

pub struct Renderer {
    sender: Sender<RenderThreadCommand>,
    receiver: Receiver<RenderThreadResponse>,
    waiting_for_next_frame: bool,
    pub(crate) progress: f64,
}

impl Renderer {
    pub(crate) fn create() -> Self {
        let (command_sender, command_revceiver) = channel();
        let (response_sender, response_receiver) = channel();

        let handle = std::thread::spawn(|| {
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
            waiting_for_next_frame: false,
            progress: 0.0,
        }
    }

    fn send_command(&self, command: RenderThreadCommand) {
        self.sender
            .send(command)
            .expect("Unable to comunicate with renderer");
    }

    pub fn render(&mut self, image: &mut ColorImage, params: RenderParams, scene: &Scene) {
        if !self.waiting_for_next_frame {
            self.send_command(RenderThreadCommand::UpdateScene(scene.clone()));
            self.send_command(RenderThreadCommand::UpdateRenderParams(params));
            self.send_command(RenderThreadCommand::RequestFrame);
            self.waiting_for_next_frame = true
        } else {
            while let Ok(f) = self.receiver.try_recv() {
                match f {
                    RenderThreadResponse::FrameRendered(im) => {
                        *image = im;
                        self.waiting_for_next_frame = false
                    }
                    RenderThreadResponse::ProgressUpdate(fraction) => self.progress = fraction,
                }
            }
        }
    }
}
