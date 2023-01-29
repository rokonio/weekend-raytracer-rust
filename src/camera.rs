use crate::ray::Ray;
pub struct Camera {
    pub origin: glm::Vec3,
    pub lower_left_corner: glm::Vec3,
    pub horizontal: glm::Vec3,
    pub vertical: glm::Vec3,
}

impl Camera {
    pub fn new(
        look_from: glm::Vec3,
        look_at: glm::Vec3,
        vup: glm::Vec3,
        vfov: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let view_port_height = 2.0 * h;
        let view_port_width = aspect_ratio * view_port_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = view_port_width * u;
        let vertical = view_port_height * v;

        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
