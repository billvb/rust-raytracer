use std::f32;

use crate::framebuffer;
use crate::geometry::{normalize, Sphere, Vec3f};
use crate::{bar, dot, mul_scalar, vec_add, vec_mul, vec_sub};

pub struct Light {
    pub location: Vec3f,
    pub intensity: f32,
}

impl Light {
    pub fn update_loc(&mut self, new_loc: Vec3f) {
        self.location = vec_add!(self.location, new_loc);
    }
}

pub struct RayTracer {
    pub display: framebuffer::Display,
    pub camera_pos: Vec3f,
    pub camera_vec: Vec3f,
    pub camera_fov: f32,
    pub light: Light,
}

impl RayTracer {
    /// Project a ray out from the camera to a point on the screen, trace what follows.
    fn project_ray(&self, dir: Vec3f, scene: &Vec<Sphere>) -> Option<framebuffer::Pixel> {
        let mut min_range = f32::MAX;
        let mut normal: Option<Vec3f> = None;

        for sphere in scene.iter() {
            if let (Some(dist_normal)) = sphere.intersect(self.camera_pos, dir) {
                if dist_normal.0 < min_range {
                    normal = Some(dist_normal.1);
                    min_range = dist_normal.0;
                }
            }
        }

        match normal {
            Some(surface_normal) => {
                let range_vec: Vec3f = mul_scalar!(self.camera_vec, min_range);

                let pt_intersection: Vec3f = vec_add!(self.camera_pos, range_vec);

                let light_vec = normalize(vec_sub!(pt_intersection, self.light.location));
                let theta = dot!(light_vec, surface_normal);

                if theta <= 0f32 {
                    let ivory: Vec3f = (0.7, 0.9, 0.7);
                    let local_intensity = self.light.intensity * f32::abs(theta);

                    let r = vec_sub!(
                        mul_scalar!(surface_normal, 2f32 * dot!(light_vec, surface_normal)),
                        light_vec
                    );
                    let spec =
                        f32::powf(f32::max(0f32, dot!(r, dir)), 50f32) * self.light.intensity;
                    let new_color = vec_add!(
                        mul_scalar!(ivory, local_intensity * 0.8f32),
                        mul_scalar!((1f32, 1f32, 1f32), spec * 0.3)
                    );

                    let b = new_color.0;
                    let g = new_color.1;
                    let r = new_color.2;

                    assert!(r >= 0f32 && g >= 0f32 && b >= 0f32);

                    Some(
                        (((b * 254.0f32) as u32) << 24
                            | ((g * 254.0f32) as u32) << 16
                            | ((r * 254.0f32) as u32) << 8) as u32,
                    )
                } else {
                    Some(0)
                }
            }
            None => None,
        }
    }

    /// Project all rays from camera to all screen pixels.
    pub fn render_scene(&mut self, scene: &Vec<Sphere>) {
        for y in 0..self.display.height_px {
            for x in 0..self.display.width_px {
                let sx: f32 = (((2.0f32 * (x as f32) + 1.0f32) / self.display.width_px as f32)
                    - 1.0f32)
                    * f32::tan(self.camera_fov / 2.0f32)
                    * self.display.width_px as f32
                    / self.display.height_px as f32;
                let sy: f32 = (((2.0f32 * (y as f32) + 1.0f32) / self.display.height_px as f32)
                    - 1.0f32)
                    * f32::tan(self.camera_fov / 2.0f32)
                    * -1.0f32;

                let dir: Vec3f = normalize((sx, sy, -1.0f32));

                let camera_vec_dot = dot!(self.camera_vec, dir);
                let theta = f32::acos(camera_vec_dot);

                let value = self.project_ray(dir, scene).unwrap_or(0x10101000);
                self.display.setpx(x as usize, y as usize, value);
            }
        }

        self.display.draw();
    }
}
