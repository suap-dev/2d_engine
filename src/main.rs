#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

use engine::{Entity, World};
use glium::glutin::{
    event,
    event_loop::{ControlFlow, EventLoop},
};
use nalgebra_glm::{vec2, Mat2};
use std::f32::consts::TAU;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = World::new(&event_loop);

    let mut triangle = Entity::empty();
    triangle.add_vertex(vec2(0.0, 0.5));
    triangle.add_vertex(vec2(-0.5, -0.5));
    triangle.add_vertex(vec2(0.5, -0.5));

    let circle = Entity::circle([0.0, 0.0].into(), 0.3, [0.8, 0.0, 0.3, 1.0]);

    world.add(triangle);
    world.add(circle);

    event_loop.run(move |event, _, control_flow| {
        world.render();
        if let event::Event::WindowEvent { event, .. } = event {
            if let event::WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
