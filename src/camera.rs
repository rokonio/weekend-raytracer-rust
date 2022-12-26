use crate::ray;

pub struct Camera {
    pub origin: glm::Vec3,
    pub lower_left_corner: glm::Vec3,
    pub horizontal: glm::Vec3,
    pub vertical: glm::Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATION: f32 = 16.0 / 9.0;
        const VIEW_PORT_HEIGHT: f32 = 2.0;
        const VIEW_PORT_WIDTH: f32 = ASPECT_RATION * VIEW_PORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        const ORIGIN: glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);
        const HORIZONTAL: glm::Vec3 = glm::Vec3::new(VIEW_PORT_WIDTH, 0.0, 0.0);
        const VERTICAL: glm::Vec3 = glm::Vec3::new(0.0, VIEW_PORT_HEIGHT, 0.0);

        let lower_left_corner =
            ORIGIN - HORIZONTAL / 2. - VERTICAL / 2. - glm::Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Camera {
            origin: ORIGIN,
            lower_left_corner,
            horizontal: HORIZONTAL,
            vertical: VERTICAL,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        ray::Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
