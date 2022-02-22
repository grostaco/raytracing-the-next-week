use common::{
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};
use rand::{thread_rng, Rng};
use std::{
    f64::INFINITY,
    fs::File,
    io::{self, Write},
    rc::Rc,
};
use view::Camera;

mod common;
mod hittable;
mod view;

#[inline]
fn random_double<R: Rng>(mut rng: R) -> f64 {
    rng.gen_range(0.0..1.0)
}

fn random_scene() -> HittableList {
    let mut rng = thread_rng();
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(&mut rng);
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(&mut rng),
                0.2,
                b as f64 + 0.9 * random_double(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    let mat = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(&mut rng, 0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let mat = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                };
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(ray, 0.001, INFINITY) {
        if let Some((attentuation, scattered)) = rec.material.scatter(ray, &rec) {
            return ray_color(&scattered, world, depth - 1) * attentuation;
        }
        return Color::zeros();
    }
    let unit_dir = ray.direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut image = File::create("image.ppm").unwrap();
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::zeros();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    write!(image, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    let mut rng = thread_rng();
    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}  ", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let mut pixel = Color::zeros();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel = pixel + ray_color(&r, &world, max_depth);
            }

            pixel.write_color(&mut image, samples_per_pixel).unwrap();
        }
    }
}
