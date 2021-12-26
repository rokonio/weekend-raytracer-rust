use minifb::{Key, Window, WindowOptions};
extern crate nalgebra_glm as glm;
use rayon::prelude::*;

const ASPECT_RATION: f32 = 16.0 / 9.0;
const WIDTH: usize = 640;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;
const VIEW_PORT_HEIGHT: f32 = 2.0;
const VIEW_PORT_WIDTH: f32 = ASPECT_RATION * VIEW_PORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

const ORIGIN: glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);
const HORIZONTAL: glm::Vec3 = glm::Vec3::new(VIEW_PORT_WIDTH, 0.0, 0.0);
const VERTICAL: glm::Vec3 = glm::Vec3::new(0.0, VIEW_PORT_WIDTH, 0.0);

const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

type Color = glm::Vec3;

mod ray;

fn ray_color(ray: ray::Ray) -> Color {
    if hit_sphere(&ray, &glm::vec3(0.0, 0.0, -1.0), 0.5) {
        return Color::new(0.0, 1.0, 0.0);
    }
    let unit_dir = ray.dir().normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.00), t)
}

fn hit_sphere(ray: &ray::Ray, center: &glm::Vec3, radius: f32) -> bool {
    let oc: glm::Vec3 = ray.origin() - center;
    let a = ray.dir().norm_squared();
    let b = 2.0 * glm::dot(&ray.dir(), &oc);
    let c = oc.norm_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    let mut buffer = [0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    buffer.par_iter_mut().enumerate().for_each(|(xy, i)| {
        let x = xy % WIDTH;
        let y = xy / WIDTH;
        *i = pixel_processing(x, HEIGHT - y);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn pixel_processing(i: usize, j: usize) -> u32 {
    // Camera

    let lower_left_corner =
        ORIGIN - HORIZONTAL / 2. - VERTICAL / 2. - glm::Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render

    let u = i as f32 / (WIDTH - 1) as f32;
    let v = j as f32 / (HEIGHT - 1) as f32;
    let ray = ray::Ray::new(
        ORIGIN,
        lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
    );
    let pixel_color = ray_color(ray);
    out_color(pixel_color)
}

fn out_color(pixel_color: Color) -> u32 {
    let ir = (pixel_color.x * 255.999) as u8;
    let ig = (pixel_color.y * 255.999) as u8;
    let ib = (pixel_color.z * 255.999) as u8;
    from_u8_rgb(ir, ig, ib)
}
