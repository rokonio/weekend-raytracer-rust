mod camera;
mod hittable;
mod hittable_list;
mod material;
mod my_scene;
mod noise;
mod ray;
mod scene;
mod sphere;

use std::io::{stdout, Write};
use std::sync::{mpsc, Arc};
use std::{env, path, thread};

use hittable_list::HittableList;
use minifb::{Key, Window, WindowOptions};
use noise::*;
use ray::Ray;
// use my_scene::*;
use scene::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
extern crate nalgebra_glm as glm;
use image::{Rgb, RgbImage};
use once_cell::sync::OnceCell;
use rand_distr::{Distribution, UnitBall};
use rayon::prelude::*;

use crate::material::ScatterResponse;

// Util function for minifb because it takes a specially formatted u32 for
// colors
const fn from_u8_0rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

type Color = glm::Vec3;

static WORLD: OnceCell<HittableList> = OnceCell::new();
static CAMERA: OnceCell<Camera> = OnceCell::new();

const UPDATE_RATE: u64 = 30; // FPS
const MICRO_BETWEEN_FRAME: u64 = 1_000_000 / UPDATE_RATE;

fn main() {
    let now = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let default_path = "output.png".to_string();
    let save_path = args.get(1).unwrap_or(&default_path);
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
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
    window.limit_update_rate(Some(std::time::Duration::from_micros(MICRO_BETWEEN_FRAME)));
    init_world_and_camera();

    set_noise();
    // Render everything
    update_buffer(&mut buffer, &mut window);

    eprintln!(
        "\rFinished in {:.2}s",
        now.elapsed().as_millis() as f32 / 1000.0
    );

    save_buffer(&buffer, save_path);

    // Loop to keep window open
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn save_buffer(buffer: &[u32], path: impl AsRef<path::Path>) {
    let mut buffer2 = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    buffer2.pixels_mut().enumerate().for_each(|(xy, color)| {
        let [_, r, g, b] = buffer[xy].to_be_bytes();
        *color = Rgb([r, g, b]);
    });
    buffer2.save(path).unwrap();
}

fn update_buffer(buffer: &mut [u32], window: &mut Window) {
    let (sender, receiver) = mpsc::sync_channel(HEIGHT * WIDTH + 1);
    thread::spawn(move || {
        (0..(HEIGHT * WIDTH)).into_par_iter().for_each(|xy| {
            let x = xy % WIDTH;
            let y = xy / WIDTH;
            let color = pixel_processing(x, HEIGHT - y);
            if sender.send((xy, color)).is_err() {}
        });
        eprintln!(" - Finished computing");
    });
    let mut now = std::time::Instant::now();
    for (i, (xy, (r, g, b))) in receiver.iter().enumerate() {
        buffer[xy] = from_u8_0rgb(r, g, b);
        if now.elapsed().as_micros() as u64 > MICRO_BETWEEN_FRAME + 100 {
            now = std::time::Instant::now();
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
            if window.is_key_down(Key::Escape) {
                return;
            }
            let progress = i as f32 / (HEIGHT * WIDTH) as f32;
            eprint!("\r{:.1}%", progress * 100.0);
            stdout().flush().unwrap();
        }
    }
}

fn pixel_processing(i: usize, j: usize) -> (u8, u8, u8) {
    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    for s in 0..SAMPLE_PER_PIXEL {
        // Render

        let u = (i as f32 + randx(s)) / (WIDTH - 1) as f32;
        let v = (j as f32 + randy(s)) / (HEIGHT - 1) as f32;
        let ray = CAMERA.get().unwrap().get_ray(u, v);
        let r = ray_color(ray, WORLD.get().unwrap());
        pixel_color.x += r.x.min(1.0);
        pixel_color.y += r.y.min(1.0);
        pixel_color.z += r.z.min(1.0);
    }
    out_color(pixel_color)
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
    use tailcall::tailcall;
    #[tailcall]
    fn _ray_color(ray: ray::Ray, world: &dyn Hittable, depth: usize, accumulator: Color) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(&ray, 0.001, f32::INFINITY) {
            match rec.material.scatter(&ray, &rec) {
                ScatterResponse::Scatter(attenuation, scattered) => {
                    return _ray_color(
                        scattered,
                        world,
                        depth - 1,
                        attenuation.component_mul(&accumulator),
                    );
                }
                ScatterResponse::Absorb(absorbtion) => {
                    return absorbtion.component_mul(&accumulator);
                }
            }
        }

        sky_color(&ray, depth).component_mul(&accumulator)
    }
    _ray_color(ray, world, MAX_DEPTH, Color::new(1.0, 1.0, 1.0))
}

fn out_color(pixel_color: Color) -> (u8, u8, u8) {
    let scale = 1.0 / SAMPLE_PER_PIXEL as f32;
    let ir = ((pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ig = ((pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ib = ((pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    (ir, ig, ib)
}

#[inline]
fn random_in_unit_sphere() -> glm::Vec3 {
    glm::make_vec3(&UnitBall.sample(&mut rand::thread_rng())).normalize()
}

#[allow(unused)]
#[inline]
fn random_in_hemishpere(normal: &glm::Vec3) -> glm::Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
