use crate::ray;

// A trait for every object that can be "hitted" by a ray (i.e. seen on screen)
pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// A struct that keeps informations about a hit point
#[derive(Default)]
pub struct HitRecord {
    pub point: glm::Vec3,
    pub t: f32,
    pub normal: glm::Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: &glm::Vec3) {
        self.front_face = glm::dot(&ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}
