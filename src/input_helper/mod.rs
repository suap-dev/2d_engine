pub mod mouse;

use glium::glutin::event::{Event, WindowEvent};
pub use mouse::{Mouse, MouseButton};

#[derive(Default)]
pub struct InputHelper {
    pub mouse: Mouse,
}
impl InputHelper {
    /// /// from winit_input_helper crate /// ///
    /// Pass every winit event to this function and run your application logic when it returns true.
    ///
    /// The following winit events are handled:
    /// *   `Event::NewEvents` clears all internal state.
    /// *   `Event::MainEventsCleared` causes this function to return true, signifying a "step" has completed.
    /// *   `Event::WindowEvent` updates internal state, this will affect the result of accessor methods immediately.
    pub fn update<T>(&mut self, event: &Event<T>) -> bool {
        match &event {
            // Event::NewEvents(_) => {
            //     self.step();
            //     false
            // }
            Event::WindowEvent { event, .. } => {
                self.process_window_event(event);
                false
            }
            Event::MainEventsCleared => true,
            _ => false,
        }
    }

    fn process_window_event(&mut self, event: &WindowEvent) {
        
    }
}

#[derive(Default)]
struct Button {
    pressed: bool,
}
