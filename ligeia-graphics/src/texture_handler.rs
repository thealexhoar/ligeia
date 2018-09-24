use ligeia_softcode::graphics::TextureDef;
use ligeia_utils::rect::{FloatRect, UIntRect};

use std::collections::HashMap;
use std::ops::Deref;

//TODO: refactor to not use sfml
use {BASIC_VERTS, Color, ShaderHandler, RenderTexture, Texture, Vector2f, Vertex};

pub type TextureHandle = u32;

static MASTER_TEXTURE_SIZE: u32 = 1024;
static MASTER_TEXTURE_SIZE_F: f32 = 1024.;
static CLEAR_COLOR: Color = Color::TRANSPARENT;

pub struct TextureHandler {
    _handle_gen: TextureHandle,
    _master_texture: RenderTexture,
    _textures: HashMap<TextureHandle, Texture>,
    _sub_textures: HashMap<String, Vec<FloatRect>>
}

impl TextureHandler {
    pub fn new() -> Self {
        let mut out = Self {
            _handle_gen: 0,
            _master_texture: RenderTexture::new(MASTER_TEXTURE_SIZE as i32,  MASTER_TEXTURE_SIZE as i32),
            _textures: HashMap::new(),
            _sub_textures: HashMap::new()
        };

        out.add_texture(Texture::new_from_memory(
            1, 1,
            vec![
                1., 1., 1., 1.
            ]
        ));

        out.add_texture(Texture::new_from_memory(
            2, 2,
            vec![
                1., 0., 0., 1.,
                0., 1., 0., 1.,
                0., 0., 1., 1.,
                1., 1., 0., 1.,
            ]
        ));

        out
    }

    pub fn create_master_texture<'a>(&mut self, texture_defs: Vec<TextureDef>, shader_handler: &ShaderHandler) {
        let mut textures: Vec<(String, Texture, Vec<UIntRect>)> = Vec::with_capacity(texture_defs.len());
        self._sub_textures.clear();
        for texture_def in &texture_defs {
            textures.push((
                texture_def.filename.clone(),
                Texture::new_from_file(texture_def.filename.as_str()),
                texture_def.frames.clone()
            ));
        }

        //sort by height
        textures.sort_by(|t1, t2| t2.1.size().y.cmp(&t1.1.size().y));

        self._master_texture.clear();

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

            let mut vertices: [Vertex; 6] = [
                Vertex::new(
                    left as f32, top as f32,
                    1., 1., 1., 1.,
                    0., 0.
                ),
                Vertex::new(
                    (left + width) as f32, top as f32,
                    1., 1., 1., 1.,
                    1., 0.
                ),
                Vertex::new(
                    (left + width) as f32, (top + height) as f32,
                    1., 1., 1., 1.,
                    1., 1.
                ),
                Vertex::new(
                    left as f32, top as f32,
                    1., 1., 1., 1.,
                    0., 0.
                ),
                Vertex::new(
                    (left + width) as f32, (top + height) as f32,
                    1., 1., 1., 1.,
                    1., 1.
                ),
                Vertex::new(
                    left as f32, (top + height) as f32,
                    1., 1., 1., 1.,
                    0., 1.
                ),
            ];

            for i in 0..6 {
                let (x, y) = vertices[i].position_xy();
                vertices[i].set_position_xy(
                    x - MASTER_TEXTURE_SIZE_F * 0.5,
                    y - MASTER_TEXTURE_SIZE_F * 0.5
                );
            }

            self._master_texture.draw_vertices(
                &vertices,
                next_texture,
                shader_handler.get_default().unwrap(),
                None
            );

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
    }

    pub fn load_texture(&mut self, filename: &str) -> Option<TextureHandle> {
        // TODO: implement
        None
    }

    pub fn add_texture(&mut self, texture: Texture) -> TextureHandle {
        self._textures.insert(self._handle_gen, texture);
        let out = self._handle_gen;
        self._handle_gen += 1;

        out
    }

    pub fn unload_texture(&mut self, handle: TextureHandle) -> bool {
        match self._textures.remove(&handle) {
            Some(_) => true,
            None    => false
        }
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        match self ._textures.get(&handle) {
            Some(texture) => Some(texture),
            None        => None
        }
    }

    pub fn get_master_texture(&self) -> &Texture {
        self._master_texture.texture()
    }

    pub fn get_blank_texture(&self) -> &Texture {
        self._textures.get(&0).unwrap()
    }

    pub fn get_simple_texture(&self) -> &Texture {
        self._textures.get(&1).unwrap()
    }

    pub fn get_subrects(&self, filename: String) -> &Vec<FloatRect> {
        &self._sub_textures[&filename]
    }
}