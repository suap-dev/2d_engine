use std::collections::HashMap;

use glium::{VertexBuffer, Display, Program, glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder, dpi::PhysicalSize}, Surface, uniform, DrawParameters};
use nalgebra_glm::Vec2;

use super::{Entity, Vertex};

// TODO: are separate structs Citizen and Entity really needed? Figure out and posibly make 1 struct for this functionality
// TODO: do I really want to keep the whole Entity in the World? do I need to remember it?
struct Citizen {
    entity: Entity,
    vertex_buffer: VertexBuffer<Vertex>,
}

#[derive(Clone, Copy)]
pub struct CitizenId (usize);

const WORLD_DIMENSIONS: [u32;2] = [1600, 900];
const ASPECT_RATIO: f32 = WORLD_DIMENSIONS[0] as f32 / WORLD_DIMENSIONS[1] as f32;
pub struct World {
    pub display: Display,
    citizens: HashMap<usize, Citizen>,
    sky_color: [f32; 4],
    program: Program,
    hash: usize,
    framebufer_width: u32,
    framebuffer_height: u32,
    size_stabilised: bool,  // TODO: I think this is a very ugly hotfix (for handle_resize() fnction)
}
impl World {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new().with_inner_size(PhysicalSize::new(WORLD_DIMENSIONS[0], WORLD_DIMENSIONS[1]));
        let context_builder = ContextBuilder::new();        
        let display = Display::new(window_builder, context_builder, event_loop)
            .expect("Unable to initialise display.");

        let program = Program::from_source(
            &display,
            r#"
            #version 150

            in vec2 position;

            uniform vec2 u_window_dimensions;

            void main() {
                // v_color = color;
                float smaller = u_window_dimensions.x > u_window_dimensions.y ? u_window_dimensions.y : u_window_dimensions.x;
                vec2 transformed_position = (position/u_window_dimensions)*smaller;
                gl_Position = vec4(transformed_position, 0.0, 1.0);
            }
        "#,
            r#"
            #version 150

            uniform vec4 u_color;

            out vec4 color;

            void main() {
                // color = vec4(1.0, 1.0, 1.0, 1.0);
                color = u_color;
            }
        "#,
            None,
            
        )
        .expect("Program creation error.");
        Self {
            display,
            program, 
            sky_color: [0.0, 0.0, 0.0, 1.0],
            citizens: HashMap::new(),
            hash: 0,
            framebufer_width: WORLD_DIMENSIONS[0],
            framebuffer_height: WORLD_DIMENSIONS[1],
            size_stabilised: false
        }
    }
    
    // TODO: resizing is really a work in progress...
    pub fn handle_resize(&mut self, physical_size: PhysicalSize<u32>) {
        // guard/hotfix
        if !self.size_stabilised { 
            let physical_size: [u32;2] = physical_size.into();
            if physical_size == WORLD_DIMENSIONS {
                self.size_stabilised = true;
            }
            return
        }

        let (current_width, current_height) = (self.framebufer_width, self.framebuffer_height);
        let (mut new_width, mut new_height) = {
            let new_size: [u32;2] = physical_size.into();
            (new_size[0], new_size[1])
        };

        let delta_width = new_width.abs_diff(current_width);
        let delta_height = new_height.abs_diff(current_height);
        if delta_height > delta_width {            
            new_width = (new_height as f32 * ASPECT_RATIO) as u32;
        }
        else {
            new_height = (new_width as f32 / ASPECT_RATIO) as u32;
        }

        (self.framebufer_width, self.framebuffer_height) = (new_width, new_height);

        self.display.gl_window().window().set_inner_size(PhysicalSize::new(self.framebufer_width, self.framebuffer_height));

    }
    
    pub fn render(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(
            self.sky_color[0],
            self.sky_color[1],
            self.sky_color[2],
            self.sky_color[3],
        );
        let (framebuffer_x,framebuffer_y) = &self.display.get_framebuffer_dimensions();
        for citizen in &self.citizens {
            frame
                .draw(
                    &citizen.1.vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                    &self.program,
                    &uniform! {
                        u_window_dimensions: [*framebuffer_x as f32, *framebuffer_y as f32],
                        u_color: citizen.1.entity.color,
                    },
                    &DrawParameters::default(),
                )
                .expect("Unable to draw this entity.");
        }
        frame.finish().expect("Unable to finish drawing a frame.");
    }
    pub fn add(&mut self, e: Entity) -> CitizenId {
        self.hash += 1;
        self.citizens.insert(self.hash, Citizen {
            vertex_buffer: self.vertex_buffer(&e),
            entity: e,
        });
        CitizenId(self.hash)
    }
    fn vertex_buffer(&self, entity: &Entity) -> VertexBuffer<Vertex> {
        let mut data = Vec::new();
        for vertex in &entity.base_shape {
            data.push(Vertex {
                position: (*vertex).into(),
            });
        }
        VertexBuffer::new(
            &self.display,
            &data,
        )
        .expect("VertexBuffer creation failed.")
    }
    // TODO: for optimisation purposes this could be done on CPU or GPU depending on whether Citizen taking part in pysics simulation or not
    pub fn translate_citizen(&mut self, id: CitizenId, vector: Vec2) {
        let id = id.0;
        let citizen = self.citizens.get_mut(&id);
        if let Some(citizen) = citizen {            
            citizen.entity.translate(vector);
            let vecnew: Vec<Vertex> = citizen.entity.base_shape.iter().map(|t| (*t).into()).collect();
            citizen.vertex_buffer.write(&vecnew);
        }
    }
}