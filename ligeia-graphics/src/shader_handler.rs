use gl;
use gl::types::*;

use std::collections::HashMap;
use std::ops::Drop;

use Shader;


//TODO: refactor to not require SFML!
pub type ShaderHandle = u32;



pub struct ShaderHandler {
    _handle_gen: ShaderHandle,
    _shaders: HashMap<ShaderHandle, Shader>
}

impl ShaderHandler {
    pub fn new() -> Self {
        Self {
            _handle_gen: 0,
            _shaders: HashMap::new()
        }
    }

    pub fn load_shader(
        &mut self,
        vert_source: &str,
        frag_source: &str,
        frag_data_location: &str,
        frag_texture_uniform: &str,
        vert_pos_name: &str,
        vert_color_name: &str,
        vert_tex_coords_name: &str
    ) -> ShaderHandle {
        let handle = self._handle_gen;
        self._shaders.insert(handle, Shader::new(
            vert_source,
            frag_source,
            frag_data_location,
            frag_texture_uniform,
            vert_pos_name,
            vert_color_name,
            vert_tex_coords_name
        ));
        self._handle_gen += 1;

        handle
    }

    pub fn load_shader_from_files(
        &mut self,
        vert_file: Option<&str>,
        geom_file: Option<&str>,
        frag_file: Option<&str>
    ) -> Option<ShaderHandle> {
        //TODO: implement
        None
    }

    pub fn unload_shader(&mut self, handle: ShaderHandle) -> bool {
        match self._shaders.remove(&handle) {
            Some(_) => true,
            None    => false
        }
    }

    pub fn get_shader(&self, handle: ShaderHandle) -> Option<&Shader> {
        self._shaders.get(&handle)
    }

    pub fn get_default(&self) -> Option<&Shader> {
        self._shaders.get(&0)
    }


}

