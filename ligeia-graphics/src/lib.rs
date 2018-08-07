extern crate bit_set;
extern crate gl;
extern crate ligeia_softcode;
extern crate ligeia_utils;
extern crate sdl2;


mod basic_verts;
mod color;
mod directional_sprite;
mod framebuffer;
mod ground_sprite;
mod layered_sprite;
mod managed_camera;
mod renderable;
mod shader;
mod shader_handler;
mod shadow;
mod sprite;
mod texture;
mod texture_handler;
mod vector2;
mod vertex;
mod window;

pub use basic_verts::{
    BASIC_VERTS,
    SCREEN_VERTS
};
pub use color::Color;
pub use directional_sprite::DirectionalSprite;
pub use framebuffer::Framebuffer;
pub use ground_sprite::GroundSprite;
pub use layered_sprite::LayeredSprite;
pub use managed_camera::ManagedCamera;
pub use renderable::Renderable;
pub use shader::Shader;
pub use shader_handler::{ShaderHandle, ShaderHandler};
pub use shadow::Shadow;
pub use sprite::Sprite;
pub use texture::Texture;
pub use texture_handler::{TextureHandle, TextureHandler};
pub use vector2::{Vector2, Vector2f, Vector2i, Vector2u};
pub use vertex::{Vertex, VERTEX_SIZE};
pub use window::Window;


#[cfg(test)]
mod tests {
    #[test]
    fn gfx_test() {
        use sdl2;
        use Window;
        use ShaderHandler;
        use Texture;
        use Vertex;

        println!("Graphics test!");

        let sdl_obj = sdl2::init().unwrap();

        let video = sdl_obj.video().unwrap();

        let mut window = Window::new(&video, 800, 600, 800, 600, "test!");

        let mut shader_handler = ShaderHandler::new();
        let vs_src: &str = "
            #version 330
            in vec2 position;
            in vec4 color;
            in vec2 tex_coords;

            out vec4 v_color;
            out vec2 v_tex_coords;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
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

        let shader_handle = shader_handler.load_shader(
            vs_src,
            fs_src,
            "out_color",
            "v_texture",
            "position",
            "color",
            "tex_coords"
        );
        let shader = shader_handler.get_shader(shader_handle).unwrap();

        let texture0 = Texture::new_from_memory(
            2, 2,
            vec![
                1., 0., 0., 1.,
                0., 1., 0., 1.,
                0., 0., 1., 1.,
                1., 1., 0., 1.,
            ]
        );

        let texture1 = Texture::new_from_file("../assets/textures/db32.png");


        let vertices = vec![
            Vertex::new(-0.5, 0.5, 1., 1., 1., 1., 0., 0.), // top left
            Vertex::new(0.5, -0.5, 1., 1., 1., 1., 1., 1.), // bottom right
            Vertex::new(-0.5, -0.5, 1., 1., 1., 1., 0., 1.),// bottom left

            Vertex::new(-0.5, 0.5, 1., 1., 1., 1., 0., 0.), // top left
            Vertex::new(0.5, -0.5, 1., 1., 1., 1., 1., 1.), // bottom right
            Vertex::new(0.5, 0.5, 1., 1., 1., 1., 1., 0.),  // top right
        ];
        for i in 0..2 {
            for j in 0..60 {
                window.clear();
                window.draw_vertices(&vertices[..], &texture0, shader);
                window.draw_framebuffer(shader);
                window.display();
            }
            for j in 0..60 {
                window.clear();
                window.draw_vertices(&vertices[..], &texture1, shader);
                window.draw_framebuffer(shader);
                window.display();
            }
        }
    }
}