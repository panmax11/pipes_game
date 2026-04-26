use std::collections::HashMap;

use nalgebra::Vector2;

use crate::tween::Tween;

pub struct TweenManager {
    pub tweens: HashMap<Vector2<usize>, Tween>,
}
impl TweenManager {
    pub fn new() -> Self {
        Self {
            tweens: HashMap::new(),
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        for tween in self.tweens.values_mut() {
            tween.step(delta_time);
        }
    }
    pub fn clear(&mut self) {
        let mut to_remove = vec![];

        for (pos, tween) in self.tweens.iter_mut() {
            if tween.is_done {
                to_remove.push(*pos);
            }
        }

        for pos in to_remove {
            self.tweens.remove(&pos);
        }
    }
}
