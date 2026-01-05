use crate::consts::*;

#[derive(Debug)]
pub struct Point2D(pub u32, pub u32);
impl Point2D {
    pub fn to_canvas_coordinates(&self) -> Point3D {
        todo!();
    }

    pub fn distance(self, p2: &Point2D) -> f64 {
        let p1 = self;
        // Pythagorean formula
        f64::sqrt(
            (u32::pow(p2.0.max(p1.0) - p1.0.min(p2.0), 2)
                + u32::pow(p2.1.max(p1.1) - p1.1.min(p2.1), 2)) as f64,
        )
    }
}

#[derive(Debug)]
pub struct Point3D(pub f64, pub f64, pub f64);
impl Point3D {
    pub fn to_screen_coordinates(&self) -> Point2D {
        let Point3D(x, y, z) = *self;

        // Normalize to the interval [0, 1]
        let x = (x / f64::MAX + 1.0) / 2.0;
        let y = (-y / f64::MAX + 1.0) / 2.0;
        let z = (z / f64::MAX + 1.0) / 2.0;

        assert!(x <= 1.0 && y <= 1.0 && z <= 1.0);

        if self.2 == 0.0 {
            Point2D(
                (x * (SCREEN_WIDTH - 1) as f64) as u32, // Scale to the interval [0, SCREEN_WIDTH)
                (y * (SCREEN_HEIGHT - 1) as f64) as u32, // Scale to the interval [0, SCREEN_HEIGHT)
            )
        } else {
            let x = x / z;
            let y = y / z;

            Point2D(
                (x * (SCREEN_WIDTH - 1) as f64) as u32,
                (y * (SCREEN_HEIGHT - 1) as f64) as u32,
            )
        }
    }

    pub fn distance(self, p2: &Point3D) -> f64 {
        let p1 = self;
        // Pythagorean formula
        f64::sqrt(
            f64::powf(p2.0 - p1.0, 2.0) + f64::powf(p2.1 - p1.1, 2.0) + f64::powf(p2.2 - p1.2, 2.0),
        )
    }
}
