#![allow(unused)]

use crate::material::*;
use crate::sphere::*;
use crate::*;

// Give a name to the output file. Png is the recommended file format
pub const OUTPUT_NAME: &str = "scene_output.png";

// Rendering settings
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WIDTH: usize = 900;
pub const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
pub const SAMPLE_PER_PIXEL: usize = 100;
pub const MAX_DEPTH: usize = 50;

// Material definition
// pub const MATERIAL_LEFT: Lambertian = Lambertian::new(Color::new(0.0, 0.0,
// 1.0)); pub const MATERIAL_RIGHT: Lambertian = Lambertian::new(Color::new(1.0,
// 0.0, 0.0));
pub const MATERIAL_GROUND: Lambertian = Lambertian::new(Color::new(0.8, 0.8, 0.0));
pub const MATERIAL_CENTER: Lambertian = Lambertian::new(Color::new(0.1, 0.2, 0.5));
pub const MATERIAL_LEFT: Dielectic = Dielectic::new(1.5);
pub const MATERIAL_RIGHT: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

// Object creation
pub fn init_world_and_camera() {
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
        glm::vec3(-1.0, 0.0, -1.0),
        -0.4,
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

    let look_from = glm::vec3(3.0, 3.0, 2.0);
    let look_at = glm::vec3(0.0, 0.0, -1.0);
    let v_up = glm::vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).norm();
    let aperture = 2.0;
    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    if CAMERA.set(camera).is_err() {
        panic!("Tried to set CAMERA twice. This is a bug");
    }
}

// Sky color
pub fn sky_color(ray: &Ray, _depth: usize) -> Color {
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.00, 1.00, 1.00).lerp(&Color::new(0.5, 0.7, 1.0), t)
}
