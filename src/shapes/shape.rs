use crate::material::{Diffuse, Material};
use crate::ray::Ray;
use crate::utils::{Color, Vec3};

pub trait Collide {
    fn get_intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, outward_normal: Vec3, t: f32, front_face: bool) -> Self {
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
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
    object: Box<dyn Collide>,
    material: Box<dyn Material>,
}

impl ShapeBuilder {
    /// Creates a new ShapeBuilder
    pub fn new(object: Box<dyn Collide>) -> Self {
        ShapeBuilder {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            object,
            material: Box::new(Diffuse::new(Color::new(150, 150, 150))),
        }
    }

    /// Set the position of the ShapeBuilder
    pub fn set_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    /// Set the rotation of the ShapeBuilder
    pub fn set_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    /// Set the material of the ShapeBuilder
    pub fn set_material(mut self, material: Box<dyn Material>) -> Self {
        self.material = material;
        self
    }

    /// Converts the ShapeBuilder into a usable Shape
    pub fn to_shape(self) -> Shape {
        Shape {
            position: self.position,
            rotation: self.rotation,
            object: self.object,
            material: self.material,
        }
    }
}

/// A shape to add to a Scene for rendering.
/// Use the ShapeBuilder struct to generate Shapes more easily and with default values for some fields.
pub struct Shape {
    position: Vec3,
    rotation: Vec3,
    pub object: Box<dyn Collide>,
    pub material: Box<dyn Material>,
}

impl Shape {
    /// Creates a new Shape
    pub fn new(
        position: Vec3,
        rotation: Vec3,
        object: Box<dyn Collide>,
        material: Box<dyn Material>,
    ) -> Self {
        Shape {
            position,
            rotation,
            object,
            material,
        }
    }
}
