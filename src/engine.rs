use glium::{
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    implement_vertex, uniform, Display, DrawParameters, Program, Surface, VertexBuffer,
};
use nalgebra_glm::Vec2;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
impl From<Vec2> for Vertex {
    fn from(value: Vec2) -> Self {
        Self {
            position: value.into(),
            color: [1.0, 1.0, 1.0, 1.0], // default color is white
        }
    }
}
implement_vertex!(Vertex, position, color);

// TODO: are separate structs Citizen and Entity really needed? Figure out and posibly make 1 struct for this functionality
struct Citizen {
    entity: Entity,
    vertex_buffer: VertexBuffer<Vertex>,
}

pub struct World {
    display: Display,
    citizens: Vec<Citizen>,
    sky_color: [f32; 4],
    program: Program,
}
impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new();
        let context_builder = ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, event_loop)
            .expect("Unable to initialise display.");
        println!("{:?}", display.get_framebuffer_dimensions());
        let program = Program::from_source(
            &display,
            r#"
            #version 150

            in vec2 position;
            in vec4 color;

            out vec4 v_color;

            uniform vec2 u_window_dimensions;

            void main() {
                v_color = color;
                float smaller = u_window_dimensions.x > u_window_dimensions.y ? u_window_dimensions.y : u_window_dimensions.x;
                vec2 transformed_position = (position/u_window_dimensions)*smaller;
                gl_Position = vec4(transformed_position, 0.0, 1.0);
            }
        "#,
            r#"
            #version 150

            in vec4 v_color;
            out vec4 color;

            void main() {
                color = v_color;
            }
        "#,
            None,
            
        )
        .expect("Program creation error.");
        Self {
            display,
            program, 
            sky_color: [0.0, 0.0, 0.0, 1.0],
            citizens: Vec::new(),
        }
    }
    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(
            self.sky_color[0],
            self.sky_color[1],
            self.sky_color[2],
            self.sky_color[3],
        );
        let dimensions = &self.display.get_framebuffer_dimensions();
        for citizen in &self.citizens {
            frame
                .draw(
                    &citizen.vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                    &self.program,
                    &uniform! {
                        u_window_dimensions: [dimensions.0 as f32, dimensions.1 as f32],
                    },
                    &DrawParameters::default(),
                )
                .expect("Unable to draw this entity.");
        }
        // frame.draw(_, _, program, uniforms, draw_parameters)
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    pub fn add(&mut self, e: Entity) {
        self.citizens.push(Citizen {
            // vertex_buffer: e.vertex_buffer(&self.display),
            vertex_buffer: self.vertex_buffer(&e),
            entity: e,
        });
    }    

    fn vertex_buffer(&self, entity: &Entity) -> VertexBuffer<Vertex> {
        // let thingy: Vec<Vertex> = self.vertices.iter().map(|v| (*v).into()).collect();
        let mut data = Vec::new();
        for vertex in entity.base_shape {
            data.push(Vertex {
                position: vertex.into(),
                color: entity.color,
            });
        }
        VertexBuffer::new(
            &self.display,
            // &self
            //     .vertices
            //     .iter()
            //     .map(|v| (*v).into())
            //     .collect::<Vec<Vertex>>(),
            &data,
        )
        .expect("VertexBuffer creation failed.")
    }
}

pub struct Entity {
    base_shape: Vec<Vec2>,
    color: [f32; 4],
}
impl Entity {
    pub const fn empty() -> Self {
        Self {
            base_shape: Vec::new(),
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub const fn empty_with_color(color: [f32; 4]) -> Self {
        Self {
            base_shape: Vec::new(),
            color,
        }
    }

    pub fn add_vertex(&mut self, v: Vec2) {
        self.base_shape.push(v);
    }
}

