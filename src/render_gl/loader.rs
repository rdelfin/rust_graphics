use std::collections::HashMap;
use std::fs;
use std::io;

use crate::render_gl::resources::{self, Resources};
use crate::render_gl::model_parsers::interface::ModelData;

use nalgebra_glm as glm;

#[derive(Debug, Fail)] // derive Fail, in addition to Debug
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad {
        name: String,
        #[cause]
        inner: resources::Error,
    },
    #[fail(display = "Could not interpret line: \"{}\"", line)]
    InvalidObjLine { line: String },
}


pub trait Loader {
    fn load_file(
        &mut self,
        file_path: &str,
        res: &Resources,
    ) -> Result<HashMap<String, ModelData>, Error>;
}

pub struct ObjLoader {}

impl Loader for ObjLoader {
    fn load_file(
        &mut self,
        _resource_name: &str,
        _res: &Resources,
    ) -> Result<HashMap<String, ModelData>, Error> {
        let map_result: HashMap<String, ModelData> = HashMap::new();
        Ok(map_result)
    }
}
