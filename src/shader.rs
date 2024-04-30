use crate::*;

pub type VertexShader = Box<dyn Fn(&Vertex, &Matrix4, &Matrix4) -> Uniform>; //(world vertex for lighting, ndc position)
pub type FragmentShader =
    Box<dyn Fn(&Fragment, &Material, &PointLight, &Vec3, &Matrix4) -> ShadedFragment>; //(screen_pos,depth,color)
pub struct Shader {
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
}
impl Shader {
    pub fn new(vertex_shader: VertexShader, fragment_shader: FragmentShader) -> Self {
        Self {
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn run_vertex_shader(
        &self,
        vertex: &Vertex,
        mvp: &Matrix4,
        model_mat: &Matrix4,
    ) -> Uniform {
        (self.vertex_shader)(vertex, mvp, model_mat)
    }

    pub fn run_fragment_shader(
        &self,
        frame: &Fragment,
        material: &Material,
        light: &PointLight,
        camera_position: &Vec3,
        model_mat: &Matrix4,
    ) -> ShadedFragment {
        (self.fragment_shader)(frame, material, light, camera_position, model_mat)
    }
}

pub struct ShadedFragment {
    pub screen_pos: (u32, u32),
    pub depth: f32,
    pub color: Color,
}
impl ShadedFragment {
    pub fn new(screen_pos: (u32, u32), depth: f32, color: Color) -> Self {
        Self {
            screen_pos,
            depth,
            color,
        }
    }
}
