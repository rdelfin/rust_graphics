use gl;
use nalgebra_glm as glm;
use super::uniform::UniformFMat4;
use crate::render_gl::uniform::{Uniform, Error as UniformError};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to apply uniforms")]
    ResourceLoad {
        #[cause] inner: UniformError,
    },
}

pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub up: glm::Vec3,
    pub left: glm::Vec3,
    pub forwards: glm::Vec3,
    pub position: glm::Vec3,
    pub center: glm::Vec3,

    pub proj_uniform: UniformFMat4,
    pub view_uniform: UniformFMat4,
}

impl Viewport {
    pub fn update_size(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
        self.update_placement(self.up, self.center, self.position);
    }

    pub fn rotate_by(&mut self, x: f32, y: f32, speed: f32) {
        let temp_pos = glm::rotate_vec3(
            &self.position, x / self.w as f32 * speed, &self.up
        );
        self.position = glm::rotate_vec3(
            &temp_pos, -y / self.h as f32 * speed, &self.left
        );
        self.update_placement(self.up, self.center, self.position);
    }

    pub fn update_placement(&mut self, up: glm::Vec3, center: glm::Vec3, position: glm::Vec3) {
        self.position = position;
        self.center = center;
        self.forwards = glm::normalize(&(center - position));
        self.left = up.cross(&self.forwards);
        self.left = glm::normalize(&self.left);
        self.up = self.forwards.cross(&self.left);
        self.up = glm::normalize(&self.up);

        self.update_proj_mat();
        self.update_view_mat();
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe { gl.Viewport(self.x, self.y, self.w, self.h) };
    }

    pub fn clean(&self, gl: &gl::Gl) {
        unsafe { gl.Clear(gl::COLOR_BUFFER_BIT) };
    }

    pub fn update_proj_mat(&mut self) {
        self.proj_uniform.update(
                &glm::perspective_fov(
                (3.0 * std::f32::consts::PI / 2.0) as f32,
                self.w as f32,
                self.h as f32,
                0.5_f32,
                1000.0_f32,
            )
        );
    }

    pub fn update_view_mat(&mut self) {
        self.view_uniform.update(&glm::look_at(&self.position, &self.center, &self.up));
    }

    pub fn apply_uniforms(&mut self, program_id: gl::types::GLuint) -> Result<(), Error> {
        self.view_uniform.apply_uniform(program_id).map_err(
            |e| Error::ResourceLoad { inner: e }
        )?;
        self.proj_uniform.apply_uniform(program_id).map_err(
            |e| Error::ResourceLoad { inner: e }
        )?;
        Ok(())
    }

    pub fn for_window(
        gl: &gl::Gl, w: i32, h: i32, up: glm::Vec3, center: glm::Vec3, position: glm::Vec3
    ) -> Viewport {
        let mut viewport = Viewport {
            x: 0,
            y: 0,
            w,
            h,
            up: glm::vec3(0.0, 0.0, 0.0),
            left: glm::vec3(0.0, 0.0, 0.0),
            center: glm::vec3(0.0, 0.0, 0.0),
            position: glm::vec3(0.0, 0.0, 0.0),
            forwards: glm::vec3(0.0, 0.0, 0.0),
            proj_uniform: UniformFMat4::new_with_loc(gl, "projection", glm::mat4(
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ), 2),
            view_uniform: UniformFMat4::new_with_loc(gl, "view", glm::mat4(
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ), 1),
        };

        viewport.update_placement(up, center, position);
        viewport
    }
}