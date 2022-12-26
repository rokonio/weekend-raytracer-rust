mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use sphere::Sphere;
use std::sync::{mpsc, Arc};
use std::thread;

use minifb::{Key, Window, WindowOptions};
extern crate nalgebra_glm as glm;
use once_cell::sync::OnceCell;
use rand_distr::{Distribution, UnitBall};
use rayon::prelude::*;

const ASPECT_RATION: f32 = 16.0 / 9.0;
const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;
// const VIEW_PORT_HEIGHT: f32 = 2.0;
// const VIEW_PORT_WIDTH: f32 = ASPECT_RATION * VIEW_PORT_HEIGHT;
// const FOCAL_LENGTH: f32 = 1.0;
//
// const ORIGIN: glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);
// const HORIZONTAL: glm::Vec3 = glm::Vec3::new(VIEW_PORT_WIDTH, 0.0, 0.0);
// const VERTICAL: glm::Vec3 = glm::Vec3::new(0.0, VIEW_PORT_HEIGHT, 0.0);
const SAMPLE_PER_PIXEL: usize = 80;
const UPDATE_RATE: usize = 10_000 / (SAMPLE_PER_PIXEL);
const MAX_DEPTH: usize = 25;

// Util function for minifb because it takes a specially formatted u32 for colors
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

type Color = glm::Vec3;

const MATERIAL_GROUND: Lambertian = Lambertian {
    albedo: Color::new(0.8, 0.8, 0.0),
};
const MATERIAL_CENTER: Lambertian = Lambertian {
    albedo: Color::new(0.7, 0.3, 0.3),
};
const MATERIAL_LEFT: Metal = Metal {
    albedo: Color::new(0.8, 0.8, 0.8),
    fuzz: 0.0,
};
const MATERIAL_RIGHT: Metal = Metal {
    albedo: Color::new(0.8, 0.6, 0.2),
    fuzz: 1.0,
};
static WORLD: OnceCell<HittableList> = OnceCell::new();
static CAMERA: OnceCell<Camera> = OnceCell::new();

fn init_world_and_camera() {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        glm::vec3(0.0, -100.5, -1.0),
        100.0,
        Arc::new(MATERIAL_GROUND),
    )));
    world.add(Box::new(Sphere::new(
        glm::vec3(0.0, 0.0, -1.0),
        0.5,
        Arc::new(MATERIAL_CENTER),
    )));
    world.add(Box::new(Sphere::new(
        glm::vec3(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(MATERIAL_LEFT),
    )));
    world.add(Box::new(Sphere::new(
        glm::vec3(1.0, 0.0, -1.0),
        0.5,
        Arc::new(MATERIAL_RIGHT),
    )));
    if WORLD.set(world).is_err() {
        panic!("Tried to set WORLD twice. This is a bug");
    }

    let camera = Camera::new();
    if CAMERA.set(camera).is_err() {
        panic!("Tried to set CAMERA twice. This is a bug");
    }
}

fn main() {
    let mut buffer = [0u32; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Raytracing - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    init_world_and_camera();

    // Render everything
    update_buffer(&mut buffer, &mut window);

    // Loop to keep window open
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn update_buffer(buffer: &mut [u32], window: &mut Window) {
    let (sender, receiver) = mpsc::sync_channel(HEIGHT * WIDTH + 1);
    thread::spawn(move || {
        (0..(HEIGHT * WIDTH)).into_par_iter().for_each(|xy| {
            let x = xy % WIDTH;
            let y = xy / WIDTH;
            let color = pixel_processing(x, HEIGHT - y);
            if let Ok(_) = sender.send((xy, color)) {}
        });
    });
    for (i, (xy, color)) in receiver.iter().enumerate() {
        buffer[xy] = color;
        // Since this is a slow function, we update it once in every UPDATE_RATE calculation
        if i % UPDATE_RATE == 0 {
            // PERFORMANCE maybe update the window in a parrallel thread so it doesn't block
            // the buffer from updating
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            if window.is_key_down(Key::Escape) {
                return;
            }
        }
    }
}

fn pixel_processing(i: usize, j: usize) -> u32 {
    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    for _ in 0..SAMPLE_PER_PIXEL {
        // Render

        let u = (i as f32 + rand::random::<f32>()) / (WIDTH - 1) as f32;
        let v = (j as f32 + rand::random::<f32>()) / (HEIGHT - 1) as f32;
        let ray = CAMERA.get().unwrap().get_ray(u, v);
        pixel_color += ray_color(ray, WORLD.get().unwrap(), MAX_DEPTH);
    }
    out_color(pixel_color)
}

fn ray_color(ray: ray::Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(&ray, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(&ray, &rec) {
            return attenuation.component_mul(&ray_color(scattered, world, depth - 1));
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.00), t)
}

fn out_color(pixel_color: Color) -> u32 {
    let scale = 1.0 / SAMPLE_PER_PIXEL as f32;
    let ir = ((pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ig = ((pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ib = ((pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    from_u8_rgb(ir, ig, ib)
}

#[inline]
fn random_in_unit_sphere() -> glm::Vec3 {
    glm::make_vec3(&UnitBall.sample(&mut rand::thread_rng())).normalize()
}

#[inline]
fn random_in_hemishpere(normal: &glm::Vec3) -> glm::Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
