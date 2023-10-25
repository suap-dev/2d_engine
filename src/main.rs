#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

// mod engine;

// use std::{f32::consts::TAU, time::Instant};

// use glium::glutin::{
//     dpi::PhysicalPosition,
//     event::{self, ElementState},
//     event_loop::{ControlFlow, EventLoop},
// };
// use nalgebra_glm::vec2;

// use engine::world;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{self, EventLoop},
    keyboard::Key,
    window::WindowBuilder,
};

use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new().expect("Unable to create EventLoop");
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Unable to build Window");

    let input = WinitInputHelper::new();

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            eprintln!("The close button was pressed; stopping");
            elwt.exit();
        }

        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Character(character),
                            state,
                            ..
                        },
                    ..
                },
            ..
        } => match state {
            winit::event::ElementState::Pressed => match &*character {
                "A" | "a" => {
                    println!("Left.");
                }
                _ => {}
            },
            winit::event::ElementState::Released => todo!(),
        },

        Event::AboutToWait => {
            todo!();
        }

        _ => {}
    });
}
// fn main() {
//     let event_loop = EventLoop::new();
//     let mut world = world::World::new(&event_loop);
//     world.fill(50, 50, vec2(0.3, 0.5), TAU / 45.0);

//     let mut mouse_position = PhysicalPosition::new(-1.0, -1.0);
//     let mut now = Instant::now();
//     let mut debug_iterations: usize = 0;
//     event_loop.run(move |event, _, control_flow| {
//         control_flow.set_poll();
//         match event {
//             event::Event::WindowEvent { window_id, event } => {
//                 match event {
//                     event::WindowEvent::CloseRequested => {
//                         *control_flow = ControlFlow::Exit;
//                     }
//                     #[allow(deprecated)]
//                     event::WindowEvent::CursorMoved {
//                         device_id: _,
//                         position,
//                         modifiers: _,
//                     } => {
//                         mouse_position = position;
//                     }
//                     #[allow(deprecated, unused_variables)]
//                     event::WindowEvent::MouseInput {
//                         device_id: _,
//                         state,
//                         button,
//                         modifiers: _,
//                     } => {
//                         if state == ElementState::Released {
//                             world.add_obj_at(world.to_gl_coords(vec2(
//                                 mouse_position.x as f32,/
//                                 mouse_position.y as f32,
//                             )));
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//             event::Event::MainEventsCleared => {
//                 debug_iterations += 1;
//                 let dt = now.elapsed();
//                 now = Instant::now();

//                 let update_instant = Instant::now();
//                 world.update(dt);
//                 let update_time = update_instant.elapsed();

//                 let render_instant = Instant::now();
//                 world.render();
//                 let render_time = render_instant.elapsed();

//                 #[allow(clippy::uninlined_format_args)]
//                 if debug_iterations % 1_000 == 0 {
//                     println!("nr of objects: {:?}", world.entities_number());
//                     println!("loop time: {:?}", dt);
//                     println!("update time: {:?}", update_time);
//                     println!("render time: {:?}", render_time);
//                     println!();
//                 };
//             }
//             _ => {}
//         }
//     });
// }
