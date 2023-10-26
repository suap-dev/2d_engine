#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;
mod verlet;

use std::{
    f32::consts::TAU,
    time::{Duration, Instant},
};

use glium::glutin::event_loop::{ControlFlow, EventLoop};
use nalgebra_glm::vec2;
use winit_input_helper::WinitInputHelper;

use engine::world;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = world::World::new(&event_loop);
    world.populate(32, 32, vec2(0.3, 0.5), TAU / 45.0);

    let mut input = WinitInputHelper::new();
    let mut timer = Timer::new();

    // DEBUG
    let mut debug_iterations: usize = 0;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        #[allow(clippy::collapsible_if)]
        if input.update(&event) {
            // debug_iterations += 1;

            if input.quit() {
                *control_flow = ControlFlow::Exit;
            }
            if input.mouse_held(0) {
                if let Some((x, y)) = input.mouse() {
                    world.add_obj_at(world.to_gl_coords(vec2(x, y)));
                }
            }
            
            world.update_positions(timer.dt());
            world.solve_collisions();
            world.update_vertex_buffer();
            world.render();

            // #[allow(clippy::uninlined_format_args)]
            // if debug_iterations % 2_000 == 0 {
            //     println!("nr of objects: {:?}", world.entities_number());
            //     println!("loop time: {:?}", dt);
            //     println!("collisions update time: {:?}", update_time);
            //     println!("vertex buffer update time: {:?}", update_buffer_time);
            //     println!("render time: {:?}", render_time);
            //     println!();
            // };
        }
    });
}

struct Timer {
    last_instant: Instant,
}
impl Timer {
    fn new() -> Self {
        Self {
            last_instant: Instant::now(),
        }
    }
    fn dt(&mut self) -> Duration {
        let dt = self.last_instant.elapsed();
        self.last_instant = Instant::now();
        dt
    }
}
