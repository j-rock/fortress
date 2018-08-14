use render::{
    FragmentShader,
    GeometryShader,
    VertexShader,
    ShaderProgram,
};
use std::{
    collections::HashMap,
    path::PathBuf,
};

pub struct ShaderManager {
    vertex_shaders: HashMap<PathBuf, VertexShader>,
    geometry_shaders: HashMap<PathBuf, GeometryShader>,
    fragment_shaders: HashMap<PathBuf, FragmentShader>,
    short_shader_programs: HashMap<(PathBuf, PathBuf), ShaderProgram>,
    long_shader_programs: HashMap<(PathBuf, PathBuf, PathBuf), ShaderProgram>,
}

pub struct ShaderProgramManager {
}