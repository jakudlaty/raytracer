use egui::{ColorImage};
use egui::color::gamma_u8_from_linear_f32;
use crate::{Color3, Ray, Vec3};
use crate::math::Point3;

pub struct RenderParams {
    pub(crate) focal_length: f64,
    pub(crate) radius: f64,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            focal_length: 1.0,
            radius: 0.5
        }
    }
}
pub struct Renderer {}

impl Renderer {
    fn ray_color(ray: &Ray, params: &RenderParams) -> Vec3 {
        let center = Point3::new(0.0, 0.0, -1.0);
        let radius = params.radius;
        let hit_distance = Self::hit_sphere(&center, radius, ray);
        if hit_distance > 0.0 {
            let n = (ray.at(hit_distance) - Vec3::new(0.0, 0.0, -1.0)).normalized();
            return Color3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
        }

        let unit_direction = ray.direction() / ray.direction().length();

        let t = 0.5 * (unit_direction.y() + 1.0);

        return Color3::splat(1.0).lerp(1.0 - t, &BG_COLOR);
        // return (Color3::splat(1.0 - t)) + (BG_COLOR * t);
    }

    fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
        let oc = r.origin() - center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = 2.0 * Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - radius * radius;


        let discriminant = (b * b) - (4.0 * a * c);
        return if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        };
    }


    pub fn render(&mut self, image: &mut ColorImage, params: &RenderParams) {
        let image_width = image.size[0] as f64;
        let image_height = image.size[1] as f64;
        let aspect_ratio = image_width / image_height;

        let viewport_height: f64 = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = params.focal_length;
        let scale = viewport_width / image_width;

        let origin = Point3::new(0.0, 0.0, 0.0);

        // let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = Vec3::new(origin.x() - (viewport_width / 2.0), origin.y() - (viewport_height / 2.0), origin.z() - focal_length);


        for y in 0..image.size[1] {
            for x in 0..image.size[0] {
                let u = x as f64 * scale;
                let v = y as f64 * scale;

                let ray = Ray::new(
                    origin,
                    lower_left_corner + Vec3::new(u, v, 0.0) - origin,
                );

                Self::set_pixel(image, x, y, Self::ray_color(&ray, params));
            }
        }
    }

    fn set_pixel(render_image: &mut ColorImage, x: usize, y: usize, color: Color3) {
        let size = render_image.size;
        let dest = &mut render_image.pixels[y * size[0] + x];
        dest[0] = fast_round(color.x() * ALMOST_256);
        dest[1] = fast_round(color.y() * ALMOST_256);
        dest[2] = fast_round(color.z() * ALMOST_256);
    }
}

fn fast_round(r: f64) -> u8 {
    (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
}

const ALMOST_256: f64 = 255.999;
static BG_COLOR: Vec3 = Color3 { data: [0.5, 0.7, 1.0] };
