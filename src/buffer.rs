use crate::math::{Vec2, Vec3};

pub struct Buffer<T: Copy> {
    width: u32,
    height: u32,
    data: Vec<T>,
}
impl<T: Copy> Buffer<T> {
    pub fn new_with_capacity(width: u32, height: u32, default: T) -> Self {
        Self {
            width,
            height,
            data: vec![default; width as usize * height as usize],
        }
    }

    pub fn set_value(&mut self, x: u32, y: u32, value: T) {
        let index = x as usize + y as usize * self.width as usize;
        if self.data.len() > index {
            self.data[index] = value
        }
    }
    pub fn get_value(&self, x: u32, y: u32) -> Option<&T> {
        let index = x as usize + y as usize * self.width as usize;
        self.data.get(index)
    }
    pub fn get_buffer(&self) -> &[T] {
        self.data.as_slice()
    }
    pub fn get_len(&self) -> usize {
        self.data.len()
    }
}

pub type DepthBuffer = Buffer<f32>;
pub struct FrameBuffer {
    buffer: Buffer<Vec3>,
}
impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: Buffer::new_with_capacity(width, height, Vec3::default()),
        }
    }
    pub fn draw_pixel(&mut self, pixel: (u32, u32), color: Vec3) {
        self.buffer.set_value(pixel.0, pixel.1, color)
    }
    pub fn flatten(&self) -> Vec<u8> {
        let mut flatten = Vec::with_capacity(self.buffer.get_len());
        self.buffer.get_buffer().iter().for_each(|color| {
            flatten.push(color.x as u8);
            flatten.push(color.y as u8);
            flatten.push(color.z as u8);
        });

        flatten
    }
}
