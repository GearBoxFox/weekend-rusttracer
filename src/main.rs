use minifb::{Key, Window, WindowOptions};
use rand::rngs::ThreadRng;

pub mod ray;
pub mod vec3;
pub mod hitables;
pub mod camera;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitables::{Sphere, HitList};
use crate::camera::Camera;
use rand::Rng;

// define constant
const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const NS: f32 = 100.0;



// calculate the color seen by a given ray
fn color(ray:Ray, world:&HitList) -> Vec3 {
    // check if we hit a sphere
    match world.hit(&ray, 0.001, f32::MAX) {
        Some(hit) => {
            let target = hit.p + hit.normal + Vec3::random_in_unit_circle(); 
            return 0.5 * color(Ray::new(hit.p, target - hit.p), &world);
        }
        None => {
            let unit_direction = ray.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            return ((1.0 - t) * Vec3::ONE) + (t * Vec3::new(0.5, 0.7, 1.0));
        }
    }
}

// converts a vector into a u32 with ARGB format for minifb
fn vec_to_color(color: Vec3) -> u32 {
    0 << 24 | ((color.x * 255.99) as u32) << 16 | ((color.y * 255.99) as u32) << 8 | ((color.z * 255.99) as u32)
}

fn main() {
    let cam = Camera::new();
    let mut rng:ThreadRng = rand::thread_rng();

    let world:HitList = HitList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
        ]);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "Rusttracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // itterate over every pixel
    let mut index = 0;
    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            // average the color of surrounding pixels for anti-aliasing
            let mut col = Vec3::ZERO; 
            for _ in 0..NS as i32 {
                // find current pixel in regards to screen
                let u:f32 = ((x as f32) + rng.gen::<f32>()) / (WIDTH as f32);
                let v:f32 = ((y as f32) + rng.gen::<f32>()) / (HEIGHT as f32);

                // cast a ray at that pixel and find the color
                let ray  = cam.get_ray(u, v);
                let newcol = color(ray, &world);
                col += newcol;
            }
            // calculate RGB format
            // average of pixel aka anti-aliasing
            col = col / NS as f32; 
            // gamma correct aka 0-1 scale properly
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            buffer[index] = vec_to_color(col);
            index += 1;
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
