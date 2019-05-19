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
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    vertices: Vec<Vertex>,
    norm: glm::Vec3,
    start: glm::Vec3,
    end: glm::Vec3,
    num_steps: i32,
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

        let vertices = Mesh::generate_vertices(
            start, end, norm, num_steps, |_x, _y| { 0.0_f32 },
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

        Ok(Mesh{program, vbo, vao, vertices, norm: glm::normalize(&norm), start, end, num_steps})
    }

    pub fn get_program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

    pub fn update_vertices(&mut self, f: impl Fn(f32, f32) -> f32) {
        let vertices = Mesh::generate_vertices(
            self.start, self.end, self.norm, self.num_steps, f
        );

        self.vertices = vertices;

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&self.vertices);
        self.vbo.unbind();
    }

    pub fn render(&mut self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::LINES,  // mode
                0,  // starting index in the enabled arrays
                self.vertices.len() as gl::types::GLsizei,  // number of indices to be rendered
            );
        }
    }

    fn generate_vertices(
        start: glm::Vec3,
        end: glm::Vec3,
        norm: glm::Vec3,
        num_steps: i32,
        f: impl Fn(f32, f32) -> f32
    ) -> Vec<Vertex> {
        // Calculate plane vectors
        let normalized_norm = glm::normalize(&norm);
        let step_full = end - start;
        let step_norm = glm::normalize(&step_full);
        // sqrt(c^2/2) gives length of an isosceles right-angle triangle.
        let step_len = (glm::length(&step_full).powi(2) / 2.0_f32).sqrt() / num_steps as f32;
        let crs = glm::normalize(&norm.cross(&step_norm));
        let base_one = glm::normalize(&(step_norm + crs)) * step_len;
        let base_two = glm::normalize(&(step_norm - crs)) * step_len;

        // Setup vertices

        let mut vertices: Vec<Vertex> = Vec::new();

        for x in 0..num_steps {
            for y in 0..num_steps {
                let x_f = x as f32;
                let y_f = y as f32;

                let norm_x = x_f / num_steps as f32;
                let norm_y = y_f / num_steps as f32;
                let norm_num_steps = 1.0_f32 / num_steps as f32;

                let offset = normalized_norm * f(norm_x, norm_y);
                let offset_x1 = normalized_norm * f(norm_x + norm_num_steps, norm_y);
                let offset_y1 = normalized_norm * f(norm_x, norm_y + norm_num_steps);
                let offset_x1_y1 = normalized_norm * f(norm_x + norm_num_steps, norm_y + norm_num_steps);


                vertices.push(Vertex {
                    pos: (start + base_one * x_f + base_two * y_f + offset).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });
                vertices.push(Vertex {
                    pos: (start + base_one * (x_f+1.0_f32) + base_two * y_f + offset_x1).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });
                vertices.push(Vertex {
                    pos: (start + base_one * x_f + base_two * y_f + offset).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });
                vertices.push(Vertex {
                    pos: (start + base_one * x_f + base_two * (y_f+1.0_f32) + offset_y1).into(),
                    clr: (1.0, 0.0, 0.0).into(),
                });

                if x == num_steps - 1 {
                    vertices.push(Vertex {
                        pos: (start + base_one * (x_f+1.0_f32) + base_two * y_f + offset_x1).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                    vertices.push(Vertex {
                        pos: (
                            start + base_one * (x_f+1.0_f32) + base_two * (y_f+1.0_f32) + offset_x1_y1
                        ).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                }

                if y == num_steps - 1 {
                    vertices.push(Vertex {
                        pos: (start + base_one * x_f + base_two * (y_f+1.0_f32) + offset_y1).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                    vertices.push(Vertex {
                        pos: (
                            start + base_one * (x_f+1.0_f32) + base_two * (y_f+1.0_f32) + offset_x1_y1
                        ).into(),
                        clr: (1.0, 0.0, 0.0).into(),
                    });
                }
            }
        }

        vertices
    }
}
