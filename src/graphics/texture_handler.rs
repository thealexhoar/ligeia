use ligeia_softcode::graphics::TextureDef;
use ligeia_utils::rect::{FloatRect, UIntRect};
use sfml::graphics::{BlendMode, Color, blend_mode::Factor, PrimitiveType, RenderStates, RenderTarget, RenderTexture, Texture, Vertex, View};
use sfml::system::{SfBox, Vector2f};
use std::collections::HashMap;
use std::ops::Deref;

use graphics::{BASIC_VERTS, ShaderHandler};

pub type TextureHandle = u32;

static MASTER_TEXTURE_SIZE: u32 = 1024;
static MASTER_TEXTURE_SIZE_F: f32 = 1024.;
static CLEAR_COLOR: Color = Color::TRANSPARENT;

pub struct TextureHandler {
    _handle_gen: TextureHandle,
    _master_texture: RenderTexture,
    _textures: HashMap<TextureHandle, SfBox<Texture>>,
    _sub_textures: HashMap<String, Vec<FloatRect>>
}

impl TextureHandler {
    pub fn new() -> Self {
        Self {
            _handle_gen: 0,
            _master_texture: RenderTexture::new(MASTER_TEXTURE_SIZE, MASTER_TEXTURE_SIZE, false).unwrap(),
            _textures: HashMap::new(),
            _sub_textures: HashMap::new()
        }
    }

    pub fn create_master_texture<'a>(&mut self, texture_defs: Vec<TextureDef>, shader_handler: &ShaderHandler<'a>) {
        let mut textures = Vec::<(String, SfBox<Texture>, Vec<UIntRect>)>::with_capacity(texture_defs.len());

        self._sub_textures.clear();

        for texture_def in &texture_defs {
            textures.push((
                texture_def.filename.clone(),
                Texture::from_file(texture_def.filename.as_str()).unwrap(),
                texture_def.frames.clone()
            ));
        }

        //sort by height
        textures.sort_by(|t1, t2| t2.1.deref().size().y.cmp(&t1.1.deref().size().y));

        let view = View::new(
            Vector2f { x: MASTER_TEXTURE_SIZE_F * 0.5, y: MASTER_TEXTURE_SIZE_F * 0.5},
            Vector2f { x: MASTER_TEXTURE_SIZE_F, y: -MASTER_TEXTURE_SIZE_F}
        );
        self._master_texture.set_view(&view);
        self._master_texture.clear(&CLEAR_COLOR);

        let mut top: u32 = 0;
        let mut left: u32 = 0;
        let mut next_top: u32 = 0;
        for i in 0..textures.len() {
            let (ref filename, ref next_texture, ref frames) = textures[i];
            let width = next_texture.size().x;
            let height = next_texture.size().y;


            if left + width >= MASTER_TEXTURE_SIZE {
                top = next_top;
                left = 0;
            }
            if left == 0 {
                next_top = top + height;
            }

            let mut rs = RenderStates::default();
            rs.blend_mode = BlendMode::default();
            rs.blend_mode.color_src_factor = Factor::One;
            rs.blend_mode.color_dst_factor = Factor::Zero;
            rs.blend_mode.alpha_src_factor = Factor::One;
            rs.blend_mode.alpha_dst_factor = Factor::Zero;
            rs.shader = shader_handler.get_default();
            rs.texture = Some(next_texture.deref());

            let vertices: [Vertex; 4] = [
                Vertex {
                    position: Vector2f {x: left as f32, y: top as f32},
                    color: Color::WHITE,
                    tex_coords: Vector2f {x: 0., y: 0.}
                },
                Vertex {
                    position: Vector2f {x: (left + width) as f32, y: top as f32},
                    color: Color::WHITE,
                    tex_coords: Vector2f {x: 1., y: 0.}
                },
                Vertex {
                    position: Vector2f {x: (left + width) as f32, y: (top + height) as f32},
                    color: Color::WHITE,
                    tex_coords: Vector2f {x: 1., y: 1.}
                },
                Vertex {
                    position: Vector2f {x: left as f32, y: (top + height) as f32},
                    color: Color::WHITE,
                    tex_coords: Vector2f {x: 0., y: 1.}
                },
            ];

            self._master_texture.draw_primitives(&vertices, PrimitiveType::Quads, rs);

            let mut frame_vec = Vec::with_capacity(frames.len());

            for i in 0..frames.len() {
                let pixel_frame = frames[i];
                let uv_frame = FloatRect::new(
                    (pixel_frame.left + left) as f32 / MASTER_TEXTURE_SIZE_F,
                    (pixel_frame.top + top) as f32 / MASTER_TEXTURE_SIZE_F,
                    (pixel_frame.width) as f32 / MASTER_TEXTURE_SIZE_F,
                    (pixel_frame.height) as f32 / MASTER_TEXTURE_SIZE_F
                );
                frame_vec.push(uv_frame);
            }
            self._sub_textures.insert(filename.clone(), frame_vec);

            left += width;
        }
        let total_pix = MASTER_TEXTURE_SIZE * MASTER_TEXTURE_SIZE;
        let used_pix = MASTER_TEXTURE_SIZE * top + (next_top - top) * left;
        let percent_pix = (used_pix as f32) / (total_pix as f32) * 100.;
        println!(
            "Master Texture Created: {}^2 ({}) px, USED: {}px / {:.2}%",
            MASTER_TEXTURE_SIZE,
            total_pix,
            used_pix,
            percent_pix
        );
        self._master_texture.display();
    }

    pub fn load_texture(&mut self, filename: &str) -> Option<TextureHandle> {
        match Texture::from_file(filename) {
            Some(t_box) => {
                self._textures.insert(self._handle_gen, t_box);
                self._handle_gen += 1;
                Some(self._handle_gen - 1)
            },
            None        => None
        }
    }

    pub fn unload_texture(&mut self, handle: TextureHandle) -> bool {
        match self._textures.remove(&handle) {
            Some(_) => true,
            None    => false
        }
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        match self ._textures.get(&handle) {
            Some(t_box) => Some(t_box.deref()),
            None        => None
        }
    }

    pub fn get_master_texture(&self) -> &Texture {
        self._master_texture.texture()
    }

    pub fn get_subrects(&self, filename: String) -> &Vec<FloatRect> {
        &self._sub_textures[&filename]
    }
}