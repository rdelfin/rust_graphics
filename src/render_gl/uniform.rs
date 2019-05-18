use gl;
use nalgebra_glm as glm;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Unable to find uniform location: {}", name)]
    UniformNotFound { name: String },
}

pub trait Uniform {
    fn apply_uniform(&mut self, program_id: gl::types::GLuint) -> Result<(), Error>;
}


pub struct UniformFMat4 {
    name: String,
    gl: gl::Gl,
    data: glm::Mat4,
    location: i32,
}

impl UniformFMat4 {
    pub fn new(gl: &gl::Gl, name: &str, data: glm::Mat4) -> UniformFMat4 {
        return UniformFMat4 { gl: gl.clone(), name: name.to_string(), data, location: -1 }
    }

    pub fn update(&mut self, data: &glm::Mat4) {
        self.data = data.clone();
    }
}

impl Uniform for UniformFMat4 {
    fn apply_uniform(&mut self, program_id: gl::types::GLuint) -> Result<(), Error> {
        if self.location < 0 {
            unsafe {
                self.location = self.gl.GetUniformLocation(
                    program_id, self.name.as_ptr() as *const gl::types::GLchar
                );
            }
        }

        // If value is still bellow 0, GeUniformLocation returned an error
        if self.location < 0 {
            return Err(Error::UniformNotFound { name: self.name.clone() });
        }

        unsafe {
            self.gl.UniformMatrix4fv(self.location, 1, gl::FALSE, self.data.as_ptr());
        }

        Ok(())
    }

}
