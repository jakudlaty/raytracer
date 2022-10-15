use crate::renderer::hittable::Hittable;
use crate::renderer::scene::{Scene, SceneObject};
use crate::renderer::{RenderParams, Renderer};
use crate::{MyApp, Vec3};
use egui::{Button, Color32, ColorImage, ProgressBar, Response, Stroke, TextureFilter, TextureHandle, Ui, Widget};

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
            renderer: Renderer::create(),
            scene: Scene::default(),
        }
    }
    pub fn request_render(&mut self, params: RenderParams) {
        self.renderer.request_render(params, &self.scene);
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let texture: &mut TextureHandle = self.tex_handle.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx()
                .load_texture("my-image", self.render_image.clone(), TextureFilter::Linear)
        });

        self.renderer
            .update_result(&mut self.render_image);

        texture.set(self.render_image.clone(), TextureFilter::Linear);
        ui.vertical(|ui| {
            if self.renderer.waiting_for_next_frame {
                let pb = ProgressBar::new(self.renderer.progress as f32)
                    .animate(self.renderer.waiting_for_next_frame)
                    .show_percentage();
                pb.ui(ui);
            }
            ui.image(texture, ui.available_size());
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Render parameters ");


                egui::ComboBox::from_label("Render resolution")
                    .selected_text(format!("{}", self.params.resolution))
                    .show_ui(ui, |ui| {
                        for res in &self.params.available_resolutions {
                            ui.selectable_value(
                                &mut self.params.resolution,
                                *res,
                                format!("{}", res),
                            );
                        }
                    });
                ui.add(
                    egui::Slider::new(&mut self.params.focal_length, 0.0..=1.0)
                        .text("Focal length"),
                );
                ui.add(
                    egui::Slider::new(&mut self.params.samples, 0..=1000).text("Number of samples"),
                );

                ui.add(
                    egui::Slider::new(&mut self.params.min_ray_distance, 0.0001..=0.1)
                        .text("Min ray distance"),
                );
                ui.separator();
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
                                let mut x = sphere.color.into();
                                ui2.color_edit_button_rgb(&mut x);
                                sphere.color = x.into()
                            });
                        }
                    });
                }
                ui.separator();
                ui.vertical_centered(|ui| {
                    let button = Button::new("Render")
                        .fill(Color32::from_rgb(50, 70, 137))
                        .stroke(Stroke::new(1.0, Color32::WHITE));

                    let button_ui = ui.add_sized([160., 40.], button);

                    if button_ui.clicked() {
                        self.render_box.request_render(self.params.clone())
                    }
                })
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_box.draw(ui);
        });
        ctx.request_repaint();
    }
}
