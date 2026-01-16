use crate::consts::*;
use crate::points::{Point2D, Point3D};
use crate::Scene;

type Pixel = u32;
type Subpixels = (u8, u8, u8);

fn to_rgb((red, green, blue): Subpixels) -> Pixel {
    let mut pixel: u32 = 0;
    pixel += blue as u32;
    pixel += (green as u32) << 8;
    pixel += (red as u32) << 16;

    pixel
}

fn index_buffer(buf: &mut [u32], Point2D {x, y}: Point2D) -> &mut u32 {
    let x = x.round_ties_even(); // Banker's round
    let y = y.round_ties_even();

    &mut buf[(x as usize) + (y as usize) * STRIDE as usize]
}

pub fn render(scene: &Scene, buf: &mut [u32]) {
    for pix in buf.iter_mut() {
        *pix = to_rgb((0, 0, 0));
    }

    for obj in scene {
        for vertex in &obj.vertices {
            let vertex = vertex.to_screen_coordinates();
            let x = vertex.x;
            let y = vertex.y;

            for i in -10..=10 {
                for j in -10..=10 {
                    let x = x + i as f64;
                    let y = y + j as f64;
                    *index_buffer(buf, Point2D { x, y }) = to_rgb((255, 255, 255));
                }
            }
        }

        for edge in &obj.edges { // Draw lines
            let start = edge.0.to_screen_coordinates();
            let end = edge.1.to_screen_coordinates();

            if (end.x - start.x).abs() == 0.0 { // Vertical line case - m is undefined
                for y in (start.y.min(end.y) as u32)..(end.y.max(start.y) + 1.0) as u32 {
                    *index_buffer(buf, Point2D { x: end.x, y: y as f64 }) = to_rgb((255, 255, 255));
                }
                continue
            }

            let m = (end.y - start.y) / (end.x - start.x); // m = rise/run
            let c = start.y as f64 - m * start.x as f64; // c = y - mx

            for x in (start.x.min(end.x) as usize)..=(end.x.max(start.x) as usize) {
                let x = x as f64;
                let y = m * x + c; // y = mx + c
                *index_buffer(buf, Point2D { x, y }) = to_rgb((255, 255, 255));
            }
        }


        for (p0, p1, p2) in &obj.triangles {

            let (p0, p1, p2) = (p0.to_screen_coordinates(), p1.to_screen_coordinates(), p2.to_screen_coordinates());
            // Finds the leftmost and rightmost points on the triangle and only iterates over them.
            let horizontal_range = p0.x.min(p1.x.min(p2.x)) as usize..=p0.x.max(p1.x.max(p2.x)) as usize;
            let vertical_range = p0.y.min(p1.y.min(p2.y)) as usize..=p0.y.max(p1.y.max(p2.y)) as usize;

            for x in horizontal_range.clone() {
                for y in vertical_range.clone() {
                    let p = Point2D{x: x as f64, y: y as f64};

                    // adapted from https://stackoverflow.com/a/14382692
                    let area = 0.5 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
                    let s = 1.0/(2.0 * area) * (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * p.x + (p0.x - p2.x) * p.y);
                    let t = 1.0/(2.0 * area) * (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * p.x + (p1.x - p0.x) * p.y);

                    if s > 0.0 && t > 0.0 && 1.0 - s - t > 0.0 {
                        *index_buffer(buf, p) = to_rgb((255, 255, 255));
                    };
                }
            }
            
        }

    }
}
