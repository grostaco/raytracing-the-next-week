use rand::{thread_rng, Rng};

use crate::hittable::hit_record::HitRecord;

use super::{
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metal {
    pub fuzz: f64,
    pub albedo: Color,
}

pub struct Dielectric {
    ir: f64,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().unit().reflect(&hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );

        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        Some((self.albedo, Ray::new(hit_record.p, scatter_direction)))
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_direction = ray.direction().unit();
        let cos_theta = unit_direction.dot(&-hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let mut rng = thread_rng();

        let direction = if refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(hit_record.p, direction)))
    }
}
