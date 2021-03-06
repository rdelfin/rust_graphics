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
    #[location = 2]
    offset: data::one_f32,
}

pub struct Grid {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    vertices: Vec<Vertex>,
    _scale: f32,
    num_steps: i32,
}

impl Grid {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        scale: f32,
        num_steps: i32
    ) -> Result<Grid, failure::Error> {
        // Setup shader program
        let program = render_gl::Program::from_res(gl, res, "shaders/grid")?;

        let vertices = Grid::generate_vertices(
            scale, num_steps, |_x, _y| { 0.0_f32 },
        );

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

        Ok(Grid{program, vbo, vao, vertices, _scale: scale, num_steps})
    }

    pub fn get_program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

    pub fn update_vertices(&mut self, f: impl Fn(f32, f32) -> f32) {
        let mut idx = 0;
        for x in -self.num_steps..(self.num_steps+1) {
            for y in -self.num_steps..(self.num_steps+1) {
                let x_f = x as f32 / (self.num_steps as f32);
                let y_f = y as f32 / (self.num_steps as f32);

                self.vertices[idx].offset = f(x_f, y_f).into();
                idx += 1;
            }
        }

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&self.vertices);
        self.vbo.unbind();
    }

    pub fn render(&mut self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::POINTS,  // mode
                0,  // starting index in the enabled arrays
                self.vertices.len() as gl::types::GLsizei,  // number of indices to be rendered
            );
        }
    }

    fn generate_vertices(
        scale: f32,
        num_steps: i32,
        f: impl Fn(f32, f32) -> f32
    ) -> Vec<Vertex> {
        // Setup vertices

        let mut vertices: Vec<Vertex> = Vec::new();

        for x in -num_steps..(num_steps+1) {
            for y in -num_steps..(num_steps+1) {
                let x_f = x as f32 / (num_steps as f32);
                let y_f = y as f32 / (num_steps as f32);

                let scaled_x = x_f * scale as f32;
                let scaled_y = y_f * scale as f32;

                vertices.push(Vertex {
                    pos: (scaled_x, 0.0, scaled_y).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                    offset: f(x_f, y_f).into(),
                });
            }
        }

        vertices
    }
}
