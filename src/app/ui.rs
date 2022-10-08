use egui::{Color32, ColorImage, Response, TextureFilter, TextureHandle, Ui};
use crate::{Color3, Ray, Vec3};
use crate::math::Point3;
use crate::renderer::RenderParams;

pub struct RenderBox {
    tex_handle: Option<TextureHandle>,
    render_image: ColorImage,
}



const ALMOST_256: f64 = 255.9999999999999999;

impl RenderBox {
    pub fn new() -> RenderBox {
        let image_data = ColorImage::new([800, 600], Color32::default());
        Self {
            tex_handle: None,
            render_image: image_data,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color3) {
        let size = self.render_image.size;
        let dest = &mut self.render_image.pixels[y * size[0] + x];
        dest[0] = (color.x() * ALMOST_256) as u8;
        dest[1] = (color.y() * ALMOST_256) as u8;
        dest[2] = (color.z() * ALMOST_256) as u8;
    }

    pub fn render(&mut self, ui: &mut Ui, params: &RenderParams) -> Response {
        let size = self.render_image.size;
        let image_width = size[0] as f64;
        let image_height = size[1] as f64;
        let aspect_ratio = image_width / image_height;

        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width = aspect_ratio * VIEWPORT_HEIGHT;
        let focal_length = params.focal_length;
        let scale = viewport_width / image_width;

        let origin = Point3::new(0.0, 0.0, 0.0);

        // let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = Vec3::new(origin.x() - viewport_width / 2.0, origin.y() - VIEWPORT_HEIGHT / 2.0, origin.z() - focal_length);


        for y in 0..size[1] {
            for x in 0..size[0] {
                let u = x as f64 * scale;
                let v = y as f64 * scale;

                let ray = Ray::new(
                    origin,
                    lower_left_corner + Vec3::new(u, v, 0.0) - origin,
                );

                self.set_pixel(x, y, ray_color(&ray));
            }
        }

        let texture: &mut egui::TextureHandle = self.tex_handle.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                egui::ColorImage::example(),
                egui::TextureFilter::Linear,
            )
        });

        texture.set(self.render_image.clone(), TextureFilter::Linear);
        ui.image(texture, ui.available_size())
    }
}

static BG_COLOR: Vec3 = Color3 { data: [0.5, 0.7, 1.0] };

fn ray_color(r: &Ray) -> Vec3 {
    let center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let hit_distance = hit_sphere(&center, radius, r);
    if hit_distance > 0.0 {
        return Color3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color3::splat(1.0 - t)) + (BG_COLOR * t);
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());

    let c = oc.length_squared() * radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    };
}
