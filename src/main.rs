mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use std::sync::mpsc;
use std::thread;

use minifb::{Key, Window, WindowOptions};
extern crate nalgebra_glm as glm;
use once_cell::sync::OnceCell;
use rayon::prelude::*;

const ASPECT_RATION: f32 = 16.0 / 9.0;
const WIDTH: usize = 640;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;
const VIEW_PORT_HEIGHT: f32 = 2.0;
const VIEW_PORT_WIDTH: f32 = ASPECT_RATION * VIEW_PORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

const ORIGIN: glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);
const HORIZONTAL: glm::Vec3 = glm::Vec3::new(VIEW_PORT_WIDTH, 0.0, 0.0);
const VERTICAL: glm::Vec3 = glm::Vec3::new(0.0, VIEW_PORT_HEIGHT, 0.0);

const UPDATE_RATE: usize = 5_000;

// Util function for minifb because it takes a specially formatted u32 for colors
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

type Color = glm::Vec3;

static WORLD: OnceCell<HittableList> = OnceCell::new();

fn init_world() {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(glm::vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(glm::vec3(0.0, -100.5, -1.0), 100.0)));
    if WORLD.set(world).is_err() {
        panic!("Tried to set WORLD twice. This is a bug");
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
    init_world();

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
            sender.send((xy, color)).unwrap();
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
    let lower_left_corner =
        ORIGIN - HORIZONTAL / 2. - VERTICAL / 2. - glm::Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render

    let u = i as f32 / (WIDTH - 1) as f32;
    let v = j as f32 / (HEIGHT - 1) as f32;
    let ray = ray::Ray::new(
        ORIGIN,
        lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
    );
    let pixel_color = ray_color(ray, WORLD.get().unwrap());
    out_color(pixel_color)
}

fn ray_color(ray: ray::Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(&ray, 0.0, f32::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.00), t)
}

fn out_color(pixel_color: Color) -> u32 {
    let ir = (pixel_color.x * 255.999) as u8;
    let ig = (pixel_color.y * 255.999) as u8;
    let ib = (pixel_color.z * 255.999) as u8;
    from_u8_rgb(ir, ig, ib)
}
