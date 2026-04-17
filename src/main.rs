use num_traits::PrimInt;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

mod app;
mod sprite;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    event_loop.run_app(&mut app).unwrap();
}
pub fn idx<T>(x: T, y: T, width: T) -> T
where 
    T: PrimInt
{
    y * width + x
}