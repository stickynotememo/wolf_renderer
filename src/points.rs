use std::collections::HashMap;

use crate::consts::*;

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64, pub y: f64, pub z: f64
}

#[derive(Debug)]
pub struct Object {
    // Invariants:
    // Every vertex in edges must have a corresponding vertex in vertices
    pub vertices: Vec<Point3D>,
    pub edges: Vec<(Point3D, Point3D)>,
    pub edge_indices: Vec<(usize, usize)>,
    pub faces: Vec<(Point3D, Point3D, Point3D)>,
    pub face_indices: Vec<(usize, usize, usize)>,
    // the update() function
}

impl Object {
    /// Takes a vector of vertices and a vector of indices for each of the `edge_indices` and
    /// `face_indices` parameters. These indices index the `vertices` array and represent the
    /// endpoints of the line segment (in the case of an edge) or the vertices of the triangle
    /// (in the case of a face)
    pub fn new(vertices: Vec<Point3D>,
            edge_indices: Vec<(usize, usize)>,
            face_indices: Vec<(usize, usize, usize)>) -> Self {

        let mut ret = Self {vertices, edges: vec![], edge_indices, faces: vec![], face_indices};
        for (i_start, i_end) in ret.edge_indices.iter() {
            ret.edges.push((ret.vertices[*i_start].clone(), ret.vertices[*i_end].clone()));
        }

        for (i_one, i_two, i_three /* The three vertices of the triangle */) in ret.face_indices.iter() {
            ret.faces.push((ret.vertices[*i_one].clone(), ret.vertices[*i_two].clone(), ret.vertices[*i_three].clone()));
        }

        ret
    }

    // Updates the `edges` and `faces` to correspond to updated vertices.
    pub fn update(&mut self) {
        self.edges = vec![];
        for (i_start, i_end) in self.edge_indices.iter() {
            self.edges.push((self.vertices[*i_start].clone(), self.vertices[*i_end].clone()));
        }

        self.faces = vec![];
        for (i_one, i_two, i_three /* The three vertices of the triangle */) in self.face_indices.iter() {
            self.faces.push((self.vertices[*i_one].clone(), self.vertices[*i_two].clone(), self.vertices[*i_three].clone()));
        }
    }
}

pub type Scene = Vec<Object>;

impl Point3D {
    pub fn to_screen_coordinates(&self) -> Point2D {
        let Point3D {x, y, z} = *self;

        if z == 0.0 {
            // Normalize to the interval [0, 1]
            let x = (x / f64::MAX + 1.0) / 2.0;
            let y = (-y / f64::MAX + 1.0) / 2.0;
            let z = (z / f64::MAX + 1.0) / 2.0;

            assert!(x <= 1.0 && y <= 1.0 && z <= 1.0);

            Point2D {
                x: x * (SCREEN_WIDTH - 1) as f64, // Scale to the interval [0, SCREEN_WIDTH)
                y: y * (SCREEN_HEIGHT - 1) as f64, // Scale to the interval [0, SCREEN_HEIGHT)
            }
        } else {
            let x = x / z;
            let y = y / z;

            // Normalize to the interval [0, 1]
            let x = (x / f64::MAX + 1.0) / 2.0;
            let y = (-y / f64::MAX + 1.0) / 2.0;
            let z = (z / f64::MAX + 1.0) / 2.0;

            assert!(x <= 1.0 && y <= 1.0 && z <= 1.0);

            Point2D {
                x: x * (SCREEN_WIDTH - 1) as f64,
                y: y * (SCREEN_HEIGHT - 1) as f64,
            }
        }
    }

    pub fn distance(self, p2: &Point3D) -> f64 {
        let p1 = self;
        // Pythagorean formula
        f64::sqrt(
            f64::powf(p2.x - p1.x, 2.0) + f64::powf(p2.y - p1.y, 2.0) + f64::powf(p2.z - p1.z, 2.0) // Fix to use max and min
        )
    }
}

impl Point2D {
    pub fn to_canvas_coordinates(&self) -> Point3D {
        todo!();
    }

    pub fn distance(self, p2: &Point2D) -> f64 {
        let p1 = self;
        // Pythagorean formula
        // TODO: use hypot function
        f64::sqrt(
            (f64::powf(p2.x.max(p1.x) - p1.x.min(p2.x), 2.0)
                + f64::powf(p2.y.max(p1.y) - p1.y.min(p2.y), 2.0)) as f64,
        )
    }
}
