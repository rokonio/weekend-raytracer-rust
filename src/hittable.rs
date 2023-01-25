use crate::material::MaterialObject;
use crate::ray::Ray;

// A trait for every object that can be "hitted" by a ray (i.e. seen on screen)
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// A struct that keeps informations about a hit point
pub struct HitRecord {
    pub point: glm::Vec3,
    pub t: f32,
    pub normal: glm::Vec3,
    pub front_face: bool,
    pub material: MaterialObject,
}

impl HitRecord {
    pub fn new_with_front_face(
        point: glm::Vec3,
        t: f32,
        material: MaterialObject,
        ray: &Ray,
        outward_normal: &glm::Vec3,
    ) -> HitRecord {
        let front_face = glm::dot(&ray.dir, outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            t,
            material,
            front_face,
            normal,
        }
    }
}
