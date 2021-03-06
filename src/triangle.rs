use gl;
use failure;
use crate::render_gl::{self, data, buffer};
use crate::render_gl::resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::f32_f32_f32,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {
        // Setup shader program
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        // Setup vertex buffer object

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0).into(),
            },  // bottom left
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0).into(),
            },  // bottom right
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0).into(),
            },  // top right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0).into(),
            },  // bottom left
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0).into(),
            },  // top right
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                clr: (1.0, 1.0, 0.0).into(),
            }
        ];

        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.dynamic_draw_data(&vertices);
        vbo.unbind();

        // Setup vertex array buffer
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle{program, _vbo: vbo, vao})


    }

    pub fn get_program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,  // mode
                0,  // starting index in the enabled arrays
                6,  // number of indices to be rendered
            );
        }
    }
}