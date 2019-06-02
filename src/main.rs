#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

extern crate gl;
extern crate sdl2;
extern crate nalgebra_glm;

pub mod render_gl;
pub mod resources;
mod triangle;
mod game;
mod grid;
mod debug;
mod wave_estimator;

use failure::err_msg;
use resources::Resources;
use crate::wave_estimator::WaveEstimator;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Expected Optional to be Some, is None")]
    NoneValue(String),
}

struct MainGame {
    grid: Option<grid::Grid>,
    estimator: Option<WaveEstimator>,
}

impl MainGame {
    fn new() -> MainGame {
        MainGame { grid: None, estimator: None }
    }
}

impl game::Game for MainGame {
    fn load(&mut self, gl: &gl::Gl, res: &Resources) -> Result<(), failure::Error> {
        // let triangle = triangle::Triangle::new(&res, &gl)?;
        let grid = grid::Grid::new(res, gl, 1.0, 30)?;
        self.grid = Some(grid);
        self.estimator = Some(wave_estimator::WaveEstimator::new(30, 5.0, |x, y| {
            0.2 * f32::sin(2.0 * std::f32::consts::PI*(x+1.0)) * f32::sin(2.0 * std::f32::consts::PI*(y+1.0))
        }));

        Ok(())
    }

    fn update(&mut self) -> Result<(), failure::Error> {
        match &mut self.estimator {
            Some(estimator) => {
                estimator.update(0.01);
            },
            None => {
                return Err(Error::NoneValue("estimator".to_string()))?;
            },
        }

        Ok(())
    }

    fn render(&mut self, gl: &gl::Gl, viewport: &mut render_gl::Viewport) -> Result<(), failure::Error> {
        match &mut self.grid {
            Some(grid) => {
                viewport.apply_uniforms(grid.get_program_id())?;
                grid.render(&gl);
            },
            None => {
                return Err(Error::NoneValue("grid".to_string()))?;
            },
        }

        Ok(())
    }
}

fn main() {
    let mut game = game::GameExecutor::new(
        MainGame::new(), (1280, 800)
    );
    game.run();
}
