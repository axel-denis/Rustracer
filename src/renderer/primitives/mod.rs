//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;
use vectors::VectorF;
use vectors::Point;
use vectors::resolve_quadratic_equation;

pub trait Object {
    fn intersection(&self, ray: VectorF) -> bool;
}

#[derive(Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f64,
}

pub struct Plan {
    origin: Point,
    endPoint: Point,
}

impl Sphere {
    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

impl Plan {
    pub fn set_origin(&mut self, point: Point) {
        self.origin = point;
    }
    pub fn set_endPoint(&mut self, point: Point) {
        self.endPoint = point;
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: VectorF) -> bool {
        let result = resolve_quadratic_equation(ray.direction.x.powf(2.0) + ray.direction.y.powf(2.0) + ray.direction.z.powf(2.0),
                                   2.0 * (ray.direction.x * (ray.origin.x - self.origin.x) + ray.direction.y * (ray.origin.y - self.origin.y) + ray.direction.z * (ray.origin.z - self.origin.z)),
                                   ((ray.origin.x - self.origin.x).powf(2.0) + (ray.origin.y - self.origin.y).powf(2.0) + (ray.origin.z - self.origin.z).powf(2.0)) - self.radius.powf(2.0));
        if result.is_empty() {
            false
        } else {
            println!("Sphere: {:?} {:?}", self, ray);
            println!("result: {:?}", result);
            true
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: VectorF) -> bool {
        return true;
    }
}
