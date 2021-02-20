mod material;
mod ray_tracing;
mod vec_math;

use std::{fs::File, io::BufWriter, path::Path};

use material::{Diffusor, Material, Reflector, Refractor};
use rand::prelude::*;
use ray_tracing::{Camera, Scene, Sphere};
use vec_math::{random_double_in_interval, Color, Point3, Vec3};

fn clamp(val: f64, bounds: (f64, f64)) -> f64 {
    if val < bounds.0 {
        bounds.0
    } else if val > bounds.1 {
        bounds.1
    } else {
        val
    }
}

fn generate_random_scene(rng: &mut ThreadRng) -> Scene {
    let mut scene = Scene { hittables: vec![] };
    scene.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        std::rc::Rc::new(Diffusor {
            color: Color::new(0.2, 0.2, 0.2),
        }),
    )));

    for a in -11..12 {
        for b in -11..12 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let selector = rng.gen_range(0..5);
                let material: std::rc::Rc<dyn Material> = if selector < 3 {
                    std::rc::Rc::new(Diffusor {
                        color: Color::random(rng),
                    })
                } else if selector < 4 {
                    std::rc::Rc::new(Reflector {
                        color: Color::random_in_interval(rng, (0.5, 1.0)),
                        fuzz_coeff: random_double_in_interval(rng, (0.0, 0.3)),
                    })
                } else {
                    std::rc::Rc::new(Refractor {
                        color: Color::random(rng),
                        fuzz_coeff: random_double_in_interval(rng, (0.0, 0.5)),
                        refr_coeff: random_double_in_interval(rng, (1.1, 1.7)),
                    })
                };
                scene.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }
    scene.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        std::rc::Rc::new(Refractor {
            color: Color::random(rng),
            fuzz_coeff: 0.0,
            refr_coeff: 1.5,
        }),
    )));
    scene.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        std::rc::Rc::new(Diffusor {
            color: Color::new(0.4, 0.2, 0.1),
        }),
    )));
    scene.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        std::rc::Rc::new(Reflector {
            color: Color::new(0.7, 0.6, 0.5),
            fuzz_coeff: 0.0,
        }),
    )));
    scene
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let width = 1200;
    let height = (width as f64 / aspect_ratio).floor() as u32;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vector_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vector_up,
        20.0f64.to_radians(),
        aspect_ratio,
        0.1,
        10.0,
    );
    let depth = 50u32;
    let samples_per_pixel = 500;
    let scale = 1.0 / samples_per_pixel as f64;

    let mut rng = rand::thread_rng();
    let scene = generate_random_scene(&mut rng);

    let mut result_vec: Vec<u8> = vec![];
    result_vec.reserve(width as usize * height as usize * 4);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                color += camera
                    .create_ray(&mut rng, u, v)
                    .color(&mut rng, &scene, depth)
                    * scale;
            }
            result_vec.push((clamp(color.data[0].sqrt(), (0.0, 0.999)) * 256.0) as u8);
            result_vec.push((clamp(color.data[1].sqrt(), (0.0, 0.999)) * 256.0) as u8);
            result_vec.push((clamp(color.data[2].sqrt(), (0.0, 0.999)) * 256.0) as u8);
            result_vec.push(255);
        }
    }
    let path = Path::new(r"image1.png");
    let file = File::create(path).unwrap();
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&result_vec).unwrap();
}
