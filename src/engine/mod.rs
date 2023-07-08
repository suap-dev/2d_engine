pub mod world;

use std::{f32::consts::TAU, collections::HashMap};

use glium::{
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    implement_vertex, uniform, Display, DrawParameters, Program, Surface, VertexBuffer,
};
use nalgebra_glm::{Vec2, vec2, mat2};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
impl From<Vec2> for Vertex {
    fn from(value: Vec2) -> Self {
        Self {
            position: value.into(),
        }
    }
}
implement_vertex!(Vertex, position);//, color);

pub struct Entity {
    base_shape: Vec<Vec2>,
    color: [f32; 4],
}
impl Entity {
    pub fn circle(origin: Vec2, radius: f32, color: [f32; 4]) -> Self{        
        const VERTICES: usize = 32;
        let angle = TAU / VERTICES as f32;

        let mut base_shape: Vec<Vec2> = Vec::new();

        let mut temp_vertex_position = vec2(0.0, radius);
        let rotation_matrix = mat2(angle.cos(), -angle.sin(), angle.sin(), angle.cos());

        base_shape.push(temp_vertex_position + origin);  // 0-th vertex

        for _ in 1..VERTICES {
            temp_vertex_position = rotation_matrix * temp_vertex_position;
            base_shape.push(temp_vertex_position + origin);
        }

        Self {
            base_shape,
            color,
        }
    }

    pub fn polygon(vertices: Vec<Vec2>, color: [f32;4]) -> Self {
        Self {
            base_shape: vertices,
            color
        }
    }

    pub fn rectangle(origin: Vec2, width: f32, height: f32, color: [f32; 4]) -> Self {
        Self{
            base_shape: vec![
                vec2(origin.x - width/2.0, origin.y + height/2.0),
                vec2(origin.x - width/2.0, origin.y - height/2.0),
                vec2(origin.x + width/2.0, origin.y - height/2.0),
                vec2(origin.x + width/2.0, origin.y + height/2.0),
            ],
            color,
        }
    }

    pub fn translate(&mut self, vector: Vec2) {
        for vertex_position in &mut self.base_shape {
            *vertex_position += vector;
        }
    }
}

