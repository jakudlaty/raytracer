use egui::{Color32, ColorImage, ImageData, Response, TextureFilter, TextureHandle, Ui};
use crate::{Color3, Ray, Vec3};
use crate::math::Point3;

pub struct RenderBox {
    tex_handle: TextureHandle,
    render_image: ColorImage,
}

pub struct RenderParams {
    pub(crate) focal_length: f64,
}

const ALMOST_256: f64 = 255.9999999999999999;

impl RenderBox {
    pub fn new(ctx: &egui::Context) -> RenderBox {
        let image_data = ColorImage::new([1600, 1200], Color32::default());
        let tex_handle = ctx.load_texture("render", ImageData::Color(image_data.clone()), TextureFilter::Nearest);

        Self {
            tex_handle,
            render_image: image_data,
        }
    }

    fn pixel_color(color: Color3) -> Color32 {
        Color32::from_rgb(
            (ALMOST_256 * color.x()) as u8,
            (ALMOST_256 * color.y()) as u8,
            (ALMOST_256 * color.z()) as u8,
        )
    }

    pub fn render(&mut self, ui: &mut Ui, params: &RenderParams) -> Response {
        let size = self.render_image.size;
        let image_width = size[0] as f64;
        let image_height = size[1] as f64;
        let aspect_ratio = image_width / image_height;

        const viewport_height: f64 = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = params.focal_length;
        let scale = viewport_width / image_width;

        let origin = Point3::new(0.0, 0.0, 0.0);

        // let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = Vec3::new(origin.x() - viewport_width / 2.0, origin.y() - viewport_height / 2.0, origin.z() - focal_length);


        for y in 0..size[1] {
            for x in 0..size[0] {
                let u = x as f64 * scale;
                let v = y as f64 * scale;

                let ray = Ray::new(
                    origin,
                    lower_left_corner + Vec3::new(u, v, 0.0) - origin,
                );


                let color32 = Self::pixel_color(ray_color(&ray));
                self.render_image.pixels[y * size[0] + x] = color32;
            }
        }


        self.tex_handle
            .set(self.render_image.clone(), TextureFilter::Nearest);


        ui.image(&self.tex_handle, ui.available_size())
    }
}

static BG_COLOR: Vec3 = Color3 { data: [0.5, 0.7, 1.0] };

fn ray_color(r: &Ray) -> Vec3 {
    let center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    if hit_sphere(&center, radius, r) {
        return Color3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color3::splat(1.0 - t)) + (BG_COLOR * t);
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());

    let c = oc.length_squared() * radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    return discriminant > 0.0;
}
