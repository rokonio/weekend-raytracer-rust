use crate::{hittable::*, material::MaterialObject, ray};

pub struct Sphere {
    pub center: glm::Vec3,
    pub radius: f32,
    pub material: MaterialObject,
}

impl Sphere {
    pub const fn new(center: glm::Vec3, radius: f32, material: MaterialObject) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: glm::Vec3 = ray.origin - self.center; // Ray origin to center vector
        let a = ray.dir.norm_squared();
        let half_b = &ray.dir.dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;
        let rec = HitRecord::new_with_front_face(
            ray.at(root),
            root,
            self.material.clone(),
            ray,
            &outward_normal,
        );
        Some(rec)
    }
}
