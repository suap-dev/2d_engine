#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod engine;

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
            if input.quit() {
                *control_flow = ControlFlow::Exit;
            }
            if input.mouse_held(0) {
                if let Some((x, y)) = input.mouse() {
                    world.add_obj_at(world.to_gl_coords(vec2(x, y)));
                }
            }

            debug_iterations += 1;

            let update_instant = Instant::now();
            let dt = timer.dt();
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
