use crate::*;

use obj::{load_obj, Obj, TexturedVertex};
use std::fs::File;
use std::io::BufReader;

pub struct Mesh {
    pub vertexes: Vec<Vertex>,
    pub indies: Vec<usize>,
    offset: usize,
    //pub topology: TopologyList,
    //pub material: Material,
}

impl Mesh {
    pub fn load(path: &str) -> Self {
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
        Self {
            vertexes,
            indies,
            offset: 0,
        }
    }

    pub fn reset_offset(&mut self) {
        self.offset = 0;
    }
}
impl Iterator for Mesh {
    type Item = [Vertex; 3];
    fn next(&mut self) -> Option<Self::Item> {
        if self.indies.len() > self.offset + 3 {
            let triangle = Some([
                self.vertexes[self.indies[self.offset]],
                self.vertexes[self.indies[self.offset + 1]],
                self.vertexes[self.indies[self.offset + 2]],
            ]);
            self.offset += 3;
            triangle
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
    //pub color: Vec3,
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

// pub enum TopologyList {
//     Triangle,
//     Line,
//     Point,
// }
