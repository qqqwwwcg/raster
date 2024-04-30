use crate::{BaseColor, NormalMap, SpecularMap, Vec3};

pub struct PointLight {
    pub position: Vec3,
    pub intensity: f32,
}
impl PointLight {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

pub struct PhongMaterial {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub specular_shininess: f32,
}
impl PhongMaterial {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, specular_shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            specular_shininess,
        }
    }
}
impl Default for PhongMaterial {
    fn default() -> Self {
        PhongMaterial::new(
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.64, 0.64, 0.64),
            Vec3::new(0.5, 0.5, 0.5),
            64.0,
        )
    }
}

pub struct Material {
    // light: PointLight,
    pub material: PhongMaterial,
    pub base: BaseColor,
    pub normal: Option<NormalMap>,
    pub specular: Option<SpecularMap>,
}
impl Material {
    pub fn new(
        material: PhongMaterial,
        base: BaseColor,
        normal: Option<NormalMap>,
        specular: Option<SpecularMap>,
    ) -> Self {
        Self {
            material,
            base,
            normal,
            specular,
        }
    }
}
