extern crate andrew;
extern crate smithay_client_toolkit as sctk;

use std::io::{Read, Seek, SeekFrom, Write};
use std::time::{Duration, Instant};

use sctk::reexports::client::protocol::{wl_seat::WlSeat, wl_shm, wl_surface};
use sctk::shm::{DoubleMemPool, MemPool};
use sctk::window::{ConceptFrame, Event as WEvent};

use andrew::shapes::rectangle;
use andrew::text;
use andrew::text::fontconfig;

sctk::default_environment!(TestExample, desktop);

fn main() {
    let (env, display, mut event_queue) = sctk::new_default_environment!(TestExample, desktop)
        .expect("Unable to connect to a Wayland compositor");

    let _seat = env.manager.instantiate_range::<WlSeat>(1, 6).unwrap();

    let mut dimensions = (600, 400);
    let surface = env.create_surface().detach();
    let mut next_action = None::<WEvent>;

    let mut window = env
        .create_window::<ConceptFrame, _>(surface, None, dimensions, move |evt, mut dispatch_data| {
            let next_actn = dispatch_data.get::<Option<WEvent>>().unwrap();
            // Keep last event in priority order : Close > Configure > Refresh
            let replace = match (&evt, &*next_actn) {
                (_, &None)
                | (_, &Some(WEvent::Refresh))
                | (&WEvent::Configure { .. }, &Some(WEvent::Configure { .. }))
                | (&WEvent::Close, _) => true,
                _ => false,
            };
            if replace {
                *next_actn = Some(evt);
            }
        })
        .expect("Failed to create a window !");

    let mut pools = DoubleMemPool::new(
        env.get_global().expect("Failed to get `WlShm` global."),
        |_| {},
    )
    .expect("Failed to create a memory pool !");

    let mut font_data = Vec::new();
    let font_filename = fontconfig::FontConfig::new()
        .unwrap()
        .get_regular_family_fonts("sans")
        .unwrap()
        .into_iter()
        .filter(|path| {
            use std::ffi::OsStr;
            path.extension()
                .map(|ext| ext == OsStr::new("otf") || ext == OsStr::new("ttf"))
                .unwrap_or(false)
        })
        .next()
        .unwrap();
    ::std::fs::File::open(&font_filename)
        .unwrap()
        .read_to_end(&mut font_data)
        .unwrap();

    if !env
        .get_shell()
        .expect("Expected environment to contain a shell.")
        .needs_configure()
    {
        if let Some(pool) = pools.pool() {
            redraw(pool, window.surface(), dimensions, &font_data);
        }
        window.refresh();
    }

    loop {
        match next_action.take() {
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
                    redraw(pool, window.surface(), dimensions, &font_data);
                }
            }
            None => {}
        }

        display.flush().unwrap();
        event_queue
            .dispatch(&mut next_action, |_, _, _| {})
            .unwrap();
    }
}

fn redraw(
    pool: &mut MemPool,
    surface: &wl_surface::WlSurface,
    dimensions: (u32, u32),
    font_data: &[u8],
) {
    let (buf_x, buf_y) = (dimensions.0 as usize, dimensions.1 as usize);

    pool.resize(4 * buf_x * buf_y)
        .expect("Failed to resize the memory pool.");

    let mut buf: Vec<u8> = vec![255; 4 * buf_x * buf_y];
    let mut canvas =
        andrew::Canvas::new(&mut buf, buf_x, buf_y, 4 * buf_x, andrew::Endian::native());

    println!("______________");
    let mut total_dur = Duration::new(0, 0);

    // Draw background
    let (block_w, block_h) = (buf_x / 20, buf_y / 20);
    for block_y in 0..21 {
        for block_x in 0..21 {
            let color = if (block_x + (block_y % 2)) % 2 == 0 {
                [255, 0, 0, 0]
            } else {
                [255, 255, 255, 255]
            };

            let block = rectangle::Rectangle::new(
                (block_w * block_x, block_h * block_y),
                (block_w, block_h),
                None,
                Some(color),
            );
            let timer = Instant::now();
            canvas.draw(&block);
            total_dur += timer.elapsed()
        }
    }
    println!("Background draw time: {:?}", total_dur);

    let rectangle = rectangle::Rectangle::new(
        (buf_x / 30, buf_y / 4),
        (buf_x - (buf_x / 30) * 2, buf_y - buf_y / 2),
        Some((
            15,
            [255, 170, 20, 45],
            rectangle::Sides::TOP ^ rectangle::Sides::BOTTOM,
            Some(10),
        )),
        Some([255, 170, 20, 45]),
    );
    let mut timer = Instant::now();
    canvas.draw(&rectangle);
    println!("Rectangle draw time: {:?}", timer.elapsed());
    total_dur += timer.elapsed();

    let text_h = buf_x as f32 / 80.;
    let text_hh = text_h / 2.;
    let mut text = text::Text::new(
        (63, 69),
        [255, 255, 255, 255],
        font_data,
        text_h,
        2.0,
        "“Life is the art of drawing without an eraser.” - John W. Gardner",
    );
    text.pos = (
        buf_x / 2 - text.get_width() / 2,
        buf_y / 2 - text_hh as usize,
    );

    let text_box = rectangle::Rectangle::new(
        (
            buf_x / 2 - text.get_width() / 2 - 10,
            buf_y / 2 - text_hh as usize - 10,
        ),
        (text.get_width() + 20, text_h as usize + 20),
        Some((3, [255, 255, 255, 255], rectangle::Sides::ALL, Some(5))),
        None,
    );

    timer = Instant::now();
    canvas.draw(&text_box);
    println!("Text box draw time: {:?}", timer.elapsed());
    total_dur += timer.elapsed();

    timer = Instant::now();
    canvas.draw(&text);
    println!("Text draw time: {:?}", timer.elapsed());
    total_dur += timer.elapsed();

    println!("Total draw time: {:?}", total_dur);

    pool.seek(SeekFrom::Start(0)).unwrap();
    pool.write_all(canvas.buffer).unwrap();
    pool.flush().unwrap();

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
