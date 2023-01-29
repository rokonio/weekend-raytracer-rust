use crate::ray::Ray;
pub struct Camera {
    pub origin: glm::Vec3,
    pub lower_left_corner: glm::Vec3,
    pub horizontal: glm::Vec3,
    pub vertical: glm::Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let view_port_height = 2.0 * h;
        let view_port_width = aspect_ratio * view_port_height;
        const FOCAL_LENGTH: f32 = 1.0;

        const ORIGIN: glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);
        let horizontal: glm::Vec3 = glm::Vec3::new(view_port_width, 0.0, 0.0);
        let vertical: glm::Vec3 = glm::Vec3::new(0.0, view_port_height, 0.0);

        let lower_left_corner =
            ORIGIN - horizontal / 2. - vertical / 2. - glm::Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Camera {
            origin: ORIGIN,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
