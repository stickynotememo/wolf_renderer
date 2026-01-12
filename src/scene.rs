use core::f64;
use crate::points::Point3D;

#[derive(Debug)]
pub struct Object {
    // Invariants:
    // Every vertex in edges must have a corresponding vertex in vertices
    pub vertices: Vec<Point3D>,
    pub edges: Vec<(Point3D, Point3D)>,
    pub faces: Vec<(Point3D, Point3D, Point3D)>,
}

pub type Scene = Vec<Object>;

pub fn set_scene(t: u128, scene: &mut Scene) {
    for obj in scene {
        for vertex in obj.vertices.iter_mut() {
            vertex.z += 1.0;
        }

        for edge in obj.edges.iter_mut() {
            edge.0.z += 1.0;
            edge.1.z += 1.0;
        }
    }
}
