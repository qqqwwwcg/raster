use crate::*;

pub struct Render {
    width: u32,
    height: u32,
    frame_buffer: FrameBuffer,
    depth_buffer: DepthBuffer,
    camera: Camera,
    shader: Shader,
}
impl Render {
    pub fn new(width: u32, height: u32, camera: Camera, shader: Shader) -> Self {
        Self {
            width,
            height,
            frame_buffer: FrameBuffer::new(width, height, [0; 3]),
            depth_buffer: DepthBuffer::new(width, height, -1.0),
            camera,
            shader,
        }
    }
    pub fn reset(&mut self) {
        self.frame_buffer.reset([0; 3]);
        self.depth_buffer.reset(-1.0);
    }
    fn digital_differential_analyzer_draw_line(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        steep: Steep,
        color: Color,
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
        color: Color,
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
        color: Color,
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

    pub fn draw_point(&mut self, point: Vec2, color: Color) {
        self.frame_buffer
            .draw_pixel((point.x as u32, point.y as u32), color)
    }

    pub fn draw_line(
        &mut self,
        start: Vec2,
        end: Vec2,
        color: Color,
        algorithm: DrawLineAlgorithm,
    ) {
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
    pub fn raster_triangle(&mut self, triangle: &Triangle) -> Vec<Fragment> {
        let a = triangle.a.position;
        let b = triangle.b.position;
        let c = triangle.c.position;

        //Barycentric Coordinates
        let barycentric_coordinates = |p: Vec2| {
            let gamma = ((a.y - b.y) * p.x + (b.x - a.x) * p.y + a.x * b.y - b.x * a.y)
                / ((a.y - b.y) * c.x + (b.x - a.x) * c.y + a.x * b.y - b.x * a.y);
            let beta = ((a.y - c.y) * p.x + (c.x - a.x) * p.y + a.x * c.y - c.x * a.y)
                / ((a.y - c.y) * b.x + (c.x - a.x) * b.y + a.x * c.y - c.x * a.y);
            (1.0 - beta - gamma, beta, gamma)
        };

        //get bounding
        let min_x = a.x.min(b.x).min(c.x) as u32;
        let min_y = a.y.min(b.y).min(c.y) as u32;
        let max_x = a.x.max(b.x).max(c.x) as u32;
        let max_y = a.y.max(b.y).max(c.y) as u32;

        let mut fragments = vec![];
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let pixel_center = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                //TODO 透视投影矫正
                let (alpha, beta, gamma) = barycentric_coordinates(pixel_center);
                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
                    let frame = triangle.create_frame(Vec3::new(alpha, beta, gamma), (x, y));
                    fragments.push(frame)
                }
            }
        }

        fragments
    }

    pub fn draw(&mut self, mesh: &Mesh, light: &PointLight, model_mat: Matrix4) {
        //Vertex Shader
        let mvp = self.camera.get_projection_matrix() * self.camera.get_view_matrix() * model_mat;
        let uniforms: Vec<Uniform> = mesh
            .vertexes
            .iter()
            .map(|vertex| self.shader.run_vertex_shader(vertex, &mvp, &model_mat))
            .collect();

        //Primitive Assembly
        let n_face = mesh.indies.len() / 3;
        let mut triangles = Vec::with_capacity(n_face);
        for i in 0..n_face {
            let triangle = Triangle::new(
                uniforms[mesh.indies[3 * i]],
                uniforms[mesh.indies[3 * i + 1]],
                uniforms[mesh.indies[3 * i + 2]],
            );
            triangles.push(triangle)
        }

        //back face culling
        let camera_dir = self.camera.get_dir();
        triangles.retain(|triangle| triangle.get_world_normal().dot(&camera_dir) < 0.0);

        //near & far plane culling
        triangles.retain(|triangle| !triangle.ndc_culling_test());

        //TODO:clipping

        //screen mapping
        let width = self.width as f32;
        let height = self.height as f32;
        triangles.iter_mut().for_each(|triangle| {
            //Screen origin is Top left corner
            let view_prot_transform = |ndc: &mut Vec3| {
                ndc.x = (ndc.x + 1.0) * 0.5 * width as f32;
                ndc.y = (-ndc.y + 1.0) * 0.5 * height as f32;
            };
            view_prot_transform(&mut triangle.a.position);
            view_prot_transform(&mut triangle.b.position);
            view_prot_transform(&mut triangle.c.position);
        });

        //Rasterization
        let fragments: Vec<Fragment> = triangles
            .iter()
            .flat_map(|triangle| self.raster_triangle(triangle))
            .collect();

        //Fragment Shader
        let camera_position = self.camera.get_position();
        let shaded_fragments: Vec<ShadedFragment> = fragments
            .iter()
            .map(|fragment| {
                self.shader.run_fragment_shader(
                    fragment,
                    &mesh.material,
                    light,
                    &camera_position,
                    &model_mat,
                )
            })
            .collect();

        //Output merge
        //depth test
        shaded_fragments.iter().for_each(|shaded_fragment| {
            if self.depth_buffer.depth_test(shaded_fragment) {
                self.depth_buffer.depth_write(shaded_fragment);

                self.frame_buffer
                    .draw_pixel(shaded_fragment.screen_pos, shaded_fragment.color)
            }
        });
    }
    pub fn get_frame(&self) -> Vec<u8> {
        self.frame_buffer.flatten()
    }
    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
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
