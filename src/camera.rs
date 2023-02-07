use rand_distr::UnitSphere;

use crate::*;
pub struct Camera {
    pub origin: glm::Vec3,
    pub lower_left_corner: glm::Vec3,
    pub horizontal: glm::Vec3,
    pub vertical: glm::Vec3,
    pub u: glm::Vec3,
    pub v: glm::Vec3,
    pub w: glm::Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: glm::Vec3,
        look_at: glm::Vec3,
        vup: glm::Vec3,
        vfov: f32,
        aspect_ratio: f32,

        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let view_port_height = 2.0 * h;
        let view_port_width = aspect_ratio * view_port_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * view_port_width * u;
        let vertical = focus_dist * view_port_height * v;

        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(s + t);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk(seed: f32) -> glm::Vec3 {
    let r = &mut StdRng::seed_from_u64(f32_to_unique_u64(seed));
    glm::make_vec3(&UnitSphere.sample(r)).normalize()
}
