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

        let mut x = x0;
        let mut y = y0;
        while x <= x1 {
            let pixel = match steep {
                Steep::X => (x as u32, y as u32),
                Steep::Y => (y as u32, x as u32),
            };
            self.frame_buffer.draw_pixel(pixel, color);

            x += 1.0;
            y += k; //optimize add k ,replace mul y = x * k +b,
        }
    }
    fn middle_point_draw_line(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        steep: Steep,
        color: Vec3,
    ) {
        let inc = if y1 < y0 { -1.0 } else { 1.0 };
        //f(x,y) = (y1 -y2) * x + (x2 - x1) * y + x1 * y2 - x2 * y1
        let f = |x: f32, y: f32| (y0 - y1) * x + (x1 - x0) * y + x0 * y1 - x1 * y0;

        let mut x = x0;
        let mut y = y0;

        while x <= x1 {
            let pixel = match steep {
                Steep::X => (x as u32, y as u32),
                Steep::Y => (y as u32, x as u32),
            };
            self.frame_buffer.draw_pixel(pixel, color);

            x += 1.0;
            if inc * f(x, y + 0.5 * inc) < 0. {
                y += inc
            }
        }
    }
    fn bresenham_draw_line(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        steep: Steep,
        color: Vec3,
    ) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let k = dy;
        let inc = if y1 < y0 { -1 } else { 1 };

        let mut delta = -inc * dx;
        let mut x = x0;
        let mut y = y0;

        while x <= x1 {
            let pixel = match steep {
                Steep::X => (x as u32, y as u32),
                Steep::Y => (y as u32, x as u32),
            };
            self.frame_buffer.draw_pixel(pixel, color);

            x += 1;
            delta += k << 1;
            if inc * delta > 0 {
                y += inc;
                delta -= inc * (dx << 1);
            }
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
            DrawLineAlgorithm::MiddlePoint => {
                self.middle_point_draw_line(x0, y0, x1, y1, steep, color)
            }
            DrawLineAlgorithm::Bresenham => {
                self.bresenham_draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, steep, color)
            }
        }
    }

    //Barycentric Coordinates incremental updating
    pub fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Vec3) {
        //get bounding
        let min = Vec2::new(a.x.min(b.x).min(c.x), a.y.min(b.y).min(c.y));
        let max = Vec2::new(a.x.max(b.x).max(c.x), a.y.max(b.y).max(c.y));

        let inv_beta = 1.0 / ((a.y - c.y) * b.x + (c.x - a.x) * b.y + a.x * c.y - c.x * a.y);
        let inv_gamma = 1.0 / ((a.y - b.y) * c.x + (b.x - a.x) * c.y + a.x * b.y - b.x * a.y);
        let delta_beta_x = (a.y - c.y) * inv_beta;
        let delta_beta_y = (c.x - a.x) * inv_beta;
        let beta0 = ((a.y - c.y) * min.x + (c.x - a.x) * min.y + a.x * c.y - c.x * a.y) * inv_beta;
        let delta_gamma_x = (a.y - b.y) * inv_gamma;
        let delta_gamma_y = (b.x - a.x) * inv_gamma;
        let gamma0 =
            ((a.y - b.y) * min.x + (b.x - a.x) * min.y + a.x * b.y - b.x * a.y) * inv_gamma;

        let mut x = min.x;
        let mut y = min.y;
        let mut beta = beta0;
        let mut gamma = gamma0;
        let mut alpha = 1.0 - beta - gamma;

        while y <= max.y {
            while x <= max.x {
                if alpha >= 0. && beta >= 0. && gamma >= 0. {
                    self.frame_buffer.draw_pixel((x as u32, y as u32), color)
                }

                x += 1.0;

                //Barycentric Coordinates incremental updating
                beta += delta_beta_x;
                gamma += delta_gamma_x;
                alpha = 1.0 - beta - gamma;
            }

            x = min.x;
            y += 1.0;
            beta = beta0 + (y - min.y) * delta_beta_y;
            gamma = gamma0 + (y - min.y) * delta_gamma_y;
            alpha = 1.0 - beta - gamma;
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
