use core::f64;
use crate::points::Point3D;

#[derive(Debug)]
pub struct Object {
    // Invariants:
    // Every vertex in edges must have a corresponding vertex in vertices
    pub vertices: Vec<Point3D>,
    pub edges: Vec<(Point3D, Point3D)>,
}

pub type Scene = Vec<Object>;

pub fn set_scene(t: u128, scene: &mut Scene) {
    for obj in scene {
        obj.vertices[0] = Point3D(f64::MAX / 2.0, f64::MAX / 2.0, 1.0_f64.max(t as f64 / 100.0));
    }
}
