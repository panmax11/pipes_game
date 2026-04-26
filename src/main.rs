use num_traits::PrimInt;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

mod app;
mod direction;
mod input;
mod sprite;
mod tween;
mod tween_manager;
mod wfc;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    event_loop.run_app(&mut app).unwrap();
}
pub fn idx<T>(x: T, y: T, width: T) -> T
where
    T: PrimInt,
{
    y * width + x
}
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}
