use crate::*;

use obj::{load_obj, Obj, TexturedVertex};
use std::fs::File;
use std::io::BufReader;

pub struct Mesh {
    pub vertexes: Vec<Vertex>,
    pub indies: Vec<usize>,
    pub material: Material,
}

impl Mesh {
    pub fn new(vertexes: Vec<Vertex>, indies: Vec<usize>, material: Material) -> Self {
        Self {
            vertexes,
            indies,
            material,
        }
    }
    pub fn get_bounding(&self) -> (Vec3, Vec3) {
        let mut min = self.vertexes[0].position;
        let mut max = self.vertexes[0].position;
        self.vertexes.iter().for_each(|vertex| {
            min = vertex.position.min(&min);
            max = vertex.position.max(&max);
        });

        (min, max)
    }
}

pub fn load_model(path: &str) -> (Vec<Vertex>, Vec<usize>) {
    let input = BufReader::new(File::open(path).unwrap());
    let model: Obj<TexturedVertex> = load_obj(input).unwrap();

    let vertexes = model
        .vertices
        .iter()
        .map(|vertex| {
            Vertex::new(
                vertex.position.into(),
                vertex.normal.into(),
                Vec3::from(vertex.texture).downgrade(),
            )
        })
        .collect();

    let indies = model.indices.iter().map(|index| *index as usize).collect();
    (vertexes, indies)
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
}
impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texcoord: Vec2) -> Self {
        Self {
            position,
            normal,
            texcoord,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Uniform {
    world_position: Vec3,
    world_normal: Vec3,
    pub position: Vec3, //local position => clipping position => ndc position => screen(x,y) + ndc.z
    texcoord: Vec2,
}
impl Uniform {
    pub fn new(world_position: Vec3, world_normal: Vec3, position: Vec3, texcoord: Vec2) -> Self {
        Self {
            world_position,
            world_normal,
            position,
            texcoord,
        }
    }
}
#[derive(Debug)]
pub struct Triangle {
    pub a: Uniform,
    pub b: Uniform,
    pub c: Uniform,
}
impl Triangle {
    pub fn new(a: Uniform, b: Uniform, c: Uniform) -> Self {
        Self { a, b, c }
    }

    pub fn get_world_normal(&self) -> Vec3 {
        (self.b.world_position - self.a.world_position)
            .cross(&(self.c.world_position - self.a.world_position))
            .normalize()
    }

    pub fn ndc_culling_test(&self) -> bool {
        let ndc_culling = |position: &Vec3| {
            position.x.abs() > 1.0 || position.y.abs() > 1.0 || position.z.abs() > 1.0
        };

        ndc_culling(&self.a.position)
            || ndc_culling(&self.b.position)
            || ndc_culling(&self.c.position)
    }

    pub fn create_frame(&self, barycentric_coordinates: Vec3, screen_pos: (u32, u32)) -> Fragment {
        let alpha = barycentric_coordinates.x;
        let beta = barycentric_coordinates.y;
        let gamma = barycentric_coordinates.z;

        let world_position = alpha * self.a.world_position
            + beta * self.b.world_position
            + gamma * self.c.world_position;
        let world_normal =
            alpha * self.a.world_normal + beta * self.b.world_normal + gamma * self.c.world_normal;
        let texcoord = alpha * self.a.texcoord + beta * self.b.texcoord + gamma * self.c.texcoord;
        let depth =
            alpha * self.a.position.z + beta * self.b.position.z + gamma * self.c.position.z;

        Fragment::new(world_position, world_normal, texcoord, screen_pos, depth)
    }
}

pub struct Fragment {
    pub world_position: Vec3,
    pub world_normal: Vec3,
    pub texcoord: Vec2,
    pub screen_pos: (u32, u32),
    pub depth: f32,
}

impl Fragment {
    pub fn new(
        world_position: Vec3,
        world_normal: Vec3,
        texcoord: Vec2,
        screen_pos: (u32, u32),
        depth: f32,
    ) -> Self {
        Self {
            world_position,
            world_normal: world_normal.normalize(),
            texcoord,
            screen_pos,
            depth,
        }
    }
}
