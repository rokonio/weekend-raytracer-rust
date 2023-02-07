#![allow(unused)]

use crate::material::*;
use crate::sphere::*;
use crate::*;

// Give a name to the output file. Png is the recommended file format
pub const OUTPUT_NAME: &str = "scene_output.png";

// Rendering settings
pub const ASPECT_RATIO: f32 = 3.0 / 2.0;
pub const WIDTH: usize = 1200;
pub const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
pub const SAMPLE_PER_PIXEL: usize = 500;
pub const MAX_DEPTH: usize = 50;

pub const MATERIAL_GROUND: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
// pub const MATERIAL_CENTER: Lambertian = Lambertian::new(Color::new(0.1, 0.2,
// 0.5)); pub const MATERIAL_LEFT: Dielectic = Dielectic::new(1.5);
// pub const MATERIAL_RIGHT: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

// Object creation

pub fn rand_vec(r: &mut impl Rng) -> glm::Vec3 {
    glm::vec3(r.gen(), r.gen(), r.gen())
}

pub fn rand_vec_between(r: &mut impl Rng, min: f32, max: f32) -> glm::Vec3 {
    glm::vec3(
        r.gen_range(min..max),
        r.gen_range(min..max),
        r.gen_range(min..max),
    )
}

pub fn init_world_and_camera() {
    let mut r = StdRng::seed_from_u64(SEED);
    let mut world = HittableList::default();
    world.add(Sphere::new(
        glm::vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(MATERIAL_GROUND),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat: f32 = r.gen();
            let center = glm::vec3(a + 0.9 * r.gen::<f32>(), 0.2, b + 0.9 * r.gen::<f32>());

            if (center - glm::vec3(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: MaterialObject;
                if choose_mat < 0.78 {
                    let albedo = rand_vec(&mut r).component_mul(&rand_vec(&mut r));
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.92 {
                    let albedo = rand_vec_between(&mut r, 0.5, 1.0);
                    let fuzz = r.gen_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    sphere_material = Arc::new(Dielectic::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectic::new(1.5));
    world.add(Sphere::new(glm::vec3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(glm::vec3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(glm::vec3(4.0, 1.0, 0.0), 1.0, material3));

    if WORLD.set(world).is_err() {
        panic!("Tried to set WORLD twice. This is a bug");
    }

    let look_from = glm::vec3(13.0, 2.0, 3.0);
    let look_at = glm::vec3(0.0, 0.0, 0.0);
    let v_up = glm::vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
