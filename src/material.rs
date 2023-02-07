use std::sync::Arc;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{f32_to_unique_u64, random_in_unit_sphere, Color};

pub enum ScatterResponse {
    Scatter(Color, Ray),
    Absorb(Color),
}
use ScatterResponse::*;

pub trait Material {
    // Output: Option<(attenuation, scattered_ray)>
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> ScatterResponse;
}

pub type MaterialObject = Arc<dyn Material + Send + Sync>;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> ScatterResponse {
        let mut scaterred_direction = rec.normal + random_in_unit_sphere(rec.point.sum());
        if scaterred_direction.is_empty() {
            scaterred_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scaterred_direction);
        Scatter(self.albedo, scattered)
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> ScatterResponse {
        let reflected = glm::reflect_vec(&ray_in.dir.normalize(), &rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * random_in_unit_sphere(rec.point.sum()),
        );
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Scatter(self.albedo, scattered)
        } else {
            Absorb(Color::new(0.0, 0.0, 0.0))
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightSource {
    pub albedo: Color,
}

impl LightSource {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LightSource {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> ScatterResponse {
        Absorb(self.albedo)
    }
}

#[derive(Debug, Clone)]
pub struct Dielectic {
    pub refraction_i: f32,
}

impl Dielectic {
    pub const fn new(refraction_i: f32) -> Self {
        Self { refraction_i }
    }
}

impl Material for Dielectic {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> ScatterResponse {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_i
        } else {
            self.refraction_i
        };

        let unit_direction = ray_in.dir.normalize();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let can_reflect = reflectance(cos_theta, refraction_ratio)
            > StdRng::seed_from_u64(f32_to_unique_u64(rec.point.sum())).gen();
        let direction = if cannot_refract || can_reflect {
            glm::reflect_vec(&unit_direction, &rec.normal)
        } else {
            glm::refract_vec(&unit_direction, &rec.normal, refraction_ratio)
        };
        let scattered = Ray::new(rec.point, direction);

        Scatter(attenuation, scattered)
    }
}

pub fn reflectance(cosine: f32, refraction_i: f32) -> f32 {
    // Use shlick's approximation for reflectance
    let r0 = (1.0 - refraction_i) / (1.0 + refraction_i);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
