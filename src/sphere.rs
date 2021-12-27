use crate::{hittable::*, ray};

pub struct Sphere {
    pub center: glm::Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: glm::Vec3 = ray.origin() - self.center;
        let a = ray.dir().norm_squared();
        let half_b = glm::dot(&ray.dir(), &oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b - sqrtd) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(root);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}

impl Sphere {
    pub fn new(center: glm::Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}
