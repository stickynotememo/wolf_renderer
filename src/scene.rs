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
        obj.vertices.push(Point3D(t as f64, t as f64, t as f64));
    }
}
