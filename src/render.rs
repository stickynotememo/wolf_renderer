use crate::consts::*;
use crate::scene::Scene;

type Pixel = u32;
type Subpixels = (u8, u8, u8);

fn to_rgb((red, green, blue): Subpixels) -> Pixel {
    let mut pixel: u32 = 0;
    pixel += blue as u32;
    pixel += (green as u32) << 8;
    pixel += (red as u32) << 16;

    pixel
}

fn index_buffer(buf: &mut [u32], x: f64, y: f64) -> &mut u32 {
    let x = x.round_ties_even(); // Banker's round
    let y = y.round_ties_even();

    &mut buf[(x as usize) + (y as usize) * STRIDE as usize]
}

pub fn render(t: u128, scene: &Scene, buf: &mut [u32]) {
    for pix in buf.iter_mut() {
        *pix = to_rgb((0, 0, 0));
    }

    for obj in scene {
        for vertex in &obj.vertices {
            let vertex = vertex.to_screen_coordinates();
            let x = vertex.0;
            let y = vertex.1;

            for i in -10..10 {
                for j in -10..10 {
                    let x = x + i as f64;
                    let y = y + j as f64;
                    *index_buffer(buf, x, y) = to_rgb((255, 255, 255));
                }
            }
        }

        for edge in &obj.edges { // Draw lines
            let start = edge.0.to_screen_coordinates();
            let end = edge.1.to_screen_coordinates();

            if (end.0 - start.0).abs() == 0.0 { // Vertical line case - m is undefined
                for y in (start.1.min(end.1) as u32)..(end.1.max(start.1) + 1.0) as u32 {
                    *index_buffer(buf, end.0, y as f64) = to_rgb((255, 255, 255));
                }
                continue
            }

            let m = (end.1 - start.1) / (end.0 - start.0); // m = rise/run
            let c = start.1 as f64 - m * start.0 as f64; // c = y - mx

            dbg!(start, end, "===");
            for x in (start.0.min(end.0) as usize)..(end.0.max(start.0) as usize) {
                let x = x as f64;
                let y = m * x + c; // y = mx + c
                *index_buffer(buf, x, y) = to_rgb((255, 255, 255));
            }
        }
    }
}
