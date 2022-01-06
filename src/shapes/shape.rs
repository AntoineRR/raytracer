use crate::ray::Ray;
use crate::utils::Vec3;

pub trait Collide {
    fn get_intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

/// Abstraction dedicated to building a shape. Allows to construct a shape providing only the struct instance
/// implementing the `Collide` trait.
///
/// # Example
///
/// ```
/// let shape = ShapeBuilder::new(Box::new(
///     Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)
/// ))
///     .set_position(Vec3::new(1.0, 0.0, -1.0))
///     .set_rotation(Vec3::new(0.0, 1.0, 0.0))
///     .to_shape();
/// ```
pub struct ShapeBuilder {
    position: Vec3,
    rotation: Vec3,
    pub object: Box<dyn Collide>,
}

impl ShapeBuilder {
    pub fn new(object: Box<dyn Collide>) -> Self {
        ShapeBuilder {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            object,
        }
    }

    pub fn set_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn set_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn to_shape(self) -> Shape {
        Shape {
            position: self.position,
            rotation: self.rotation,
            object: self.object,
        }
    }
}

pub struct Shape {
    position: Vec3,
    rotation: Vec3,
    pub object: Box<dyn Collide>,
}

impl Shape {
    pub fn new(position: Vec3, rotation: Vec3, object: Box<dyn Collide>) -> Self {
        Shape {
            position,
            rotation,
            object,
        }
    }
}
