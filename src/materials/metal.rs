use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    vec3::{random_in_unit_sphere, reflect, Color},
};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> (Ray, Color) {
        let reflected_direction = reflect(&r_in.direction(), &hitrecord.normal);

        let scattered_ray = Ray::new(
            hitrecord.p,
            reflected_direction + self.fuzz * random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        (scattered_ray, attenuation)
    }
}
