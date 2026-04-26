use std::f32::consts::PI;

use crate::lerp;

pub struct Tween {
    pub value: f32,
    pub target: f32,
    pub duration: f32,
    pub is_done: bool,
    timer: f32,
}
impl Tween {
    pub fn new(value: f32, target: f32, duration: f32) -> Self {
        Self {
            value,
            target,
            duration,
            is_done: false,
            timer: 0.0,
        }
    }
    pub fn step(&mut self, delta_time: f32) {
        let factor = self.timer / self.duration;

        let mut diff = (self.target - self.value).rem_euclid(2.0 * PI);

        if diff > PI {
            diff -= PI * 2.0;
        }

        let new_value = lerp(self.value, self.value + diff, factor).rem_euclid(2.0 * PI);
        self.value = new_value;

        if factor > 0.9 {
            self.value = self.target;
            self.is_done = true;
            return;
        }

        self.timer += delta_time;
    }
}
