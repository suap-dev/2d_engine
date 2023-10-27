#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod bench;
mod engine;
mod verlet;

use std::{
    f32::consts::TAU,
    time::{Duration, Instant},
};

use glium::glutin::event_loop::{ControlFlow, EventLoop};
use nalgebra_glm::vec2;
use winit_input_helper::WinitInputHelper;

use bench::Bench;
use engine::world;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = world::World::new(&event_loop);
    world.populate(50, 40, vec2(0.0, 0.0), TAU / 45.0);

    let mut input = WinitInputHelper::new();
    let mut timer = Timer::new();

    // BENCHING
    let mut bench = Bench::init(4000);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        bench.loop_started();

        #[allow(clippy::collapsible_if)]
        if input.update(&event) {
            if input.quit() {
                *control_flow = ControlFlow::Exit;
            }
            if input.mouse_held(0) {
                if let Some((x, y)) = input.mouse() {
                    world.add_obj_at(world.to_gl_coords(vec2(x, y)));
                }
            }
            bench.events_cleared();

            world.update_positions(timer.dt32());
            bench.positions_updated();

            world.solve_collisions();
            bench.collisions_solved();

            world.update_vertex_buffer();
            bench.vb_updated();

            world.render();
            bench.rendering_finished();
        }

        bench.loop_ended();
        if bench.report() {
            bench.reset();
            println!(
                "Number of objects in simulation: {}",
                world.entities_number()
            );
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
    fn dt32(&mut self) -> f32 {
        let dt = self.last_instant.elapsed();
        self.last_instant = Instant::now();
        dt.as_secs_f32()
    }
}
