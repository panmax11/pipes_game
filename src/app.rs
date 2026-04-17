use std::{collections::HashMap, f32::consts::PI, sync::Arc};

use nalgebra::{Vector2, vector};
use pixels::{Pixels, SurfaceTexture, wgpu::Color};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{DeviceEvent, DeviceId, StartCause, WindowEvent}, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use winit_input_helper::WinitInputHelper;

use crate::{idx, sprite::Sprite};

pub const SCREEN_WIDTH: u32 = 400;
pub const SCREEN_HEIGHT: u32 = 400;

pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'a>>,
    pub input: WinitInputHelper,
    pub sprites: HashMap<u8, Sprite>
}
impl<'a> App<'a> {
    pub fn new() -> Self {
        let brick_sprite = Sprite::from("src/brick.png");

        let mut sprites = HashMap::new();

        if let Ok(sprite) = brick_sprite {
            sprites.insert(0, sprite);
        }

        Self { window: None, pixels: None, input: WinitInputHelper::new(), sprites}
    }
    pub fn update(&mut self) {
        if let Some(pixels) = self.pixels.as_mut()  {
            pixels.clear_color(Color::BLACK);
        }

        self.draw_sprite(0, vector![200, 200], vector![100, 100], PI / 2.0);
    }
    fn draw_sprite(&mut self, id: u8, pos: Vector2<i32>, size: Vector2<u32>, rot: f32) {
        let sprite = if let Some(x) = self.sprites.get_mut(&id) {
            x
        } else {
            return;
        };

        let pixels = if let Some(x) = self.pixels.as_mut() {
            x.frame_mut()
        } else {
            return;
        };

        let rotated = sprite.rotate(0.0);

        let half_width = rotated.size.x / 2;
        let half_height = rotated.size.y / 2;

        for sprite_x in 0..rotated.size.x {
            for sprite_y in 0..rotated.size.y {
                let pos_x = pos.x as i32 + sprite_x as i32 - half_width as i32;
                let pos_y = pos.y as i32 + sprite_y as i32 - half_height as i32;

                if pos_x >= 0 && pos_x < SCREEN_WIDTH as i32 && pos_y >= 0 && pos_y < SCREEN_HEIGHT as i32 {
                    let pixels_idx = idx(pos_x, pos_y, SCREEN_WIDTH as i32) as usize * 4;
                    let sprite_idx = idx(sprite_x, sprite_y, rotated.size.x) as usize * 4;

                    if rotated.pixels[sprite_idx + 3] != 255 {
                        continue;
                    }

                    for i in 0..4 {
                        pixels[pixels_idx + i] = rotated.pixels[sprite_idx + i];
                    }
                }
            }
        }
    }
}
impl<'a> ApplicationHandler for App<'a>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes()
            .with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT)))
            .unwrap());

        let pixels = {
            let temp = Arc::clone(&window);
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, temp);
            Pixels::new(size.width, size.height, surface).unwrap()
        };

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        self.input.process_window_event(&event);

        match event
        {
            WindowEvent::CloseRequested => {
                self.window = None;
                self.pixels = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut()
                {
                    pixels.render().ok();
                }
            }
            _ => {}
        }
    }
    fn device_event(
            &mut self,
            _: &ActiveEventLoop,
            _: DeviceId,
            event: DeviceEvent,
        ) {
        self.input.process_device_event(&event);
    }
    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.input.end_step();

        self.update();

        if let Some(window) = &self.window
        {
            window.request_redraw();
        }
    }
}