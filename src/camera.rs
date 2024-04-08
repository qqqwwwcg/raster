use crate::*;

pub struct Camera {
    frustum: Frustum,
    position: Vec3, //eye
    dir: Vec3,
    //center:Vec3 origin (0,0,0)
}
impl Camera {
    pub fn new(frustum: Frustum, position: Vec3, dir: Vec3) -> Self {
        Camera {
            frustum,
            position,
            dir,
        }
    }

    pub fn look_at(&self) -> Matrix {
        let z = self.dir.normalize();
    }

    pub fn move_offset(&mut self, offset: Vec3) {
        self.position += offset;
        self.dir = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        } - self.position;
    }
}

#[derive(Default)]
pub struct Frustum {
    near: f32,
    far: f32,
    fov: f32,
    aspect: f32, //width/height
}
impl Frustum {
    pub fn orth_projection(&self) -> Matrix {
        let h = 2.0 * self.near * (fov * 0.5).tan();
        let w = h * self.aspect;
        #[rustfmt::skip]
        let translate = Matrix::new(
            1.0,    0.0,    0.0,    0.0,
            0.0,    1.0,    0.0,    0.0,
            0.0,    0.0,    1.0,    -0.5*(self.near+self.far),
            0.0,    0.0,    0.0,    1.0
        );
        #[rustfmt::skip]
        let scale = Matrix::new(
            2.0 / w,    0.0,    0.0,    0.0,
            0.0,    2.0 / h,    0.0,    0.0,
            0.0,    0.0,    2.0 / (far - near), 0.0,
            0.0,    0.0,    0.0,    1.0,
        );

        scale * translate
    }

    pub fn perspective_projection(&self) -> Matrix {
        #[rustfmt::skip]
        let per_to_orth = Matrix::new(
            self.near,    0.0,    0.0,    0.0,
            0.0,    self.near,    0.0,    0.0,
            0.0,    0.0,   (self.near + self.far), -self.near*self.far,
            0.0,    0.0,    0.0,    1.0,
        );

        self.orth_projection() * per_to_orth
    }
}
