use image;
use image::{
    ImageFormat,
    RgbaImage
};
use std::fs::File;
use std::io::{BufRead, Cursor, Read};

use FileError;

pub struct ImageLoader {
    _image_data: RgbaImage
}

impl ImageLoader {
    pub fn open(filename: &str) -> Result<Self, FileError> {
        let open_result = match File::open(filename) {
            Ok(ref mut file) => {
                let mut data_buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut data_buffer);
                Ok(data_buffer)
            },
            Err(_)   => Err(FileError::FileDataLoadError)
        };

        let format_check_result = match open_result {
            Ok(data_buffer) => {
                match image::guess_format(&data_buffer[..]) {
                    Ok(format) => Ok((format, data_buffer)),
                    Err(_)     => Err(FileError::FileMalformedError)
                }
            },
            Err(e) => Err(e)
        };

        let image_load_result = match format_check_result {
            Ok((format, data_buffer)) => {
                let bytes = Cursor::new(data_buffer);
                match image::load(bytes, format) {
                    Ok(dynamic_image) => Ok(
                        Self {
                            _image_data: dynamic_image.to_rgba()
                        }
                    ),
                    Err(_) => Err(FileError::FileMalformedError)
                }
            },
            Err(e) => Err(e)
        };

        image_load_result
    }

    pub fn width(&self) -> u32 {
        self._image_data.width()
    }

    pub fn height(&self) -> u32 {
        self._image_data.height()
    }

    pub fn pixel_data(&self) -> Vec<f32> {
        let mut out = Vec::new();
        for pixel in self._image_data.pixels() {
            for color in pixel.data.iter() {
                let f_color = (*color as f32) / 255.;
                out.push(f_color);
            }
        }

        out
    }

}