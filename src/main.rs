extern crate gl;
extern crate sdl2;

pub mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.392156863, 0.584313725, 0.929411765, 1.0);
    }

    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        // positions      // colors
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,  0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0,
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,  // Target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // Size of
            // data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid,  // Pointer to data
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);  // Unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);  // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,  // index of the generic vertex attribute ("layout (location = 0)")
            3,  // the number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float) conversion
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,   // stride (byte offset between
            // consecutive attributes)
            std::ptr::null(),  // offset of the first component
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,  // index of the generic vertex attribute ("layout (location = 1")
            3,  // the number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float) conversion
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,   // stride (byte offset between
            // consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,  // offset of the first
            // component

        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }


    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        shader_program.set_used();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,  // mode
                0,  // starting index in the enabled arrays
                3,  // number of indexes to be rendered
            );
        }

        window.gl_swap_window();
    }
}