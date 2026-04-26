use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub const KEY_PRESS_DELAY: f32 = 0.3; // in seconds
pub const KEY_REPEAT_DELAY: f32 = 0.1;

pub struct InputAction {
    key: KeyCode,
    timer_1: f32,
    timer_2: f32,
}
impl InputAction {
    pub const fn new(key: KeyCode) -> Self {
        Self {
            key,
            timer_1: 0.0,
            timer_2: 0.0,
        }
    }
    pub fn check(&mut self, input: &WinitInputHelper) -> bool {
        let delta_time = if let Some(x) = input.delta_time() {
            x.as_secs_f32()
        } else {
            return false;
        };

        if input.key_pressed(self.key) {
            self.timer_1 = 0.0;
            self.timer_2 = 0.0;
            return true;
        }

        if input.key_held(self.key) {
            self.timer_1 += delta_time;

            if self.timer_1 > KEY_PRESS_DELAY {
                self.timer_2 += delta_time;

                if self.timer_2 > KEY_REPEAT_DELAY {
                    self.timer_2 = 0.0;
                    return true;
                }
            }
        } else {
            self.timer_1 = 0.0;
            self.timer_2 = 0.0;
            return false;
        }

        false
    }
}
