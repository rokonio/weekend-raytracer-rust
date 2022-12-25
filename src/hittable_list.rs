use crate::hittable::{HitRecord, Hittable};

type HittabkeObject = Box<dyn Hittable + Send + Sync>;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<HittabkeObject>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn add(&mut self, object: HittabkeObject) {
        self.objects.push(object);
    }
    pub fn new(objects: Vec<HittabkeObject>) -> Self {
        Self { objects }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut output_rec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(
                ray,
                t_min,
                output_rec.as_ref().map(|r: _| r.t).unwrap_or(t_max),
            ) {
                output_rec = Some(rec);
            }
        }

        output_rec
    }
}
