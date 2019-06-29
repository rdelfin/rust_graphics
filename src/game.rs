use gl;
use failure::err_msg;
use std::path::Path;
use crate::{debug, render_gl};
use crate::render_gl::resources::Resources;
use nalgebra_glm as glm;
use crate::render_gl::Viewport;

pub trait BaseGame {
    fn load(&mut self, res: &Resources, gl: &gl::Gl) -> Result<(), failure::Error>;
    fn update(&mut self, viewport: &mut Viewport) -> Result<(), failure::Error>;
    fn render(&mut self, gl: &gl::Gl, viewport: &mut Viewport) -> Result<(), failure::Error>;
}


pub struct GameExecutor<G: BaseGame> {
    game_impl: G,
    screen_dims: (u32, u32),
}


impl<G: BaseGame> GameExecutor<G> {
    pub fn new(game_impl: G, screen_dims: (u32, u32)) -> GameExecutor<G> {
        return GameExecutor{ game_impl, screen_dims }
    }

    pub fn run(&mut self) {
        if let Err(e) = self.execute() {
            println!("{}", debug::failure_to_string(e))
        }
    }

    fn execute(&mut self) -> Result<(), failure::Error> {
        let res = Resources::from_relative_exe_path(Path::new("assets-07"))?;

        let sdl = sdl2::init().map_err(err_msg)?;
        let video_subsystem = sdl.video().map_err(err_msg)?;

        let gl_attr = video_subsystem.gl_attr();
        let color_buffer = render_gl::ColorBuffer::from_color(
            glm::Vec3::new(0.0, 0.0, 0.0)
        );

        let mut dragging = false;

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window = video_subsystem
            .window("Game", self.screen_dims.0, self.screen_dims.1)
            .opengl()
            .resizable()
            .build()?;

        let _gl_context = window.gl_create_context().map_err(err_msg)?;
        let gl = gl::Gl::load_with(
            |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        );

        let mut viewport = render_gl::Viewport::for_window(
            &gl,
            self.screen_dims.0 as i32,
            self.screen_dims.1 as i32,
            glm::vec3(0.0, -1.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(1.5, 1.5, 1.5),
        );

        self.game_impl.load(&res, &gl);

        viewport.set_used(&gl);
        color_buffer.set_used(&gl);

        let mut event_pump = sdl.event_pump().map_err(err_msg)?;
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::Window {
                        win_event: sdl2::event::WindowEvent::Resized(w, h),
                        ..
                    } => {
                        self.screen_dims = (w as u32, h as u32);
                        viewport.update_size(w, h);
                        viewport.set_used(&gl);
                    },
                    sdl2::event::Event::MouseButtonDown {
                        mouse_btn: sdl2::mouse::MouseButton::Left,
                        ..
                    } => {
                        dragging = true;
                    },
                    sdl2::event::Event::MouseButtonUp {
                        mouse_btn: sdl2::mouse::MouseButton::Left,
                        ..
                    } => {
                        dragging = false;
                    }
                    sdl2::event::Event::MouseButtonDown {
                        mouse_btn,
                        ..
                    } => {
                        println!("BUTTON DOWN: {:?}", mouse_btn);
                    },
                    sdl2::event::Event::MouseButtonUp {
                        mouse_btn,
                        ..
                    } => {
                        println!("BUTTON UP: {:?}", mouse_btn);
                    },
                    sdl2::event::Event::MouseMotion {
                        xrel,
                        yrel,
                        ..
                    } => {
                        if dragging {
                            viewport.rotate_by(xrel as f32, yrel as f32, 5.0);
                        }
                    },
                    sdl2::event::Event::MouseWheel {
                        x,
                        y,
                        ..
                    } => {
                        viewport.zoom((y as f32 / 10.0).exp());
                    },
                    sdl2::event::Event::KeyUp {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'main,
                    _ => {},
                }
            }

            self.game_impl.update(&mut viewport);

            color_buffer.clear(&gl);

            self.game_impl.render(&gl, &mut viewport);

            window.gl_swap_window();
        }

        Ok(())
    }
}
