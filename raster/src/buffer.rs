use crate::math::Vec3;

pub struct Buffer<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}
impl<T: Copy> Buffer<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            data: vec![default; width * height],
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: T) {
        self.data[self.width * row + col] = value
    }
    pub fn get_value(&self, row: usize, col: usize) -> T {
        self.data[self.width * row + col]
    }
}

pub type DepthBuffer = Buffer<f32>;
pub type FrameBuffer = Buffer<Vec3>;
impl FrameBuffer {
    pub fn flatten(&self) -> Vec<u8> {
        let mut flatten = Vec::with_capacity(self.height * self.width * 3);
        self.data.iter().for_each(|color| {
            flatten.push(color.x as u8);
            flatten.push(color.y as u8);
            flatten.push(color.z as u8);
        });

        flatten
    }
}
