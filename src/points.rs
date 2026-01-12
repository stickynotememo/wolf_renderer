use crate::consts::*;

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
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

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64, pub y: f64, pub z: f64
}
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
