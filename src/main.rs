#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

extern crate gl;
extern crate sdl2;
extern crate nalgebra;

pub mod render_gl;
pub mod resources;
mod triangle;
mod debug;

use resources::Resources;
use std::path::Path;
use failure::err_msg;
use nalgebra as na;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e))
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets-07"))?;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    let mut viewport = render_gl::Viewport::for_window(800, 600);
    let color_buffer = render_gl::ColorBuffer::from_color(
        na::Vector3::new(0.392156863, 0.584313725, 0.929411765)
    );

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 800, 600)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    let triangle = triangle::Triangle::new(&res, &gl)?;

    viewport.set_used(&gl);
    color_buffer.set_used(&gl);

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                },
                _ => {},
            }
        }

        color_buffer.clear(&gl);
        triangle.render(&gl);

        window.gl_swap_window();
    }

    Ok(())
}
