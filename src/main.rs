#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

use engine::world;
use glium::glutin::{
    dpi::PhysicalPosition,
    event::{self, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use nalgebra_glm::vec2;
use std::{f32::consts::TAU, time::Instant};

fn main() {
    let event_loop = EventLoop::new();
    let mut world = world::World::new(&event_loop);
    world.fill(32, 32, vec2(0.3, 0.5), TAU / 45.0);

    let mut mouse_position = PhysicalPosition::new(-1.0, -1.0);
    let mut now = Instant::now();
    let mut debug_iterations: usize = 0;
    event_loop.run(move |event, _, control_flow| {
        debug_iterations += 1;
        let dt = now.elapsed();
        now = Instant::now();

        let update_instant = Instant::now();
        world.update(dt);
        let update_time = update_instant.elapsed();

        let render_instant = Instant::now();
        world.render();
        let render_time = render_instant.elapsed();

        #[allow(clippy::uninlined_format_args)]
        if debug_iterations % 4_000 == 0 {
            println!("nr of objects: {:?}", world.citizens_number());
            println!("loop time: {:?}", dt);
            println!("update time: {:?}", update_time);
            println!("render time: {:?}", render_time);
            println!();
        };

        if let event::Event::WindowEvent { event, .. } = event {
            match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                #[allow(deprecated)]
                event::WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                } => {
                    mouse_position = position;
                }
                #[allow(deprecated, unused_variables)]
                event::WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    modifiers: _,
                } => {
                    if state == ElementState::Released {
                        world.add_obj_at(
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
