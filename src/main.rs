#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod bench;
mod engine2;

use std::{f32::consts::TAU, time::Instant};

use glium::glutin::event_loop::{ControlFlow, EventLoop};
use nalgebra_glm::vec2;
use winit_input_helper::WinitInputHelper;

use bench::Bench;
use engine2::world;

pub const RADIUS: f32 = 0.01;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = world::World::new(&event_loop);
    world.populate(
        40,
        40,
        vec2(0.0, 0.0),
        TAU / 45.0,
        RADIUS,
        RADIUS / 3.0,
        0,
        RADIUS / 5.0,
    );

    let mut input = WinitInputHelper::new();
    let mut timer = Timer::new();

    // TODO: Add this functionality to Timer:
    let mut mouse_timer: Instant = Instant::now();
    let mut mouse_tick_delta = 0.0;
    let mouse_tick_every: f32 = 0.1; //seconds

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
            if let Some((x, y)) = input.mouse() {
                if input.mouse_pressed(0) {
                    mouse_timer = Instant::now();
                    world.add_obj_at(world.to_gl_coords(vec2(x, y)), RADIUS);
                    println!("now");
                }
                if input.mouse_held(0) {
                    mouse_tick_delta += mouse_timer.elapsed().as_secs_f32();
                    mouse_timer = Instant::now();
                    while mouse_tick_delta > mouse_tick_every {
                        world.add_obj_at(world.to_gl_coords(vec2(x, y)), RADIUS);
                        mouse_tick_delta -= mouse_tick_every;
                    }
                }
            }
            bench.events_cleared();

            // world.update_positions(timer.dt32());
            // bench.positions_updated();

            // // world.solve_collisions_with_grid();
            // world.solve_collisions();
            // bench.collisions_solved();

            world.update(timer.dt32(), 2);
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
