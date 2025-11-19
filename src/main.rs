#![feature(more_qualified_paths)]
use std::fs::File;
use std::time::Duration;
use std::{ffi::CString, thread::sleep};
use std::os::fd::BorrowedFd;

use memmap2::{MmapMut, MmapOptions};

use wayland_client::{
    Connection, Dispatch, EventQueue, Proxy, backend::ObjectId, delegate_noop, protocol::{
        wl_buffer::{self, WlBuffer},
        wl_compositor::{self, WlCompositor},
        wl_registry, wl_shell,
        wl_shm::{self, Format, WlShm},
        wl_shm_pool::{self, WlShmPool},
        wl_surface::{self, Event, WlSurface},
    }
};

use libc::{O_CREAT, O_RDWR, PROT_READ, PROT_WRITE, S_IROTH, S_IRWXU, ftruncate, mmap, shm_open};

use wayland_protocols::xdg::{
    self,
    shell::client::{
        xdg_surface::{self, XdgSurface},
        xdg_toplevel::XdgToplevel,
        xdg_wm_base::{self, XdgWmBase},
    },
};

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

#[derive(Debug)]
struct State {
    exit: bool,
    _ready: bool,
    compositor: Option<WlCompositor>,
    surface: Option<WlSurface>,
    shm: Option<WlShm>,
    formats: Vec<Format>,
    buffer: Option<WlBuffer>,
    wm_base: Option<XdgWmBase>,
    xdg_surface: Option<XdgSurface>,
    toplevel_surface: Option<XdgToplevel>,
    pool: Option<WlShmPool>,
    memory_map: Option<MmapMut>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        // Iterate over the globals given by the registry.
        // Use the details from the global (name & version) to bind the proxy to the state
        // variable, specifying which global is beiong bound in the turbofish of proxy.bind()
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_compositor" => {
                    let compositor = proxy
                        .bind::<wl_compositor::WlCompositor, _, _>(name, version, qhandle, *data);
                    let surface = compositor.create_surface(qhandle, *data);
                    state.compositor = Some(compositor);
                    state.surface = Some(surface);
                }
                "wl_shm" => {
                    let shm = proxy.bind::<wl_shm::WlShm, _, _>(name, version, qhandle, *data);
                    state.shm = Some(shm);
                }
                "xdg_wm_base" => {
                    let wm_base =
                        proxy.bind::<xdg_wm_base::XdgWmBase, _, _>(name, version, qhandle, *data);
                    state.wm_base = Some(wm_base);
                }
                _ => {
                    // do nothing
                }
            };
        }
    }
}

impl Dispatch<WlShm, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlShm,
        event: <WlShm as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let <WlShm as Proxy>::Event::Format { format } = event {
            if let None = state.pool {
                state.formats.push(
                    format
                        .into_result()
                        .expect("Couldn't understand format, or it was unknown"),
                );
                // Any wayland client must support Xrgb8888 and Argb8888
                let channels: i32 = if state.formats.contains(&Format::Xrgb8888) {
                    4
                } else {
                    4
                }; // we know that the supported formats are only the two above, so we can safely assume that any format which is not Xrgb8888 is Argb8888, requiring 4 channels.
                   // FIX: 3 channels won't work, even with Xrgb8888 for some reason
                   // TODO: Update for more channels

                let size: i32 = (WIDTH * HEIGHT * channels * 2).try_into().unwrap();
                let name = CString::new("/wolf_renderer_buffer").expect("CString::new() failed"); // CString::new can only fail if a null byte is in the string. This string should never cause errors

                let shm_fd = unsafe {
                    // SAFETY: We know that that at least a few megabytes of memory
                    // should be able to be allocated for the virtual buffer.
                    // Data races should not be possible as the compositor will
                    // not modify the buffer. (Non owners do not have write
                    // permissions)
                    //
                    // TODO: Use shared_memory and implement FD return

                    let fildes = shm_open(name.as_ptr(), O_RDWR | O_CREAT, S_IRWXU | S_IROTH);
                    if fildes <= 0 {
                        panic!("Could not open SHM object. Error code: N/A") // TODO: implement errno reading
                    };
                    ftruncate(fildes, size.into());
                    BorrowedFd::borrow_raw(fildes)
                };

                let mut shm_file = File::from(shm_fd.try_clone_to_owned().unwrap()); // Converts borrowed
                // to owned. Are there problems with this?

                let mut mmap = unsafe {
                    // SAFETY: This effectively creates a memory map from a raw
                    // pointer. As above, as long as the compositor does not
                    // mutate the underlying shared memory pool, mutations
                    // should be safe and not cause data races [undefined.race]
                    // https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias
                    MmapOptions::new()
                        .len(size as usize)
                        .map_mut(&shm_file)
                        .unwrap()
                };

                state.memory_map = Some(mmap);
                let shm_pool = proxy.create_pool(shm_fd, size.try_into().unwrap(), qhandle, *data);
                // TODO: Move drawing logic to event loop (double flush)
                state.pool = Some(shm_pool);
            };
        } else {
            unimplemented!()
        }
    }
}

impl Dispatch<WlBuffer, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlBuffer,
        event: <WlBuffer as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<XdgWmBase, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &XdgWmBase,
        event: <XdgWmBase as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            state.wm_base.as_ref().unwrap().pong(serial); // Ok to unwrap as a ping won't be recieved until
                                                          // all the globals are bound (including xdg_wm_base)
                                                          // TODO: figure out as_ref
        } else {
            unreachable!();
        };
    }
}

impl Dispatch<XdgSurface, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &XdgSurface,
        event: <XdgSurface as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let <XdgSurface as Proxy>::Event::Configure { serial } = event {
            proxy.ack_configure(serial);
        }
    }
}

impl Dispatch<XdgToplevel, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &XdgToplevel,
        event: <XdgToplevel as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let <XdgToplevel as Proxy>::Event::Configure {
            width,
            height,
            states,
        } = event
        {
            // state.xdg_surface.unwrap().ack_configure(serial);
            // FIX: Figure out what serial I need for ack_configure with a toplevel
            // xdg_surface
        }
    }
}

impl Dispatch<WlSurface, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlSurface,
        event: <WlSurface as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}
delegate_noop!(State: wl_shm_pool::WlShmPool);
delegate_noop!(State: wl_compositor::WlCompositor);

fn main() {
    let Ok(conn) = Connection::connect_to_env() else {
        panic!("Couldn't connect to Wayland environment");
    };

    let mut state = State {
        exit: false,
        _ready: false,
        formats: vec![],
        compositor: None,
        surface: None,
        shm: None,
        buffer: None,
        wm_base: None,
        xdg_surface: None,
        toplevel_surface: None,
        pool: None,
        memory_map: None
    };

    let mut event_queue: EventQueue<State> = conn.new_event_queue();

    let display = conn.display();
    let qh = &event_queue.handle();

    display.get_registry(&qh, ());
    while !state.exit {
        event_queue.roundtrip(&mut state).unwrap();
        event_queue.flush().expect("hopefully flushed"); // TODO: expect
        // Assign roles to the surface as required, then attach the buffer, damage and commit
        if let None = &state.toplevel_surface {
            if let (Some(base), Some(surface), Some(buffer)) =
                (&state.wm_base, &state.surface, &state.buffer)
            {
                // Buffer is required because it is to be attached to the surface.
                let xdg_surface = base.get_xdg_surface(surface, qh, ()); // HACK: udata =/ ()
                let toplevel_surface = xdg_surface.get_toplevel(qh, ()); // HACK: udata =/ ()

                state.xdg_surface = Some(xdg_surface);
                state.toplevel_surface = Some(toplevel_surface);

                surface.attach(Some(buffer), 0, 0);
                surface.damage(0, 0, WIDTH, HEIGHT);
                surface.commit();
            };
        }

        if let Some(shm_pool) = &state.pool {
            let format = if state.formats.contains(&Format::Xrgb8888) {
                Format::Xrgb8888
            } else {
                Format::Argb8888
            };

            let channel_count = match format {
                Format::Xrgb8888 => 4,
                Format::Argb8888 => 4,
                _ => unimplemented!("Other formats not supported")
            };
               // FIX: 3 channels won't work, even with Xrgb8888 for some reason
               // TODO: Update for more channels

            let size: i32 = (WIDTH * HEIGHT * channel_count * 2).try_into().unwrap();

            let buffer = shm_pool.create_buffer(
                0,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                (WIDTH * channel_count).try_into().unwrap(),
                if state.formats.contains(&Format::Xrgb8888) {
                    Format::Xrgb8888
                } else {
                    Format::Argb8888
                },
                qh,
                (),
            );
            state.buffer = Some(buffer.clone());
        }

        state._ready = if let (Some(_), Some(_), Some(_), Some(_), Some(_), Some(mmap)) = (
            &state.compositor,
            &state.surface,
            &state.shm,
            &state.buffer,
            &state.wm_base,
            &mut state.memory_map,
        ) {
            draw(mmap);
            true
        } else {
            false
        };
    }
}

fn draw(memory_map: &mut MmapMut) { // TODO: WlCallback on frame
    let mut pixel: u64 = 0;

    for byte in &mut **memory_map {
        let pixel_offset = (pixel % 4) as u8;
        pixel += 1;

        *byte = match pixel_offset {
            1 => 0,
            _ => 255
        };
    };
}
