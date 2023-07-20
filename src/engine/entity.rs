use nalgebra_glm::Vec2;

pub struct Entity {
    pub position: Vec2,
    pub previous_position: Vec2,
    pub radius: f32,
    pub acceleration: Vec2,
    pub color: [f32; 4],
}

impl Entity {
    pub fn new(position: Vec2, radius: f32, color: [f32; 4]) -> Self {
        Self {
            position,
            radius,
            previous_position: position,
            acceleration: Vec2::zeros(),
            color,
        }
    }
    pub fn collides_with(&self, other: &Self) -> bool {
        self.position.metric_distance(&other.position) < self.radius + other.radius
    }
}
