use std::sync::Arc;

use crate::{hittable::HitRecord, random_in_unit_sphere, ray::Ray, Color};

pub trait Material {
    // Output: Option<(attenuation, scattered_ray)>
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub type MaterialObject = Arc<dyn Material + Send + Sync>;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scaterred_direction = rec.normal + random_in_unit_sphere();
        if scaterred_direction.is_empty() {
            scaterred_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scaterred_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = glm::reflect_vec(&ray_in.dir.normalize(), &rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
