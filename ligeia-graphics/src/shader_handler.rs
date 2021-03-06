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
        let vs_src: &str = "
            #version 330
            in vec2 position;
            in vec4 color;
            in vec2 tex_coords;

            uniform mat3 projection;

            out vec4 v_color;
            out vec2 v_tex_coords;

            void main() {
                vec3 intermediate_pos = projection * vec3(position, 1.0);
                gl_Position = vec4(intermediate_pos.xy, 0.0, 1.0);
                v_color = color;
                v_tex_coords = tex_coords;
            }";
        let fs_src: &str = "
            #version 330

            in vec4 v_color;
            in vec2 v_tex_coords;

            uniform sampler2D v_texture;

            layout(location = 0) out vec4 out_color;

            void main() {

                out_color = texture(v_texture, v_tex_coords) * v_color;
            }";


        let mut out = Self {
            _handle_gen: 0,
            _shaders: HashMap::new()
        };

        out.load_shader(
            vs_src,
            fs_src,
            "out_color",
            "v_texture",
            "position",
            "color",
            "tex_coords",
            "projection",
        );

        out
    }

    pub fn load_shader(
        &mut self,
        vert_source: &str,
        frag_source: &str,
        frag_data_location: &str,
        frag_texture_uniform: &str,
        vert_pos_name: &str,
        vert_color_name: &str,
        vert_tex_coords_name: &str,
        vert_proj_uniform: &str
    ) -> ShaderHandle {
        let handle = self._handle_gen;
        self._shaders.insert(handle, Shader::new(
            vert_source,
            frag_source,
            frag_data_location,
            frag_texture_uniform,
            vert_pos_name,
            vert_color_name,
            vert_tex_coords_name,
            vert_proj_uniform
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

    // TODO: implement better
    pub fn get_default(&self) -> Option<&Shader> {
        self._shaders.get(&0)
    }

}

