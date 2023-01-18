#![allow(unused)]
use crate::*;

pub const ASPECT_RATION: f32 = 16.0 / 9.0;
pub const WIDTH: usize = 1200;
pub const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;
pub const SAMPLE_PER_PIXEL: usize = 100;
pub const MAX_DEPTH: usize = 25;

const MAX: f32 = 0.95;
const DIF: f32 = 0.2;
const MET: f32 = 0.1;
const FUZ: f32 = 0.0;
pub const RED_METAL: Metal = Metal::new(Color::new(MAX, DIF, DIF), FUZ);
pub const GREEN_METAL: Metal = Metal::new(Color::new(DIF, MAX, DIF), FUZ);
pub const BLUE_METAL: Metal = Metal::new(Color::new(DIF, DIF, MAX), FUZ);
pub const METAL: Metal = Metal::new(Color::new(MET, MET, MET), 0.7);

const LMAX: f32 = 1.00;
const LMED: f32 = 0.50;
const LDIF: f32 = 0.0;
pub const LIGHT: LightSource = LightSource::new(Color::new(LMAX, LMED, LDIF));

pub fn init_world_and_camera() {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        glm::vec3(0.0, -100.9, -1.0),
        100.0,
        Arc::new(METAL),
    )));
    world.add(Box::new(Sphere::new(
        glm::vec3(0.0, 102.9, -1.0),
        100.0,
        Arc::new(METAL),
    )));
    for x in -9..9 {
        world.add(Box::new(Sphere::new(
            glm::vec3(x as f32, 0.0, -3.0),
            0.5,
            Arc::new(RED_METAL),
        )));
    }
    for x in -9..9 {
        world.add(Box::new(Sphere::new(
            glm::vec3(x as f32, 1.0, -3.0),
            0.5,
            Arc::new(GREEN_METAL),
        )));
    }
    for x in -9..9 {
        world.add(Box::new(Sphere::new(
            glm::vec3(x as f32, 2.0, -3.0),
            0.5,
            Arc::new(BLUE_METAL),
        )));
    }
    world.add(Box::new(Sphere::new(
        glm::vec3(0.0, 0.0, 22.0),
        20.0,
        Arc::new(LIGHT),
    )));
    // world.add(Box::new(Sphere::new(
    //     glm::vec3(0.0, 0.0, -8.0),
    //     1.5,
    //     Arc::new(LIGHT),
    // )));
    if WORLD.set(world).is_err() {
        panic!("Tried to set WORLD twice. This is a bug");
    }

    let camera = Camera::new();
    if CAMERA.set(camera).is_err() {
        panic!("Tried to set CAMERA twice. This is a bug");
    }
}

pub fn sky_color(ray: &Ray, depth: usize) -> Color {
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    let d = MAX_DEPTH as f32 - depth as f32;
    let p = ((unit_dir.x) * 5.0) as i32;
    let s = if p % 3 == 0 {
        d
    } else if p % 3 == 1 {
        d / 2.0
    } else {
        0.0
    };
    Color::new(0.0, 0.00, 0.00).lerp(&Color::new(s, s, s), t)
}
