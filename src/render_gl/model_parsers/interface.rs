use std::collections::HashMap;

use nalgebra_glm as glm;

#[derive(Clone)]
pub struct ModelData {
    pub points: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub name: String,
}

impl ModelData {
    pub fn new_empty() -> Self {
        return ModelData {
            points: vec![],
            normals: vec![],
            name: "".to_string(),
        };
    }

    pub fn new(points: Vec<glm::Vec3>, normals: Vec<glm::Vec3>, name: String) -> Self {
        return ModelData {
            points,
            normals,
            name,
        };
    }
}

pub trait FormatInterpreter<Token> {
    fn lex(&self, data: &str) -> Vec<Token>;
    fn parse(&self, tokens: Vec<Token>) -> HashMap<String, ModelData>;
}
