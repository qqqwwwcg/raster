use std::ops::{Add, Mul};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn cross(&self, rhs: &Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }
}
impl Mul<Vec2> for Vec2 {
    type Output = f32;
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(rhs.x * self, rhs.y * self)
    }
}
impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn downgrade(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Vec3::new(value[0], value[1], value[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Vec4::new(value.x, value.y, value.z, 1.0)
    }
}
impl From<Vec4> for Vec3 {
    fn from(value: Vec4) -> Self {
        let w_inv = 1.0 / value.w;
        Self::new(value.x * w_inv, value.y * w_inv, value.z * w_inv)
    }
}
impl Mul<Vec4> for f32 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(rhs.x * self, rhs.y * self, rhs.z * self, rhs.w * self)
    }
}

pub struct Matrix4 {
    elements: [f32; 16],
}
impl Matrix4 {
    pub fn new(elements: [f32; 16]) -> Self {
        Self { elements }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.elements[row * 4 + col]
    }
}
impl Mul<Vec4> for Matrix4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.get(0, 0) * rhs.x
                + self.get(0, 1) * rhs.y
                + self.get(0, 2) * rhs.z
                + self.get(0, 3) * rhs.w,
            self.get(1, 0) * rhs.x
                + self.get(1, 1) * rhs.y
                + self.get(1, 2) * rhs.z
                + self.get(1, 3) * rhs.w,
            self.get(2, 0) * rhs.x
                + self.get(2, 1) * rhs.y
                + self.get(2, 2) * rhs.z
                + self.get(2, 3) * rhs.w,
            self.get(3, 0) * rhs.x
                + self.get(3, 1) * rhs.y
                + self.get(3, 2) * rhs.z
                + self.get(3, 3) * rhs.w,
        )
    }
}
impl Mul<Matrix4> for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        Self::new([
            self.get(0, 0) * rhs.get(0, 0)
                + self.get(0, 1) * rhs.get(1, 0)
                + self.get(0, 2) * rhs.get(2, 0)
                + self.get(0, 3) * rhs.get(3, 0),
            self.get(0, 0) * rhs.get(0, 1)
                + self.get(0, 1) * rhs.get(1, 1)
                + self.get(0, 2) * rhs.get(2, 1)
                + self.get(0, 3) * rhs.get(3, 1),
            self.get(0, 0) * rhs.get(0, 2)
                + self.get(0, 1) * rhs.get(1, 2)
                + self.get(0, 2) * rhs.get(2, 2)
                + self.get(0, 3) * rhs.get(3, 2),
            self.get(0, 0) * rhs.get(0, 3)
                + self.get(0, 1) * rhs.get(1, 3)
                + self.get(0, 2) * rhs.get(2, 3)
                + self.get(0, 3) * rhs.get(3, 3),
            self.get(1, 0) * rhs.get(0, 0)
                + self.get(1, 1) * rhs.get(1, 0)
                + self.get(1, 2) * rhs.get(2, 0)
                + self.get(1, 3) * rhs.get(3, 0),
            self.get(1, 0) * rhs.get(0, 1)
                + self.get(1, 1) * rhs.get(1, 1)
                + self.get(1, 2) * rhs.get(2, 1)
                + self.get(1, 3) * rhs.get(3, 1),
            self.get(1, 0) * rhs.get(0, 2)
                + self.get(1, 1) * rhs.get(1, 2)
                + self.get(1, 2) * rhs.get(2, 2)
                + self.get(1, 3) * rhs.get(3, 2),
            self.get(1, 0) * rhs.get(0, 3)
                + self.get(1, 1) * rhs.get(1, 3)
                + self.get(1, 2) * rhs.get(2, 3)
                + self.get(1, 3) * rhs.get(3, 3),
            self.get(2, 0) * rhs.get(0, 0)
                + self.get(2, 1) * rhs.get(1, 0)
                + self.get(2, 2) * rhs.get(2, 0)
                + self.get(2, 3) * rhs.get(3, 0),
            self.get(2, 0) * rhs.get(0, 1)
                + self.get(2, 1) * rhs.get(1, 1)
                + self.get(2, 2) * rhs.get(2, 1)
                + self.get(2, 3) * rhs.get(3, 1),
            self.get(2, 0) * rhs.get(0, 2)
                + self.get(2, 1) * rhs.get(1, 2)
                + self.get(2, 2) * rhs.get(2, 2)
                + self.get(2, 3) * rhs.get(3, 2),
            self.get(2, 0) * rhs.get(0, 3)
                + self.get(2, 1) * rhs.get(1, 3)
                + self.get(2, 2) * rhs.get(2, 3)
                + self.get(2, 3) * rhs.get(3, 3),
            self.get(3, 0) * rhs.get(0, 0)
                + self.get(3, 1) * rhs.get(1, 0)
                + self.get(3, 2) * rhs.get(2, 0)
                + self.get(3, 3) * rhs.get(3, 0),
            self.get(3, 0) * rhs.get(0, 1)
                + self.get(3, 1) * rhs.get(1, 1)
                + self.get(3, 2) * rhs.get(2, 1)
                + self.get(3, 3) * rhs.get(3, 1),
            self.get(3, 0) * rhs.get(0, 2)
                + self.get(3, 1) * rhs.get(1, 2)
                + self.get(3, 2) * rhs.get(2, 2)
                + self.get(3, 3) * rhs.get(3, 2),
            self.get(3, 0) * rhs.get(0, 3)
                + self.get(3, 1) * rhs.get(1, 3)
                + self.get(3, 2) * rhs.get(2, 3)
                + self.get(3, 3) * rhs.get(3, 3),
        ])
    }
}
