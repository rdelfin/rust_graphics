use nalgebra_glm as glm;
use gl;

pub struct ColorBuffer {
    pub color: glm::Vec4,
}

impl ColorBuffer {
    pub fn from_color(color: glm::Vec3) -> ColorBuffer {
        ColorBuffer {
            color: glm::Vec4::new(color.x, color.y, color.z, 1.0),
        }
    }

    pub fn update_color(&mut self, color: glm::Vec3) {
        self.color = glm::Vec4::new(color.x, color.y, color.z, 1.0);
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe { gl.ClearColor(self.color.x, self.color.y, self.color.z, 1.0) };
    }

    pub fn clear(&self, gl: &gl::Gl) {
        unsafe { gl.Clear(gl::COLOR_BUFFER_BIT) };
    }
}