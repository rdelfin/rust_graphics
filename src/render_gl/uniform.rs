use gl;
use nalgebra_glm as glm;

pub trait Uniform {
    fn apply_uniform(&self, program_id: gl::types::GLuint);
}


pub struct UniformFMat4 {
    name: String,
    gl: gl::Gl,
    data: glm::Mat4,
}

impl UniformFMat4 {
    pub fn new(gl: &gl::Gl, name: &str, data: glm::Mat4) -> UniformFMat4 {
        return UniformFMat4 { gl: gl.clone(), name: name.to_string(), data }
    }

    pub fn update(&mut self, data: &glm::Mat4) {
        self.data = data.clone();
    }
}

impl Uniform for UniformFMat4 {
    fn apply_uniform(&self, program_id: gl::types::GLuint) {
        unsafe {
            let loc = self.gl.GetUniformLocation(
                program_id, self.name.as_ptr() as *const gl::types::GLchar
            );
            self.gl.UniformMatrix4fv(loc, 1, gl::FALSE, self.data.as_ptr());
        }
    }

}
