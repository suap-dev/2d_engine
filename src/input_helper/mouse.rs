use glium::glutin::dpi::PhysicalPosition;
use nalgebra_glm::RealNumber;

use super::Button;

#[derive(Default)]
pub struct Mouse {
    left_button: Button,
    right_button: Button,
    physical_position: PhysicalPosition<f64>,
}
impl Mouse {
    pub const fn is_pressed(&self, mouse_button: MouseButton) -> bool {
        match mouse_button {
            MouseButton::Right => self.right_button.pressed,
            MouseButton::Left => self.left_button.pressed,
        }
    }
    pub const fn physical_position(&self) -> PhysicalPosition<f64> {
        return self.physical_position;
    }
}
pub enum MouseButton {
    Left,
    Right,
}
