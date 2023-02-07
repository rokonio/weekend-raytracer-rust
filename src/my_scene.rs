#![allow(unused)]

use crate::material::*;
use crate::sphere::*;
use crate::*;

// Give a name to the output file. Png is the recommended file format
pub const OUTPUT_NAME: &str = "my_scene_output.png";

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WIDTH: usize = 900;
pub const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
pub const SAMPLE_PER_PIXEL: usize = 200;
pub const MAX_DEPTH: usize = 25;

const MAX: f32 = 0.95;
const DIF: f32 = 0.2;
const MET: f32 = 0.1;
const FUZ: f32 = 0.05;
pub const RED_METAL: Metal = Metal::new(Color::new(MAX, DIF, DIF), FUZ);
pub const GREEN_METAL: Metal = Metal::new(Color::new(DIF, MAX, DIF), FUZ);
pub const BLUE_METAL: Metal = Metal::new(Color::new(DIF, DIF, MAX), FUZ);
pub const METAL: Metal = Metal::new(Color::new(MET, MET, MET), 0.05);

const LMAX: f32 = 0.60;
const LMED: f32 = 0.60;
const LDIF: f32 = 0.60;
pub const LIGHT: LightSource = LightSource::new(Color::new(LMAX, LMED, LDIF));

pub fn init_world_and_camera() {
    let mut r = StdRng::seed_from_u64(SEED);

    let look_from = glm::vec3(0.0, 0.0, 0.0);
    let look_at = glm::vec3(0.0, 0.0, -2.5);
    let v_up = glm::vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - glm::vec3(0.0, 1.0, -3.0)).norm();
    let aperture = 0.25;
    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        90.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    if CAMERA.set(camera).is_err() {
        panic!("Tried to set CAMERA twice. This is a bug");
    }

    let mut world = HittableList::default();
    world.add(Sphere::new(
        glm::vec3(0.0, -100.9, -1.0),
        100.0,
        Arc::new(METAL),
    ));
    // world.add((Sphere::new(
    //     glm::vec3(0.0, 102.9, -1.0),
    //     100.0,
    //     Arc::new(METAL),
    // )));
    for x in -4..5 {
        world.add(Sphere::new(
            ((glm::vec3(x as f32, 0.0, -3.0) - glm::vec3(0.0, 0.0, 0.0)).normalize()
                + glm::vec3(0.0, 0.0, 0.0))
                * (5.0 + 1.5 * r.gen::<f32>()),
            0.5,
            Arc::new(RED_METAL),
        ));
    }
    for x in -4..5 {
        world.add(Sphere::new(
            ((glm::vec3(x as f32, 1.0, -3.0) - glm::vec3(0.0, 0.0, 1.0)).normalize()
                + glm::vec3(0.0, 0.0, 0.0))
                * (5.0 + 1.5 * r.gen::<f32>()),
            0.5,
            Arc::new(GREEN_METAL),
        ));
    }
    for x in -4..5 {
        world.add(Sphere::new(
            ((glm::vec3(x as f32, 2.0, -3.0) - glm::vec3(0.0, 0.0, 2.0)).normalize()
                + glm::vec3(0.0, 0.0, 0.0))
                * (5.0 + 1.5 * r.gen::<f32>()),
            0.5,
            Arc::new(BLUE_METAL),
        ));
    }
    for x in -2..3 {
        let p = ((glm::vec3(x as f32, 0.5, -2.5) - glm::Vec3::zeros()).normalize()
            + glm::Vec3::zeros())
            * (3.5);
        let d = Dielectic::new((5.0 + x as f32) / 2.5);
        world.add(Sphere::new(p, 0.7, Arc::new(d.clone())));
        world.add(Sphere::new(p, -0.60, Arc::new(d)));
    }
    world.add(Sphere::new(
        glm::vec3(0.0, 0.0, 22.0),
        20.0,
        Arc::new(LIGHT),
    ));
    world.add(Sphere::new(glm::vec3(0.0, 1.0, -8.0), 1.5, Arc::new(LIGHT)));
    if WORLD.set(world).is_err() {
        panic!("Tried to set WORLD twice. This is a bug");
    }
}

pub fn sky_color(ray: &Ray, depth: usize) -> Color {
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    // let d = MAX_DEPTH as f32 - MAX_DEPTH as f32;
    // let p = ((unit_dir.x) * 5.0) as i32;
    let s = 0.05;
    //     if p % 3 == 0 {
    //     d
    // } else if p % 3 == 1 {
    //     d / 2.0
    // } else {
    //     0.0
    // };
    Color::new(0.0, 0.00, 0.00).lerp(&Color::new(s, s / 2.0, s / 3.0), t)
}
