#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

use engine::{Entity, World};
use glium::glutin::{
    dpi::PhysicalPosition,
    event::{self, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use nalgebra_glm::vec2;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = World::new(&event_loop);

    let triangle = Entity::polygon(
        vec![vec2(0.0, 0.5), vec2(-0.5, -0.5), vec2(0.5, -0.5)],
        [0.2, 0.4, 0.6, 1.0],
    );
    let circle = Entity::circle([0.4, 0.4].into(), 0.2, [0.8, 0.0, 0.3, 1.0]);
    let rectangle = Entity::rectangle([-0.4, 0.4].into(), 0.2, 0.3, [0.8, 0.4, 0.3, 1.0]);

    let circle = world.add(circle);
    let rectangle = world.add(rectangle);
    let triangle = world.add(triangle);


    let mut mouse_position = PhysicalPosition::new(-1.0, -1.0);
    event_loop.run(move |event, _, control_flow| {
        world.translate_citizen(rectangle, vec2(0.00006, 0.0));
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
                }
                _ => {}
            }
        }
    });
}
