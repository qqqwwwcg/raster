#[derive(Copy, Clone, Default)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Vec3<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
