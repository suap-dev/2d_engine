mod shaders;
pub mod shape;
mod vertex;

use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    index::PrimitiveType,
    uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};

use self::{
    shape::{Shape, RADIUS},
    vertex::Vertex,
};

use super::verlet_object::VerletObject;

pub struct Renderer {
    pub display: Display,
    background_color: [f32; 4],
    pub default_shape: Shape,
    program: Program,
    vertex_buffer: Option<VertexBuffer<Vertex>>,
    index_buffer: Option<IndexBuffer<u16>>,
}
impl Renderer {
    pub fn new<T>(event_loop: &EventLoop<T>, physical_size: [u32; 2]) -> Self {
        let window_builder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(physical_size[0], physical_size[1]))
            .with_resizable(false);
        let context_builder = ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, event_loop)
            .expect("Unable to initialise display.");

        let program = Program::from_source(&display, shaders::VERTEX, shaders::FRAGMENT, None)
            .expect("Program creation error.");
        Self {
            display,
            background_color: [0.0, 0.0, 0.0, 1.0],
            default_shape: Shape::circle(RADIUS, [1.0, 1.0, 1.0, 1.0]),
            program,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(
            self.background_color[0],
            self.background_color[1],
            self.background_color[2],
            self.background_color[3],
        );
        if let Some(vb) = &self.vertex_buffer {
            if let Some(ib) = &self.index_buffer {
                frame
                    .draw(
                        vb,
                        ib,
                        &self.program,
                        &uniform! {
                            u_color: [1.0f32, 0.0, 1.0, 1.0],
                        },
                        &DrawParameters::default(),
                    )
                    .expect("Unable to draw this obj.");
            };
        }
        frame.finish().expect("Unable to finish drawing a frame.");
    }

    pub fn rewrite_vertex_buffer(&mut self, objects: &Vec<VerletObject>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for obj in objects {
            for vertex_position in &self.default_shape.vertices {
                let translated_vertex_position = vertex_position + obj.get_center();
                vertices.push(Vertex {
                    position: translated_vertex_position.into(),
                    color: obj.get_color(),
                });
            }
        }
        self.vertex_buffer = Some(
            VertexBuffer::new(&self.display, &vertices)
                .expect("Function rewrite_vertex_buffer() failed to create buffer."),
        );
    }

    pub fn rewrite_index_buffer(&mut self, objects: &Vec<VerletObject>) {
        let mut indices: Vec<u16> = Vec::new();
        let entities = objects.len();
        for obj_nr in 0..entities {
            #[allow(clippy::cast_possible_truncation)]
            indices.extend_from_slice(
                &shape::CIRCLE_INDICES
                    .as_slice()
                    .iter()
                    .map(|v_idx| v_idx + obj_nr as u16 * shape::VERTICES_OF_A_CIRCLE)
                    .collect::<Vec<u16>>(),
            );
        }
        self.index_buffer = Some(
            IndexBuffer::new(
                &self.display,
                PrimitiveType::TrianglesList,
                indices.as_slice(),
            )
            .expect("Function rewrite_index_buffer() failed to create buffer."),
        );
    }

    pub fn update_vertex_buffer(&mut self, objects: &Vec<VerletObject>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        for obj in objects {
            for vertex_position in &self.default_shape.vertices {
                let translated_vertex_position = vertex_position + obj.get_center();
                vertices.push(Vertex {
                    position: translated_vertex_position.into(),
                    color: obj.get_color(),
                });
            }
        }
        if let Some(vertex_buffer) = &self.vertex_buffer {
            vertex_buffer.write(&vertices);
        }
    }

    // I'm not sure if this is going to be useful in any forseeable future
    pub fn update_index_buffer(&mut self, objects: &Vec<VerletObject>) {
        let mut indices: Vec<u16> = Vec::new();
        let entities = objects.len();
        for obj_nr in 0..entities {
            for index in shape::CIRCLE_INDICES {
                #[allow(clippy::cast_possible_truncation)]
                indices.push(index + obj_nr as u16 * shape::VERTICES_OF_A_CIRCLE);
            }
        }

        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.write(indices.as_slice());
        }
    }
}
