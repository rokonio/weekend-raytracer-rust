use minifb::{Key, Window, WindowOptions};
use nalgebra_glm as glm;
use rayon::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
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
    let pixel_color = glm::vec3(
        i as f32 / (WIDTH - 1) as f32,
        j as f32 / (HEIGHT - 1) as f32,
        0.25,
    );
    out_color(pixel_color)
}

type Color = glm::Vec3;

fn out_color(pixel_color: Color) -> u32 {
    let ir = (pixel_color.x * 255.999) as u8;
    let ig = (pixel_color.y * 255.999) as u8;
    let ib = (pixel_color.z * 255.999) as u8;
    from_u8_rgb(ir, ig, ib)
}
