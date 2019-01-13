extern crate andrew;
extern crate smithay_client_toolkit as sctk;

use andrew::line;
use andrew::shapes::rectangle;
use andrew::text;
use andrew::text::fontconfig;

use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use sctk::keyboard::{map_keyboard_auto, Event as KbEvent, KeyState};
use sctk::utils::{DoubleMemPool, MemPool};
use sctk::window::{ConceptFrame, Event as WEvent, Window};
use sctk::Environment;

use sctk::reexports::client::protocol::wl_compositor::RequestsTrait as CompositorRequests;
use sctk::reexports::client::protocol::wl_surface::RequestsTrait as SurfaceRequests;
use sctk::reexports::client::protocol::{wl_shm, wl_surface};
use sctk::reexports::client::{Display, Proxy};

fn main() {
    let (display, mut event_queue) =
        Display::connect_to_env().expect("Failed to connect to the wayland server.");
    let env = Environment::from_display(&*display, &mut event_queue).unwrap();

    let seat = env
        .manager
        .instantiate_auto(|seat| seat.implement(|_, _| {}, ()))
        .unwrap();

    // we need a window to receive things actually
    let mut dimensions = (320u32, 240u32);
    let surface = env
        .compositor
        .create_surface(|surface| surface.implement(|_, _| {}, ()))
        .unwrap();

    let next_action = Arc::new(Mutex::new(None::<WEvent>));

    let waction = next_action.clone();
    let mut window = Window::<ConceptFrame>::init_from_env(&env, surface, dimensions, move |evt| {
        let mut next_action = waction.lock().unwrap();
        // Keep last event in priority order : Close > Configure > Refresh
        let replace = match (&evt, &*next_action) {
            (_, &None)
            | (_, &Some(WEvent::Refresh))
            | (&WEvent::Configure { .. }, &Some(WEvent::Configure { .. }))
            | (&WEvent::Close, _) => true,
            _ => false,
        };
        if replace {
            *next_action = Some(evt);
        }
    })
    .expect("Failed to create a window !");

    window.new_seat(&seat);

    let mut pools = DoubleMemPool::new(&env.shm, || {}).expect("Failed to create a memory pool !");

    let _keyboard = map_keyboard_auto(&seat, move |event: KbEvent, _| match event {
        KbEvent::Key {
            state,
            utf8: Some(text),
            ..
        } => if text == "p" && state == KeyState::Pressed {},
        _ => (),
    });

    if !env.shell.needs_configure() {
        // initial draw to bootstrap on wl_shell
        if let Some(pool) = pools.pool() {
            redraw(pool, window.surface(), dimensions);
        }
        window.refresh();
    }

    loop {
        match next_action.lock().unwrap().take() {
            Some(WEvent::Close) => break,
            Some(WEvent::Refresh) => {
                window.refresh();
                window.surface().commit();
            }
            Some(WEvent::Configure { new_size, .. }) => {
                if let Some((w, h)) = new_size {
                    window.resize(w, h);
                    dimensions = (w, h)
                }
                window.refresh();
                if let Some(pool) = pools.pool() {
                    redraw(pool, window.surface(), dimensions);
                }
            }
            None => {}
        }

        display.flush().unwrap();

        event_queue.dispatch().unwrap();
    }
}

fn redraw(pool: &mut MemPool, surface: &Proxy<wl_surface::WlSurface>, (buf_x, buf_y): (u32, u32)) {
    // resize the pool if relevant
    pool.resize((4 * buf_x * buf_y) as usize)
        .expect("Failed to resize the memory pool.");
    let mut buf: Vec<u8> = vec![0; 4 * buf_x as usize * buf_y as usize];
    let mut canvas =
        andrew::Canvas::new(&mut buf, buf_x as usize, buf_y as usize, 4 * buf_x as usize);
    let background = rectangle::Rectangle::new(
        (0, 0),
        (buf_x as usize, buf_y as usize),
        None,
        Some([0, 0, 0, 255]),
    );
    let rectangle = rectangle::Rectangle::new(
        (0, 0),
        (150, 150),
        Some((15, [0, 0, 255, 255], rectangle::Sides::ALL, Some(10))),
        Some([0, 255, 0, 255]),
    );
    let line = line::Line::new((200, 20), (250, 100), [255, 0, 0, 255], true);
    let mut font_data = Vec::new();
    ::std::fs::File::open(
        fontconfig::FontConfig::new()
            .unwrap()
            .get_regular_family_fonts("sans")
            .unwrap()
            .get(0)
            .unwrap(),
    )
    .unwrap()
    .read_to_end(&mut font_data)
    .unwrap();
    let mut text = text::Text::new(
        (63, 69),
        [0, 0, 0, 255],
        font_data,
        12.0,
        2.0,
        "hello world",
    );
    text.pos = (75 - (text.get_width() / 2), 69);
    let text_box = rectangle::Rectangle::new(
        (text.pos.0 - 3, text.pos.1),
        (text.get_width() + 6, 12),
        Some((1, [0, 0, 255, 255], rectangle::Sides::ALL, None)),
        None,
    );

    canvas.draw(&background);
    canvas.draw(&rectangle);
    canvas.draw(&line);
    canvas.draw(&text_box);
    canvas.draw(&text);

    let _ = pool.seek(SeekFrom::Start(0));
    {
        let mut writer = BufWriter::new(&mut *pool);
        writer.write(canvas.buffer).unwrap();
        let _ = writer.flush();
    }

    // get a buffer and attach it
    let new_buffer = pool.buffer(
        0,
        buf_x as i32,
        buf_y as i32,
        4 * buf_x as i32,
        wl_shm::Format::Argb8888,
    );
    surface.attach(Some(&new_buffer), 0, 0);
    surface.commit();
}
