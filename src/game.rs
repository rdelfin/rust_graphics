use crate::resources::Resources;
use crate::{render_gl, debug};
use failure::err_msg;
use sdl2;
use std::path::Path;
use std::time::SystemTime;

use nalgebra_glm as glm;
use crate::render_gl::Viewport;

pub trait Game {
    fn load(&mut self, gl: &gl::Gl, resources: &Resources) -> Result<(), failure::Error>;
    fn update(&mut self) -> Result<(), failure::Error>;
    fn render(&mut self, gl: &gl::Gl, viewport: &mut Viewport) -> Result<(), failure::Error>;
}

pub struct GameExecutor<G> where G: Game {
    game: G,
    screen: (u32, u32),
}

impl<G> GameExecutor<G> where G: Game {
    pub fn new(game: G, screen: (u32, u32)) -> GameExecutor<G> {
        GameExecutor { game, screen }
    }

    pub fn run(&mut self) {
        if let Err(e) = self.run_result() {
            println!("{}", debug::failure_to_string(e))
        }
    }

    fn run_result(&mut self) -> Result<(), failure::Error> {
        let start_time = SystemTime::now();
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
            .window("Game", self.screen.0, self.screen.1)
            .opengl()
            .resizable()
            .build()?;

        let _gl_context = window.gl_create_context().map_err(err_msg)?;
        let gl = gl::Gl::load_with(
            |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        );

        let mut viewport = render_gl::Viewport::for_window(
            &gl,
            self.screen.0 as i32,
            self.screen.1 as i32,
            glm::vec3(0.0, -1.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(1.5, 1.5, 1.5),
        );

        self.game.load(&gl, &res);

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

            self.game.update();

            color_buffer.clear(&gl);
            self.game.render(&gl, &mut viewport);
            window.gl_swap_window();
        }

        Ok(())
    }
}
