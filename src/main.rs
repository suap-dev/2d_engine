#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

use std::time::Instant;

use engine::{shape::Shape, world};
use glium::glutin::{
    dpi::PhysicalPosition,
    event::{self, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use nalgebra_glm::vec2;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = world::World::new(&event_loop);

    let mut mouse_position = PhysicalPosition::new(-1.0, -1.0);
    let mut now = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let dt = now.elapsed();
        now = Instant::now();

        world.update(dt);
        world.render();

        if let event::Event::WindowEvent { event, .. } = event {
            match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                event::WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                } => {
                    mouse_position = position;
                }
                event::WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    modifiers: _,
                } => {
                    println!("Mouse:");
                    println!(" - button: {:?}", button);
                    println!(" - state: {:?}", state);
                    println!(" - position: {:?}", mouse_position);
                    println!();

                    if state == ElementState::Released {
                        world.add(
                            Shape::circle(0.01, [1.0, 0.0, 0.0, 1.1]),
                            world.to_gl_coords(vec2(
                                mouse_position.x as f32,
                                mouse_position.y as f32,
                            )),
                        );
                    }
                }
                _ => {}
            }
        }
    });
}
