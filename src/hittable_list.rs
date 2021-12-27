use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(&ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.point = temp_rec.point;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}
