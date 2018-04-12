use sfml::graphics::Shader;
use sfml::system::SfBox;
use std::collections::HashMap;
use std::ops::Deref;

pub type ShaderHandle = u32;

pub struct ShaderHandler<'a> {
    _handle_gen: ShaderHandle,
    _shaders: HashMap<ShaderHandle, Shader<'a>>
}

impl<'a> ShaderHandler<'a> {
    pub fn new() -> Self {
        let mut shader_handler = Self {
            _handle_gen: 0,
            _shaders: HashMap::new()
        };

        shader_handler.load_shader(
            Some("assets/shaders/default.vert"),
            None,
            Some("assets/shaders/default.frag")
        );

        shader_handler
    }

    pub fn load_shader(
        &mut self,
        vert_file: Option<&str>,
        geom_file: Option<&str>,
        frag_file: Option<&str>
    ) -> Option<ShaderHandle> {
        match Shader::from_file(vert_file, geom_file, frag_file) {
            Some(shader) => {
                self._shaders.insert(self._handle_gen, shader);
                self._handle_gen += 1;
                Some(self._handle_gen - 1)
            },
            None        => None
        }
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
}