#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

extern crate gl;
extern crate sdl2;
extern crate nalgebra_glm;

pub mod render_gl;
pub mod resources;
mod triangle;
mod grid;
mod debug;
mod wave_estimator;

use resources::Resources;
use std::f32::consts::PI;
use std::path::Path;
use std::time::SystemTime;
use failure::err_msg;
use nalgebra_glm as glm;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e))
    }
}

fn run() -> Result<(), failure::Error> {
    let initial_screen = (1280, 800);
    let start_time = SystemTime::now();
    let res = Resources::from_relative_exe_path(Path::new("assets-07"))?;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();
    let color_buffer = render_gl::ColorBuffer::from_color(
        glm::Vec3::new(0.0, 0.0, 0.0)
    );

    let mut dragging = false;

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", initial_screen.0, initial_screen.1)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    let mut viewport = render_gl::Viewport::for_window(
        &gl,
        initial_screen.0 as i32,
        initial_screen.1 as i32,
        glm::vec3(0.0, -1.0, 0.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(1.5, 1.5, 1.5),
    );

    // let triangle = triangle::Triangle::new(&res, &gl)?;
    let mut grid = grid::Grid::new(&res, &gl, 1.0, 30)?;
    let mut estimator = wave_estimator::WaveEstimator::new(30, 5.0, |x, y| {
        0.2 * f32::sin(2.0 * std::f32::consts::PI*(x+1.0)) * f32::sin(2.0 * std::f32::consts::PI*(y+1.0))
    });

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
                sdl2::event::Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    dragging = true;
                },
                sdl2::event::Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    dragging = false;
                }
                sdl2::event::Event::MouseButtonDown {
                    mouse_btn,
                    ..
                } => {
                    println!("BUTTON DOWN: {:?}", mouse_btn);
                },
                sdl2::event::Event::MouseButtonUp {
                    mouse_btn,
                    ..
                } => {
                    println!("BUTTON UP: {:?}", mouse_btn);
                },
                sdl2::event::Event::MouseMotion {
                    xrel,
                    yrel,
                    ..
                } => {
                    if dragging {
                        viewport.rotate_by(xrel as f32, yrel as f32, 5.0);
                    }
                },
                sdl2::event::Event::MouseWheel {
                    x,
                    y,
                    ..
                } => {
                    viewport.zoom((y as f32 / 10.0).exp());
                },
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                _ => {},
            }
        }

        estimator.update(0.01);

        let time_in_sec = SystemTime::now()
            .duration_since(start_time)
            .expect("Time went backwards")
            .as_millis() as f32 / 1000.0_f32;
        let t = time_in_sec;

        grid.update_vertices(|x, y| {
            estimator.get_val(x, y)
        });

        color_buffer.clear(&gl);
        viewport.apply_uniforms(grid.get_program_id())?;
        grid.render(&gl);

        window.gl_swap_window();
    }

    Ok(())
}
