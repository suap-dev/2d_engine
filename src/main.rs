#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

use std::{f32::consts::TAU, time::Instant};

use glium::glutin::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use nalgebra_glm::{vec2, RealNumber};

use engine::world;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = world::World::new(&event_loop);
    world.fill(32, 32, vec2(0.3, 0.5), TAU / 45.0);

    let mut mouse = Mouse::default();
    // DEBUG
    let mut now = Instant::now();
    let mut debug_iterations: usize = 0;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent { event, .. } => match event {
                // close window
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                // mouse events
                WindowEvent::CursorMoved { position, .. } => {
                    mouse.position = position;
                }
                WindowEvent::MouseInput { state, button, .. } => match button {
                    MouseButton::Left => match state {
                        ElementState::Pressed => {
                            mouse.left_button.pressed = true;
                        }
                        ElementState::Released => {
                            mouse.left_button.pressed = false;
                        }
                    },
                    MouseButton::Right => match state {
                        ElementState::Pressed => {
                            mouse.right_button.pressed = true;
                        }
                        ElementState::Released => {
                            mouse.right_button.pressed = false;
                        }
                    },
                    _ => {}
                },
                _ => {}
            },

            // game loop
            Event::MainEventsCleared => {
                debug_iterations += 1;
                let dt = now.elapsed();
                now = Instant::now();

                #[allow(clippy::cast_possible_truncation)]
                if mouse.left_button.pressed {
                    world.add_obj_at(
                        world.to_gl_coords(vec2(mouse.position.x as f32, mouse.position.y as f32)),
                    );
                }

                let update_instant = Instant::now();
                world.update(dt);
                let update_time = update_instant.elapsed();

                let render_instant = Instant::now();
                world.render();
                let render_time = render_instant.elapsed();

                #[allow(clippy::uninlined_format_args)]
                if debug_iterations % 4_000 == 0 {
                    println!("nr of objects: {:?}", world.entities_number());
                    println!("loop time: {:?}", dt);
                    println!("update time: {:?}", update_time);
                    println!("render time: {:?}", render_time);
                    println!();
                };
            }
            _ => {}
        }
    });
}

#[derive(Default)]
struct Mouse<T: RealNumber> {
    left_button: Button,
    right_button: Button,
    position: PhysicalPosition<T>,
}
#[derive(Default)]
struct Button {
    pressed: bool,
}
