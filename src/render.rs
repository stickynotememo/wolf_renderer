use crate::consts::*;
use crate::scene::Scene;


type Pixel = u32;
type Subpixels = (u8, u8, u8);

fn to_rgb((red, green, blue): Subpixels) -> Pixel {
    let mut pixel: u32 = 0;
    pixel += blue as u32;
    pixel += 256 * green as u32;
    pixel += 256 * 256 * red as u32;

    pixel
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

            for i in [
                -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            ] {
                for j in [
                    -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                ] {
                    let x = x as i32 + i;
                    let y = y as i32 + j;
                    buf[(x + y * STRIDE as i32) as usize] = to_rgb(((t % 256).try_into().unwrap(), 0, 0));
                }
            }
        }
    }
}
