use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

impl Velocity {
    fn increase(&mut self, direction: i32) {
        let max_speed = 250.0;
        let speed_increase = 10.0;
        let new_velocity = self.0 + speed_increase * Vec3::new(direction as f32, 0.0, 0.0);
        self.0 = new_velocity.clamp_length_max(max_speed);
    }
    fn decrease(&mut self) {
        self.0 = if self.0.length() < 10.0 {
            Vec3::default()
        } else {
            self.0 * 0.9
        };
    }
    pub fn update(&mut self, direction: i32) {
        match direction {
            0 => self.decrease(),
            _ => self.increase(direction),
        }
    }
}
