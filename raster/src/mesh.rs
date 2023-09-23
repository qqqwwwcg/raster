use crate::*;

pub struct Mesh{
    pub verteies:Vec<Vertex>,
    pub indies:Vec<usize>,
    pub topology:Topology,
    //pub material: Material,
}

pub struct Vertex{
    pub positon:Vec3,
    pub normal:Vec3,
    pub uv:Vec2,
    pub color:Vec3
}

pub enum Topology{
    TriangleList,
    LineList,
    PointList
}