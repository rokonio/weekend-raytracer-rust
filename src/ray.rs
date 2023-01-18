#[allow(dead_code)]
pub struct Ray {
    pub origin: glm::Vec3,
    pub dir: glm::Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub const fn new(origin: glm::Vec3, dir: glm::Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> glm::Vec3 {
        self.origin + t * self.dir
    }
}
