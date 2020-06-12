use std::thread;
use std::time::Duration;

mod framebuffer;
mod geometry;
mod raytracer;

fn animate_simple_square(disp: &mut framebuffer::Display) {
    for i in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                disp.setpx(x + (i * 10) as usize, y + (i * 5) as usize, 0x0000ff99);
            }
        }
        disp.draw();
        thread::sleep(Duration::from_millis(100));
        disp.zero();
    }
}

fn draw_fade(disp: &mut framebuffer::Display) {
    for i in 0..disp.width_px {
        for j in 0..disp.height_px {
            let r = ((i as f32 / disp.width_px as f32) * 255.0) as u8;
            let g = ((j as f32 / disp.width_px as f32) * 255.0) as u8;
            let b = 255 - (r / 2 + g / 2);
            let px = disp.fromrgb(r, g, b);
            disp.setpx(i as usize, j as usize, px);
        }
    }
    disp.draw();
}

fn main() -> std::io::Result<()> {
    let mut disp = framebuffer::Display {
        width_px: 1920,
        height_px: 1080,
        bit_depth: 32,
        framebuffer: Box::new([0; framebuffer::FRAMEBUFFER_LEN_BYTES]),
    };

    //animate_simple_square(&mut disp);
    draw_fade(&mut disp);

    let mut spheres = Vec::new();
    let gradius = 3000f32;
    for i in 0..12 {
        let theta = 2f32 * std::f32::consts::PI / 12f32;
        let s1 = geometry::Sphere {
            center: (
                -4000f32,
                gradius * f32::sin(theta * i as f32),
                -10000f32 + gradius * f32::cos(theta * i as f32),
            ),
            radius: 350f32,
        };
        spheres.push(s1);
    }

    for i in 0..12 {
        let z = -5000f32 + -15000f32 * i as f32;
        spheres.push(geometry::Sphere {
            center: ( ((i*i) as f32) * 500f32, ((i * i) as f32) * 600f32 - 1000f32, z),
            radius: 700f32,
        });
    }

    let mut raygun = raytracer::RayTracer {
        display: disp.clone(),
        camera_pos: (0.0f32, 0.0f32, 0.0f32),
        camera_vec: (0.0f32, 0.0f32, -1.0f32),
        camera_fov: 3.1415f32 / 3f32,
        light: raytracer::Light {
            location: (0.0f32, 0.0f32, -4000f32),
            intensity: 1.0f32,
        },
    };

    raygun.light.update_loc((300f32, 500f32, -100f32));
    raygun.render_scene(&spheres);

    Ok(())
}
