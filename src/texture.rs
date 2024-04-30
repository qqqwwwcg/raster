use crate::*;
use image::{open, ImageBuffer, Rgb};

pub struct Texture {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}
impl Texture {
    pub fn load(path: &str) -> Self {
        let buffer = open(path).unwrap().into_rgb8();
        let width = buffer.width();
        let height = buffer.height();
        Self {
            buffer,
            width,
            height,
        }
    }

    //TODO mip_map
    pub fn get_pixel(&self, texcoord: Vec2) -> Vec3 {
        self.buffer
            .get_pixel(
                (texcoord.x * self.width as f32) as u32,
                ((1.0 - texcoord.y) * self.height as f32) as u32, //纹理和图片的映射 1-y
            )
            .0
            .into()
    }
}

pub enum BaseColor {
    Color(Color),
    Map(BaseColorMap),
}
pub struct BaseColorMap {
    texture: Texture,
}
impl BaseColorMap {
    pub fn new(path: &str) -> Self {
        BaseColorMap {
            texture: Texture::load(path),
        }
    }

    pub fn get_color(&self, texcoord: Vec2) -> Vec3 {
        self.texture.get_pixel(texcoord)
    }
}

pub struct NormalMap {
    texture: Texture,
}
impl NormalMap {
    pub fn new(path: &str) -> Self {
        NormalMap {
            texture: Texture::load(path),
        }
    }

    pub fn get_normal(&self, texcoord: Vec2) -> Vec3 {
        let pixel = self.texture.get_pixel(texcoord);

        Vec3::new(
            pixel.x * 2.0 - 1.0,
            pixel.y * 2.0 - 1.0,
            pixel.z * 2.0 - 1.0,
        )
    }
}

pub struct SpecularMap {
    texture: Texture,
}
impl SpecularMap {
    pub fn new(path: &str) -> Self {
        SpecularMap {
            texture: Texture::load(path),
        }
    }

    pub fn get_specular(&self, texcoord: Vec2) -> Vec3 {
        self.texture.get_pixel(texcoord)
    }
}
