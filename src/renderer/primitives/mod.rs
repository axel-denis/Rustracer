//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;

use vectors::Vector;
use vectors::resolve_quadratic_equation;
use super::renderer_common::{Transform, Texture};

pub struct Intersection<'a> {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub id: i64,
    pub object: &'a dyn Object,
}

#[derive(Clone)]
pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub id: i64,
}

pub struct Plane {
    pub texture: Texture,
    pub normal: Vector,
    pub distance: f64,
    pub id: i64,
}

pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
    pub id: i64,
}

pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
    pub id: i64,
}

pub trait Object {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn get_id(&self) -> i64;
    fn set_texture(&mut self, new: Texture);
    fn set_radius(&mut self, new: f64);
    fn set_distance(&mut self, new: f64);
    fn set_height(&mut self, new: f64);
    fn set_normal(&mut self, new: Vector);
    fn set_id(&mut self, new: i64);
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());

        if smallest_result == None {
            None
        } else {
            let point = Vector {
                x: origin.x + ray.x * smallest_result.unwrap(),
                y: origin.y + ray.y * smallest_result.unwrap(),
                z: origin.z + ray.z * smallest_result.unwrap(),
            };

            Some ( Intersection {
                normal: point - self.transform.pos,
                intersection_point: point,
                id: self.id,
                object: self,
            })
        }
    }
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn get_id(&self) -> i64 {self.id}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_distance(&mut self, _new: f64) {}
    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_id(&mut self, new: i64) {self.id = new}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn set_transform(&mut self, _new: Transform) {}
    fn get_texture(&self) -> Texture {self.texture}
    fn get_id(&self) -> i64 {self.id}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, _new: f64) {}

    fn set_distance(&mut self, new: f64) {self.distance = new}
    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, new: Vector) {self.normal = new}
    fn set_id(&mut self, new: i64) {self.id = new}
}

impl Object for Cylinder {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn get_id(&self) -> i64 {self.id}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn set_distance(&mut self, _new: f64) {}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_id(&mut self, new: i64) {self.id = new}
}

impl Object for Cone {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn get_id(&self) -> i64 {self.id}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn set_distance(&mut self, _new: f64) {}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_id(&mut self, new: i64) {self.id = new}
}
