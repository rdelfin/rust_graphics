use gl;
use failure;
use nalgebra_glm as glm;

use crate::render_gl::{self, data, buffer};
use crate::resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::f32_f32_f32,
}

pub struct Mesh {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    num_elems: usize,
}

impl Mesh {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        start: glm::Vec3,
        end: glm::Vec3,
        norm: glm::Vec3,
        num_steps: i32
    ) -> Result<Mesh, failure::Error> {
        // Setup shader program
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        // Calculate plane vectors

        let step_full = end - start;
        let step_norm = glm::normalize(&step_full);
        // sqrt(c^2/2) gives length of an isosceles right-angle triangle.
        let step_len = (glm::length(&step_full).powi(2) / 2.0_f32).sqrt() / num_steps as f32;
        let crs = glm::normalize(&norm.cross(&step_norm));
        let base_one = glm::normalize(&(step_norm + crs)) * step_len;
        let base_two = glm::normalize(&(step_norm - crs)) * step_len;

        // Setup vertex buffer object

        let mut vertices: Vec<Vertex> = Vec::new();

        for x in 0..num_steps {
            for y in 0..num_steps {
                let x_f = x as f32;
                let y_f = y as f32;

                vertices.push(Vertex {
                    pos: (start + base_one * x_f + base_two * y_f).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });
                vertices.push(Vertex {
                    pos: (start + base_one * (x_f+1.0_f32) + base_two * y_f).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });
                vertices.push(Vertex {
                    pos: (start + base_one * x_f + base_two * y_f).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });
                vertices.push(Vertex {
                    pos: (start + base_one * x_f + base_two * (y_f+1.0_f32)).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });

                if x == num_steps - 1 {
                    vertices.push(Vertex {
                        pos: (start + base_one * (x_f+1.0_f32) + base_two * y_f).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                    vertices.push(Vertex {
                        pos: (start + base_one * (x_f+1.0_f32) + base_two * (y_f+1.0_f32)).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                }

                if y == num_steps - 1 {
                    vertices.push(Vertex {
                        pos: (start + base_one * x_f + base_two * (y_f+1.0_f32)).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                    vertices.push(Vertex {
                        pos: (start + base_one * (x_f+1.0_f32) + base_two * (y_f+1.0_f32)).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                }
            }
        }

        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        // Setup vertex array buffer
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Mesh{program, _vbo: vbo, vao, num_elems: vertices.len()})


    }

    pub fn get_program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::LINES,  // mode
                0,  // starting index in the enabled arrays
                self.num_elems as gl::types::GLsizei,  // number of indices to be rendered
            );
        }
    }
}


fn print_vec3(name: &str, v: &glm::Vec3) {
    println!("{}: <{}, {}, {}>", name, v.x, v.y, v.z);
}