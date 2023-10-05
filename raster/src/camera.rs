use crate::*;

pub struct Camera {
    frustum: Frustum,
    positon: Vec2,
    dir: Vec2,
}

pub struct Frustum {
    near: f32,
    far: f32,
    fov: f32
    ,
    aspect: f32,
}
