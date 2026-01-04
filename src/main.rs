use core::f64;
use std::{num::NonZeroU32, time::Duration, u32};

use softbuffer::{Buffer, Context, Rect, Surface};
use winit::{self, event_loop::EventLoop, window::Window};

const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;
const STRIDE: u32 = SCREEN_WIDTH as u32;
const PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

type Pixel = u32;
type Subpixels = (u8, u8, u8);
struct Point2D(u32, u32);

impl Point2D {
    fn to_canvas_coordinates(&self) -> Point3D {
        todo!();
    }

    fn distance(self, p2: &Point2D) -> f64 {
        let p1 = self;
        // Pythagorean formula
        f64::sqrt((u32::pow(p2.0.max(p1.0) - p1.0.min(p2.0), 2) + u32::pow(p2.1.max(p1.1) - p1.1.min(p2.1), 2)) as f64)
    }
}

#[derive(Debug)]
struct Point3D(f64, f64, f64);
impl Point3D {
    fn to_screen_coordinates(&self) -> Point2D {
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

    fn distance(self, p2: &Point3D) -> f64 {
        let p1 = self;
        // Pythagorean formula
        f64::sqrt(
            f64::powf(p2.0 - p1.0, 2.0) + f64::powf(p2.1 - p1.1, 2.0) + f64::powf(p2.2 - p1.2, 2.0),
        )
    }
}

#[derive(Debug)]
struct Object {
    // Invariants:
    // Every vertex in edges must have a corresponding vertex in vertices
    vertices: Vec<Point3D>,
    edges: Vec<(Point3D, Point3D)>,
}

type Scene = Vec<Object>;

fn to_rgb((red, green, blue): Subpixels) -> Pixel {
    let mut pixel: u32 = 0;
    pixel += blue as u32;
    pixel += 256 * green as u32;
    pixel += 256 * 256 * red as u32;

    pixel
}

fn set_scene(t: u128, scene: &mut Scene) {
    for obj in scene {
        obj.vertices.push(Point3D(t as f64, t as f64, t as f64));
    }
}

fn render(t: u128, scene: &Scene, buf: &mut [u32]) {
    for (idx, pix) in buf.iter_mut().enumerate() {
        *pix = to_rgb((255, 255, 255));

    }
    for obj in scene {
        'i: for vertex in &obj.vertices {
            dbg!(&vertex);
            let vertex = vertex.to_screen_coordinates();
            let x = vertex.0;
            let y = vertex.1;
            dbg!(x, y);
            for i in [-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
                for j in [-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
                    let x = x as i32 + i;
                    let y = y as i32 + j;
                    buf[(x + y * STRIDE as i32) as usize] =  to_rgb((0, 0, 0));
                }
            }
        }
    }
}

fn main() {
    println!("Hello World!");
    let event_loop = EventLoop::new().expect("Couldn't initialise event loop");
    let display = event_loop.owned_display_handle();
    let ctx = Context::new(display).expect("Couldn't create display context");

    // const WINDOW_ATTRIBUTES = WindowAttributes {
    // inner_size
    // };
    let window = event_loop
        .create_window(Window::default_attributes())
        .expect("Couldn't create a window");

    // TODO:
    // let window = event_loop.create_window(WINDOW_ATTRIBUTES);

    let mut surface = Surface::new(&ctx, &window).expect("Couldn't create surface");
    surface
        .resize(
            NonZeroU32::new(SCREEN_WIDTH as u32).unwrap(),
            NonZeroU32::new(SCREEN_HEIGHT as u32).unwrap(),
        )
        .expect("Couldn't resize surface");

    let mut t: u128 = 0;

    let scene = vec![Object {
        vertices: vec![
            Point3D(0.0, f64::MAX / 2.0, 0.0),

            Point3D(f64::MAX / 2.0, -f64::MAX / 2.0, 0.0),
            Point3D(-f64::MAX / 2.0, -f64::MAX / 2.0, 0.0),
            // Point3D(0.0, 0.0, f64::MAX / 2.0),
        ],
        edges: vec![],
    }];

    loop {

        let mut sbuffer = surface.buffer_mut().expect("Couldn't create buffer");
        render(t, &scene, &mut *sbuffer);
        t += 1;

        sbuffer.present_with_damage(&[Rect {
            x: 0,
            y: 0,
            width: NonZeroU32::new(SCREEN_WIDTH as u32).unwrap(),
            height: NonZeroU32::new(SCREEN_HEIGHT as u32).unwrap()
        }]).unwrap();
    }
}

mod tests;
