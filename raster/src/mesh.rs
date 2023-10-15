use crate::*;

pub struct Mesh {
    pub verteies: Vec<Vertex>,
    pub indies: Vec<usize>,
    pub topology: TopologyList,
    //pub material: Material,
}

pub struct Vertex {
    pub positon: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub uv: Vec2<f32>,
    pub color: Vec3<f32>,
}

pub enum TopologyList {
    Triangle,
    Line,
    Point,
}
