use crate::ShadedFragment;

pub type Color = [u8; 3];

pub struct Buffer<T: Copy + Default> {
    width: u32,
    height: u32,
    data: Vec<T>,
}
impl<T: Copy + Default> Buffer<T> {
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
        self.width as usize * self.height as usize
    }

    pub fn reset(&mut self, default: T) {
        self.data.iter_mut().for_each(|element| *element = default)
    }
}

pub struct FrameBuffer {
    buffer: Buffer<Color>,
}
impl FrameBuffer {
    pub fn new(width: u32, height: u32, default: Color) -> Self {
        Self {
            buffer: Buffer::new_with_capacity(width, height, default),
        }
    }
    pub fn draw_pixel(&mut self, pixel: (u32, u32), color: Color) {
        self.buffer.set_value(pixel.0, pixel.1, color)
    }
    pub fn flatten(&self) -> Vec<u8> {
        let mut flatten = Vec::with_capacity(self.buffer.get_len());
        self.buffer.get_buffer().iter().for_each(|color| {
            flatten.push(color[0]);
            flatten.push(color[1]);
            flatten.push(color[2]);
        });

        flatten
    }
    pub fn reset(&mut self, default: Color) {
        self.buffer.reset(default)
    }
}

pub struct DepthBuffer {
    buffer: Buffer<f32>,
}
impl DepthBuffer {
    pub fn new(width: u32, height: u32, default: f32) -> Self {
        Self {
            buffer: Buffer::new_with_capacity(width, height, default),
        }
    }

    pub fn depth_test(&self, shaded_fragment: &ShadedFragment) -> bool {
        shaded_fragment.depth
            > *self
                .buffer
                .get_value(shaded_fragment.screen_pos.0, shaded_fragment.screen_pos.1)
                .unwrap()
    }

    pub fn depth_write(&mut self, shaded_fragment: &ShadedFragment) {
        self.buffer.set_value(
            shaded_fragment.screen_pos.0,
            shaded_fragment.screen_pos.1,
            shaded_fragment.depth,
        )
    }

    pub fn reset(&mut self, default: f32) {
        self.buffer.reset(default)
    }
}
