use core::f64;
use std::time::Duration;
use crate::points::Point3D;
use crate::points::Scene;

pub fn set_scene(dt: Duration, scene: &mut Scene) {
    for obj in scene {
        for vertex in obj.vertices.iter_mut() {
            vertex.z += 1.0;
        }

        obj.update();
    }
}
