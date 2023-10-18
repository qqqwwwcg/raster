use crate::*;

pub struct Render {
    //camera: Camera,
    frame_buffer: FrameBuffer,
    depth_buffer: DepthBuffer,
}
impl Render {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            frame_buffer: FrameBuffer::new(width, height),
            depth_buffer: DepthBuffer::new_with_capacity(width, height, f32::NEG_INFINITY),
        }
    }
    fn digital_differential_analyzer_draw_line(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        steep: Steep,
        color: Vec3,
    ) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let k = dy / dx;

        let mut x0 = x0;
        let mut y0 = y0;
        while x0 < x1 {
            let pixel = match steep {
                Steep::X => (x0 as u32, y0 as u32),
                Steep::Y => (y0 as u32, x0 as u32),
            };
            self.frame_buffer.draw_pixel(pixel, color);

            x0 += 1.0;
            y0 += k; //optimize add k ,replace mul y = x0 * k +b,
        }
    }

    pub fn draw_point(&mut self, point: Vec2, color: Vec3) {
        self.frame_buffer
            .draw_pixel((point.x as u32, point.y as u32), color)
    }
    pub fn draw_line(&mut self, start: Vec2, end: Vec2, color: Vec3, algorithm: DrawLineAlgorithm) {
        if start.x == end.x && start.y == end.y {
            //draw line downgrade to draw point
            self.frame_buffer
                .draw_pixel((start.x as u32, start.y as u32), color);
            return;
        }

        let steep = if (end.y - start.y).abs() > (end.x - start.x).abs() {
            Steep::Y
        } else {
            Steep::X
        };
        let (mut x0, mut y0, mut x1, mut y1) = match steep {
            Steep::X => (start.x, start.y, end.x, end.y),
            Steep::Y => (start.y, start.x, end.y, end.x), //x,y -> y,x && make sure dx !=0
        };
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1); //make sure x0<x1, while x0 < x1
            std::mem::swap(&mut y0, &mut y1);
        }

        match algorithm {
            DrawLineAlgorithm::DDA => {
                self.digital_differential_analyzer_draw_line(x0, y0, x1, y1, steep, color)
            }
            DrawLineAlgorithm::MiddlePoint => {}
            DrawLineAlgorithm::Bresenham => {}
        }
    }

    pub fn get_frame(&self) -> Vec<u8> {
        self.frame_buffer.flatten()
    }
}

pub enum DrawLineAlgorithm {
    DDA, //Digital Differential Analyzer
    MiddlePoint,
    Bresenham,
}

enum Steep {
    X,
    Y,
}
