mod apps;
mod app;
mod gfx;
mod types;
mod util;

extern crate glutin;
extern crate gleam;
extern crate time;
extern crate inotify;

use app::{App, AppKind};
use gfx::GfxContext;
use gleam::gl;
use std::env;
use time::precise_time_ns;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app_name = &args[1];
    let app_window = glutin::WindowBuilder::new().with_dimensions(1920, 1080)
                                                 .build()
                                                 .unwrap();
    app_window.set_title(app_name);
    let _ = unsafe { app_window.make_current() };
    gl::load_with(|s| app_window.get_proc_address(s) as *const _);

    let mut gfx = GfxContext::new(app_window.create_window_proxy());

    let mut app = match app_name.as_str() {
        "test" => Box::new(apps::test::Test::new(&mut gfx)) as Box<App>,
        "null" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 0, 0)) as Box<App>,
        "clear1" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 0)) as Box<App>,
        "clear2" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 2, 0)) as Box<App>,
        "clear4" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 4, 0)) as Box<App>,
        "clear8" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 8, 0)) as Box<App>,
        "quad1" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 0, 1)) as Box<App>,
        "quad2" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 0, 2)) as Box<App>,
        "quad4" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 0, 4)) as Box<App>,
        "quad8" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 0, 8)) as Box<App>,
        "clear1_quad1" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 1)) as Box<App>,
        "clear1_quad2" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 2)) as Box<App>,
        "clear1_quad3" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 3)) as Box<App>,
        "clear1_quad4" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 4)) as Box<App>,
        "clear1_quad5" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 5)) as Box<App>,
        "clear1_quad6" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 6)) as Box<App>,
        "clear1_quad7" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 7)) as Box<App>,
        "clear1_quad8" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 8)) as Box<App>,
        "clear1_quad9" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 9)) as Box<App>,
        "clear1_quad10" => Box::new(apps::quad_bench::QuadBench::new(&mut gfx, 1, 10)) as Box<App>,
        _ => panic!("unknown app name"),
    };

    let start_time = precise_time_ns();
    let mut frame_count = 0;

    loop {
        match app.kind() {
            AppKind::Test => {
                let event = app_window.wait_events().next().unwrap();
                let event = match event {
                    glutin::Event::Closed => break,
                    glutin::Event::KeyboardInput(state, scan_code, _) => {
                        match state {
                            glutin::ElementState::Pressed => {
                                match scan_code {
                                    9 => break,
                                    _ => None
                                }
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                };
                if let Some(event) = event {
                    app.on_event(event);
                }
            }
            AppKind::Benchmark => {
                if frame_count == 500 {
                    break;
                }
            }
        }

        let (width, height) = app_window.get_inner_size().unwrap();
        gfx.begin_frame(width, height);
        app.draw(&mut gfx, width, height);
        gfx.end_frame();

        frame_count += 1;

        let _ = app_window.swap_buffers();
    }

    let end_time = precise_time_ns();
    let ms = (end_time - start_time) as f64 / 1000000.0;
    println!("{} frames={} total={}ms avg={}ms", app_name, frame_count, ms, ms / frame_count as f64);

    app.deinit(&mut gfx);
}
