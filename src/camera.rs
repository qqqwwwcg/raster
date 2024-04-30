use crate::*;

#[derive(Clone)]
pub struct Camera {
    frustum: Frustum,
    position: Vec3, //eye
    rotation: Quat,
}
impl Camera {
    pub fn new(frustum: Frustum, position: Vec3, target: Vec3) -> Self {
        Self {
            frustum,
            position,
            rotation: look_at(position, target, Vec3::Y),
        }
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    pub fn get_dir(&self) -> Vec3 {
        -self.rotation.get_z_axis()
    }

    pub fn rotation_around(&mut self, target: Vec3, rotation: Quat) {
        let diff = self.position - target;
        let m_rot = rotation.get_rotation();
        self.position = Into::<Vec3>::into(m_rot.mul(diff.upgrade())) + target;
        self.rotation = rotation * self.rotation;
    }
    pub fn scale(&mut self, offset: f32) {
        let fov = self.frustum.fov + offset;
        if fov < 180.0f32.to_radians() && fov > 0.0 {
            self.frustum.fov = fov
        }
    }

    pub fn get_view_matrix(&mut self) -> Matrix4 {
        let mat_rot_inv = self.rotation.inv().get_rotation();

        let mut mat_trans_inv = Matrix4::ident();
        mat_trans_inv.set(0, 3, -self.position.x);
        mat_trans_inv.set(1, 3, -self.position.y);
        mat_trans_inv.set(2, 3, -self.position.z);

        //(TR).inv=R_inv*T_inv
        mat_rot_inv * mat_trans_inv
    }
    pub fn get_projection_matrix(&self) -> Matrix4 {
        self.frustum.perspective_projection()
    }
}

pub fn look_at(eye: Vec3, target: Vec3, world_up: Vec3) -> Quat {
    //右手坐标系 朝向-z
    let dir = (target - eye).normalize();
    let z_axis = -dir;
    let x_axis = world_up.cross(&z_axis).normalize();
    let y_axis = z_axis.cross(&x_axis).normalize();

    Quat::from_to_mat3(x_axis, y_axis, z_axis)
}

#[derive(Default, Clone)]
pub struct Frustum {
    near: f32, //near plane: z= -near in view ---> plane z = 1.0 in ndc
    far: f32,  //far plane: z= -far   in view ---> plane z = -1.0 in ndc
    fov: f32,
    aspect: f32, //width/height
}
impl Frustum {
    pub fn new(near: f32, far: f32, fov: f32, aspect: f32) -> Self {
        Self {
            near,
            far,
            fov,
            aspect,
        }
    }
    pub fn orth_projection(&self) -> Matrix4 {
        let h = 2.0 * self.near * (self.fov * 0.5).tan();
        let w = h * self.aspect;

        let near_z = -self.near;
        let far_z = -self.far;
        #[rustfmt::skip]
        let translate = Matrix4::new([
            1.0,    0.0,    0.0,    0.0,
            0.0,    1.0,    0.0,    0.0,
            0.0,    0.0,    1.0,    -0.5*(near_z+far_z),
            0.0,    0.0,    0.0,    1.0]
        );
        #[rustfmt::skip]
        let scale = Matrix4::new([
            2.0 / w,    0.0,    0.0,    0.0,
            0.0,    2.0 / h,    0.0,    0.0,
            0.0,    0.0,    2.0 / (near_z-far_z), 0.0,
            0.0,    0.0,    0.0,    1.0,]
        );

        scale * translate
    }

    pub fn perspective_projection(&self) -> Matrix4 {
        let near_z = -self.near;
        let far_z: f32 = -self.far;

        #[rustfmt::skip]
        let per_to_orth = Matrix4::new([
           near_z,    0.0,    0.0,    0.0,
            0.0,   near_z,    0.0,    0.0,
            0.0,    0.0,   near_z + far_z, -near_z*far_z,
            0.0,    0.0,    1.0,    0.0,]
        );

        self.orth_projection() * per_to_orth
    }
}
