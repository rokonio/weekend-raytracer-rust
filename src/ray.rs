#[allow(dead_code)]
pub struct Ray {
    pub origin: glm::Vec3,
    pub dir: glm::Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: glm::Vec3, dir: glm::Vec3) -> Ray {
        Ray { origin, dir }
    }
    #[inline]
    pub fn origin(&self) -> glm::Vec3 {
        self.origin
    }
    #[inline]
    pub fn dir(&self) -> glm::Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> glm::Vec3 {
        self.origin + t * self.dir
    }
}