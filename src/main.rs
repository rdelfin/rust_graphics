#[macro_use] extern crate failure;

extern crate gl;
extern crate sdl2;

pub mod render_gl;
pub mod resources;

use render_gl::data;
use resources::Resources;
use std::path::Path;
use failure::err_msg;

struct Vertex {
    #[allow(dead_code)]
    pos: data::f32_f32_f32,
    #[allow(dead_code)]
    clr: data::f32_f32_f32,
}

impl Vertex {
    fn vertex_attrib_pointers(gl: &gl::Gl) {
        let stride = std::mem::size_of::<Self>();  // byte offset between consecutive attributes

        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, 0, 0);
            data::f32_f32_f32::vertex_attrib_pointer(
                gl, stride, 1, std::mem::size_of::<data::f32_f32_f32>()
            );
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e))
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets-07"))?;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

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

    unsafe {
        gl.Viewport(0, 0, 800, 600);
        gl.ClearColor(0.392156863, 0.584313725, 0.929411765, 1.0);
    }

    let shader_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle"
    )?;

    shader_program.set_used();

    let vertices: Vec<Vertex> = vec![
        Vertex{ pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex{ pos: (0.5,  -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },
        Vertex{ pos: (0.0,   0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe { gl.GenBuffers(1, &mut vbo) };

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,  // Target
            // Size of data in bytes
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,  // Pointer to data
            gl::STATIC_DRAW,
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);  // Unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe { gl.GenVertexArrays(1, &mut vao) };

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        Vertex::vertex_attrib_pointers(&gl);
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }


    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        shader_program.set_used();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES,  // mode
                0,  // starting index in the enabled arrays
                3,  // number of indexes to be rendered
            );
        }

        window.gl_swap_window();
    }

    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
        {
            if i > 0 {
                let _ = writeln!(&mut result, "   Which caused the following issue:");
            }
            let _ = write!(&mut result, "{}", cause);
            if let Some(backtrace) = cause.backtrace() {
                let backtrace_str = format!("{}", backtrace);
                if backtrace_str.len() > 0 {
                    let _ = writeln!(&mut result, " This happened at {}", backtrace);
                } else {
                    let _ = writeln!(&mut result);
                }
            } else {
                let _ = writeln!(&mut result);
            }
        }

    result
}
