use std::{cmp::Ordering, sync::Arc};

use crate::{
    ray::Ray,
    shapes::collide::{Collide, HitRecord},
    utils::Vec3,
};

/// Axis Aligned Bounding Box.
/// It is defined by its lowest and highest corners
#[derive(Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    /// Creates a new AABB
    pub fn new(min: Vec3, max: Vec3) -> Self {
        let e = 0.000001;
        let diff = max - min;
        let mut min = min;
        let mut max = max;
        if diff.x < e {
            min.x -= e;
            max.x += e;
        } else if diff.y < e {
            min.y -= e;
            max.y += e;
        } else if diff.z < e {
            min.z -= e;
            max.z += e;
        }
        AABB { min, max }
    }

    /// Returns if the AABB was hit
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let (mut min, mut max) = (t_min, t_max);
        for i in 0..3 {
            let (t0, t1) = get_interval(self.min[i], self.max[i], ray.origin[i], ray.direction[i]);
            min = t0.max(min);
            max = t1.min(max);
            if max <= min {
                return false;
            }
        }
        return true;
    }

    fn get_longer_axis(&self) -> usize {
        let mut max = 0.0;
        let mut axis = 0;
        for a in 0..3 {
            let diff = self.max[a] - self.min[a];
            if diff > max {
                axis = a;
                max = diff;
            }
        }
        axis
    }
}

fn get_interval(min: f64, max: f64, origin: f64, direction: f64) -> (f64, f64) {
    if direction == 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }
    let t0 = (min - origin) / direction;
    let t1 = (max - origin) / direction;
    let (t0, t1) = (t0.min(t1), t0.max(t1));
    (t0, t1)
}

pub fn surrounding_box(a: AABB, b: AABB) -> AABB {
    let lowest = Vec3::new(
        a.min.x.min(b.min.x),
        a.min.y.min(b.min.y),
        a.min.z.min(b.min.z),
    );
    let highest = Vec3::new(
        a.max.x.max(b.max.x),
        a.max.y.max(b.max.y),
        a.max.z.max(b.max.z),
    );
    AABB::new(lowest, highest)
}

type ArcCollide = Arc<dyn Collide + Send + Sync>;

pub fn get_bounding_box<T>(objects: &[Arc<T>]) -> AABB
where
    T: ?Sized,
    T: Collide + Send + Sync
{
    if objects.len() < 1 {
        panic!("Please provide a vector with at least one element");
    }
    let mut aabb = objects[0].get_bounding_box().unwrap();
    for i in 1..objects.len() {
        aabb = surrounding_box(aabb, objects[i].get_bounding_box().unwrap());
    }
    aabb
}

fn box_compare_on(axis: usize) -> Box<dyn FnMut(&ArcCollide, &ArcCollide) -> Ordering> {
    Box::new(move |a: &ArcCollide, b: &ArcCollide| {
        if a.get_bounding_box().unwrap().min[axis] < b.get_bounding_box().unwrap().min[axis] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}

/// Bounding Volume Hierarchy.
/// Tree like structure to divide the scene into AABB and speed up ray/object intersection calculations.
pub struct BVH {
    left: ArcCollide,
    right: ArcCollide,
    aabb: AABB,
}

impl Collide for BVH {
    fn get_intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.get_intersection(ray, t_min, t_max);
        let t_max = if hit_left.is_some() {
            hit_left.as_ref().unwrap().t
        } else {
            t_max
        };
        let hit_right = self.right.get_intersection(ray, t_min, t_max);

        if hit_right.is_some() {
            return hit_right;
        } else if hit_left.is_some() {
            return hit_left;
        } else {
            return None;
        }
    }

    fn get_bounding_box(&self) -> Option<AABB> {
        Some(self.aabb)
    }
}

impl BVH {
    pub fn new(objects: &mut Vec<ArcCollide>, start: usize, end: usize) -> Self {
        // Choose an axis to separate the objects in two groups
        let aabb = get_bounding_box(&objects[start..end]);
        let axis = aabb.get_longer_axis();
        // Sort the objects based on the axis we chose
        let comparator = box_compare_on(axis);
        objects[start..end].sort_by(comparator);

        // Take care of the leafs of our tree
        if end - start == 1 {
            return BVH {
                left: objects[start].clone(),
                right: objects[start].clone(),
                aabb,
            };
        } else if end - start == 2 {
            return BVH {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                aabb,
            };
        }

        let mid = (end + start) / 2;

        // Separate our objects in two groups and calculate the BVH for those two groups
        BVH {
            left: Arc::new(BVH::new(objects, start, mid)),
            right: Arc::new(BVH::new(objects, mid, end)),
            aabb,
        }
    }
}
