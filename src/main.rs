#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

extern crate gl;
extern crate sdl2;
extern crate nalgebra_glm;

use std::time::SystemTime;

use crate::resources::Resources;
use crate::render_gl::Viewport;
use crate::wave_estimator::WaveEstimator;
use crate::grid::Grid;

pub mod render_gl;
pub mod resources;
mod triangle;
mod game;
mod grid;
mod debug;
mod wave_estimator;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Object expected to be 'Some' is 'None'")]
    NoneObject(String),
}

struct Game {
    grid: Option<Grid>,
    estimator: Option<WaveEstimator>,
    start_time: SystemTime,
}

impl Game {
    fn new() -> Game {
        Game{ grid: None, estimator: None, start_time: SystemTime::now() }
    }
}

impl game::BaseGame for Game {
    fn load(&mut self, res: &Resources, gl: &gl::Gl) -> Result<(), failure::Error> {
        // let triangle = triangle::Triangle::new(&res, &gl)?;
        self.grid = Some(grid::Grid::new(&res, &gl, 1.0, 30)?);
        self.estimator = Some(wave_estimator::WaveEstimator::new(30, 5.0, |x, y| {
            0.2 * f32::sin(2.0 * std::f32::consts::PI*(x+1.0)) * f32::sin(2.0 * std::f32::consts::PI*(y+1.0))
        }));

        Ok(())
    }

    fn update(&mut self, viewport: &mut Viewport) -> Result<(), failure::Error> {
        let mut estimator = self.estimator.as_mut().ok_or(Error::NoneObject("estimator".to_string()))?;
        let mut grid = self.grid.as_mut().ok_or(Error::NoneObject("grid".to_string()))?;

        estimator.update(0.01);

        let time_in_sec = SystemTime::now()
            .duration_since(self.start_time)
            .expect("Time went backwards")
            .as_millis() as f32 / 1000.0_f32;
        let t = time_in_sec;

        grid.update_vertices(|x, y| {
            estimator.get_val(x, y)
        });

        Ok(())
    }

    fn render(&mut self, gl: &gl::Gl, viewport: &mut Viewport) -> Result<(), failure::Error> {
        let mut grid = self.grid.as_mut().ok_or(Error::NoneObject("grid".to_string()))?;

        viewport.apply_uniforms(grid.get_program_id())?;
        grid.render(&gl);

        Ok(())
    }
}

fn main() {
    let mut game = game::GameExecutor::new(Game::new(), (1280, 800));
    game.run();
}
