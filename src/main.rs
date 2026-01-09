mod points;
mod render;
mod scene;

#[cfg(test)]
mod tests;

mod consts {
    pub const SCREEN_WIDTH: usize = 1920;
    pub const SCREEN_HEIGHT: usize = 1080;
    pub const STRIDE: u32 = SCREEN_WIDTH as u32;
}

use points::*;
use render::*;
use scene::*;
use consts::*;

use core::f64; use std::{num::NonZeroU32, u32};

use softbuffer::{Buffer, Context, Rect, Surface};
use winit::{self, event_loop::EventLoop, window::Window};

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

    let mut t: u128 = 1;

    let mut scene = vec![Object {
        vertices: vec![
            // Point3D(0.0, f64::MAX / 2.0, 1.0),
            // Point3D(f64::MAX / 2.0, -f64::MAX / 2.0, 1.0),
            // Point3D(-f64::MAX / 2.0, -f64::MAX / 2.0, 1.0),
            // Point3D(0.0, 0.0, f64::MAX / 2.0),
            // Point3D(0.0, 0.0, 0.0),
            Point3D(f64::MAX / 2.0, f64::MAX / 2.0, 0.0)
        ],
        edges: vec![
            (Point3D(0.0, 0.0, 0.0), Point3D(f64::MAX / 2.0, f64::MAX / 2.0, 0.0))
        ],
    }];

    loop {
        let mut sbuffer = surface.buffer_mut().expect("Couldn't create buffer");
        // set_scene(t, &mut scene);
        render(t, &scene, &mut *sbuffer);
        t += 1;

        sbuffer
            .present_with_damage(&[Rect {
                x: 0,
                y: 0,
                width: NonZeroU32::new(SCREEN_WIDTH as u32).unwrap(),
                height: NonZeroU32::new(SCREEN_HEIGHT as u32).unwrap(),
            }])
            .unwrap();
    }
}
