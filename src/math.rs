use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use crate::Color;


#[derive(Copy, Clone, Default, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn cross(&self, rhs: Self) -> f32 {
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
impl  Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
         Vec2::new(self.x - rhs.x, self.y -rhs.y)
    }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const X: Self = Vec3{x:1.0,y:0.0,z:0.0};
    pub const Y: Self =  Vec3{x:0.0,y:1.0,z:0.0};
    pub const Z: Self =  Vec3{x:0.0,y:0.0,z:1.0};

    pub fn zero()->Self{
         Vec3{x:0.0,y:0.0,z:0.0}
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn min(&self,rhs:&Self)->Self{
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }
        pub fn max(&self,rhs:&Self)->Self{
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    pub fn dot(&self,rhs: &Self)->f32{
        self.x*rhs.x+self.y*rhs.y+self.z*rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
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
    pub fn upgrade(&self)->Vec4{
        Vec4::new(self.x, self.y, self.z, 1.0) 
    }
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let inv = 1.0 / len;
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }

    pub fn length2(&self)->f32{
        self.dot(self)
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Vec3::new(value[0], value[1], value[2])
    }
}
impl From<Color> for Vec3{
    fn from(color: Color) -> Self {
        Vec3::new(color[0] as f32 /255.0, color[1]as f32 /255.0, color[2]as f32 /255.0)
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
impl Add<Vec3>for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
         Vec3::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}
impl Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z)
    }
}
impl AddAssign<Vec3> for Vec3{
    fn add_assign(&mut self, rhs: Vec3) {
        self.x+=rhs.x;
        self.y+=rhs.y;
        self.z+=rhs.z;
    }
}
impl Neg for Vec3{
    type Output=Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x,-self.y,-self.z)
    }
}

#[derive(Copy, Clone, Default,Debug)]
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

    pub fn perspective_divide(& self)->Vec3{
        let w_inv =1.0/self.w;
        Vec3::new(self.x*w_inv, self.y*w_inv, self.z*w_inv)
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

#[derive(Clone, Copy)]
pub struct Matrix3 {
    elements: [f32; 9],
}
impl Matrix3{
    pub fn new(elements:[f32;9])->Self{
        Self { elements }
    }
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.elements[row * 3 + col]
    }
    pub fn det(&self)->f32{
        self.get(0, 0)*self.get(1, 1)*self.get(2, 2)
        +    self.get(1, 0)*self.get(2, 1)*self.get(0, 2)
        +    self.get(2, 0)*self.get(0, 1)*self.get(1   , 2)
        -     self.get(0, 0)*self.get(2, 1)*self.get(1, 2)
        -     self.get(1, 0)*self.get(0, 1)*self.get(2, 2)
        -     self.get(2, 0)*self.get(1, 1)*self.get(0, 2)
    }
}
#[derive(Clone, Copy,Debug)]
pub struct Matrix4 {
    elements: [f32; 16],
}
impl Matrix4 {
    pub fn new(elements: [f32; 16]) -> Self {
        Self { elements }
    }

    pub fn mul(&self,rhs:Vec4)->Vec4{
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
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.elements[row * 4 + col]
    }
    pub fn set(&mut self,row:usize,col:usize,element:f32){
        self.elements[row*4+col]=element
    }
    #[rustfmt::skip]
    pub fn ident() -> Self {
        Self::new([
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate(translation:Vec3)->Self{
                Self::new([
            1.0, 0.0, 0.0, translation.x, 
            0.0, 1.0, 0.0,translation.y, 
            0.0, 0.0, 1.0,translation.z, 
            0.0, 0.0, 0.0, 1.0,
        ])

    }

    pub fn transpose(&self)->Self{
        Self::new([
            self.get(0, 0),self.get(1, 0),self.get(2, 0),self.get(3, 0),
            self.get(0, 1),self.get(1, 1),self.get(2, 1),self.get(3, 1),
            self.get(0, 2),self.get(1, 2),self.get(2, 2),self.get(3, 2),
            self.get(0, 3),self.get(1, 3),self.get(2, 3),self.get(3, 3),
            ])
    }
    //图省事儿 直接暴力解了
    pub fn det(&self)->f32{
        self.get(0, 0)*Matrix3::new([self.get(1, 1),self.get(1, 2),self.get(1, 3),self.get(2, 1),self.get(2, 2),self.get(2, 3),self.get(3, 1),self.get(3, 2),self.get(3, 3),]).det()
        -self.get(1, 0)*Matrix3::new([self.get(0, 1),self.get(0, 2),self.get(0, 3),self.get(2, 1),self.get(2, 2),self.get(2, 3),self.get(3, 1),self.get(3, 2),self.get(3, 3),]).det()
        +self.get(2, 0)*Matrix3::new([self.get(0, 1),self.get(0, 2),self.get(0, 3),self.get(1, 1),self.get(1, 2),self.get(1, 3),self.get(3, 1),self.get(3, 2),self.get(3, 3),]).det()
        -self.get(3, 0)*Matrix3::new([self.get(0, 1),self.get(0, 2),self.get(0, 3),self.get(1, 1),self.get(1, 2),self.get(1, 3),self.get(2, 1),self.get(2, 2),self.get(2, 3),]).det()
    }

    pub fn inv(&self)->Option<Self>{
        let det = self.det();
        if det<1e-6{
            return None
        }

        Some((1.0/det)*self.transpose() )
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
impl Mul<Matrix4> for f32{
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        let vec = rhs.elements.iter().map(|e|*e * self).collect::<Vec<f32>>();
        let slice = vec.as_slice();
        let array: [f32; 16] = match slice.try_into() {
            Ok(ba) => ba,
            Err(_) => panic!(""),
        };
        Matrix4::new( array)
    }
}

#[derive(Default,Clone,Copy,Debug)]
pub struct Quat{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Quat{
    pub fn new(x:f32,y:f32,z:f32,w:f32)->Self{
        Self { x, y, z, w }
    }
    pub fn inv(&self)->Self{
        Self::new(self.x, -self.y, -self.z, -self.w)
    }
    pub fn from_to_mat3(x_axis:Vec3,y_axis:Vec3,z_axis:Vec3)->Self{
        let tr=x_axis.x+y_axis.y+z_axis.z;
        let a = (tr+1.0).sqrt()*0.5;
        let w_inv=1.0/(4.0*a);
        let b =(y_axis.z-z_axis.y)*w_inv;
        let c=(z_axis.x-x_axis.z)*w_inv;
             let d=(x_axis.y-y_axis.x)*w_inv;
             Self::new(a, b, c, d)
    }
    pub fn from_axis_angle(axis:Vec3,radian:f32)->Self{
        let axis=axis.normalize();
        let sin=(0.5*radian).sin();
        let cos=(0.5*radian).cos();

        Quat::new(cos, sin*axis.x, sin*axis.y, sin*axis.z)
    }
    #[rustfmt::skip]
    pub fn get_rotation(&self)->Matrix4{
        let a = self.x;
        let b=self.y;
        let c =self.z;
        let d=self.w;

        Matrix4::new(
        [1.0-2.0*c*c-2.0*d*d, 2.0*b*c-2.0*a*d, 2.0*a*c+2.0*b*d, 0.0,
        2.0*b*c+2.0*a*d, 1.0-2.0*b*b-2.0*d*d, 2.0*c*d-2.0*a*b, 0.0,
        2.0*b*d-2.0*a*c, 2.0*a*b+2.0*c*d, 1.0-2.0*b*b-2.0*c*c, 0.0,
        0.0, 0.0, 0.0, 1.0])
    }
    
    pub fn get_z_axis(&self)->Vec3{
        let a = self.x;
        let b=self.y;
        let c =self.z;
        let d=self.w;

        Vec3::new(  2.0*a*c+2.0*b*d,  2.0*c*d-2.0*a*b, 1.0-2.0*b*b-2.0*c*c)
    }

    pub fn get_left_mat4(&self)->Matrix4{
        let a = self.x;
        let b=self.y;
        let c =self.z;
        let d=self.w;

        Matrix4 { elements: [a,-b,-c,-d,
            b,a,-d,c,
            c,d,a,-b,
            d,-c,b,a] }
    }
}

impl Mul<Quat> for Quat{
    type Output = Self;
    fn mul(self, rhs: Quat) -> Self::Output {
        //self * rhs = L(self)*rhs
      let vec:Vec4= rhs.into();
      (self.get_left_mat4()*vec).into()
    }  
}
impl From<Vec4> for Quat{
    fn from(value: Vec4) -> Self {
        Quat::new(value.x, value.y, value.z,value.w)
    }
}
impl From<Quat> for Vec4{
    fn from(value: Quat) -> Self {
        Vec4::new(value.x,value.y, value.z, value.w)
    }
}