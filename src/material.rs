use crate::*;
use std::sync::Arc;

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
        let mut scaterred_direction = rec.normal + random_in_unit_sphere();
        if scaterred_direction.is_empty() {
            scaterred_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scaterred_direction);
        Scatter(self.albedo, scattered)
    }
}

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
        let scattered = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Scatter(self.albedo, scattered)
        } else {
            Absorb(Color::new(0.0, 0.0, 0.0))
        }
    }
}

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

        // Listening 58 code
        //
        // let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        // let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // let cannot_refract = refraction_ratio * sin_theta > 1.0;
        // let direction = if cannot_refract {
        //     glm::reflect_vec(&unit_direction, &rec.normal)
        // } else {
        //     glm::refract_vec(&unit_direction, &rec.normal, refraction_ratio)
        // };
        // let scattered = Ray::new(rec.point, direction);

        let refracted = glm::refract_vec(&unit_direction, &rec.normal, refraction_ratio);
        let scattered = Ray::new(rec.point, refracted);
        Scatter(attenuation, scattered)
    }
}
