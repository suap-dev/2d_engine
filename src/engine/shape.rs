
// 1) screen_coordinates: u32 -- simply current state of framebuffers coordinates system - u32
// 2) gl_coords: f32  and  gl_length: f32  -- actual screen coordinates and length according to OpenGL
// 3) logical_coordinates: f32/f64  and  logical_length: f32/f64 -- my internal system of coordinates according to world size (and unit?)
// 4) world_size: [u32; 2] -- logical size of the world
// 5) unit: f32/f64 -- do we even need this? it can be used to translate the screen coordinate to logical? nevermind.

use std::f32::consts::TAU;

use nalgebra_glm::{Vec2, mat2, vec2};

// shape will speak logical units
pub struct Shape {
    pub vertices: Vec<Vec2>,
    pub color: [f32; 4],
}
const VERTICES_OF_A_CIRCLE: u32 = 16;
impl Shape {
    pub fn polygon(vertices: Vec<Vec2>, color: [f32; 4]) -> Self {
        Self {
            vertices,
            color,
            // pivot_point: Vec2::new(0.0, 0.0),
            // rotation: 0.0,
        }
    }
    pub fn circle(radius: f32, color: [f32; 4]) -> Self {
        let interior_angle = TAU / VERTICES_OF_A_CIRCLE as f32;

        let mut vertices: Vec<Vec2> = Vec::new();

        let mut temp_vertex_position = vec2(0.0, radius);
        let rotation_matrix = mat2(
            interior_angle.cos(),
            -interior_angle.sin(),
            interior_angle.sin(),
            interior_angle.cos(),
        );

        vertices.push(temp_vertex_position); // 0-th vertex

        for _ in 1..VERTICES_OF_A_CIRCLE {
            temp_vertex_position = rotation_matrix * temp_vertex_position;
            vertices.push(temp_vertex_position);
        }

        Self {
            vertices,
            color,
        }
    }
    pub fn rectangle(width: f32, height: f32, color: [f32; 4]) -> Self {
        Self {
            vertices: vec![
                vec2(height / 2.0, -width / 2.0),
                vec2(-height / 2.0, -width / 2.0),
                vec2(-height / 2.0, width / 2.0),
                vec2(height / 2.0, width / 2.0),
            ],
            color,
        }
    }
}