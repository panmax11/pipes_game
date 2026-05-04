use std::f32::consts::PI;

use image::{ImageError, ImageReader};
use nalgebra::{Vector2, vector};

use crate::idx;

#[derive(Clone)]
pub struct Sprite {
    pub pixels: Vec<u8>,
    pub size: Vector2<u32>,
}
impl Sprite {
    pub fn from(path: &str) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?.to_rgba8();

        let (width, height) = img.dimensions();

        let mut pixels = Vec::with_capacity((width * height) as usize);

        for pixel in img.pixels() {
            for color in pixel.0 {
                pixels.push(color);
            }
        }

        let size = vector![width, height];

        Ok(Self { pixels, size })
    }
    pub const fn new(pixels: Vec<u8>, size: Vector2<u32>) -> Self {
        Self { pixels, size }
    }
    pub fn scale(&self, size: Vector2<u32>) -> Sprite {
        let mut pixels = Vec::with_capacity((size.x * size.y) as usize);

        let factor_x = self.size.x as f32 / size.x as f32;
        let factor_y = self.size.y as f32 / size.y as f32;

        for sprite_y in 0..size.y {
            for sprite_x in 0..size.x {
                let scaled_x = (sprite_x as f32 * factor_x) as u32;
                let scaled_y = (sprite_y as f32 * factor_y) as u32;

                let idx = idx(scaled_x, scaled_y, self.size.x) as usize * 4;

                for i in 0..4 {
                    pixels.push(self.pixels[idx + i]);
                }
            }
        }

        Sprite::new(pixels, size)
    }
    pub fn rotate(&self, rot: f32) -> Sprite {
        let right_angles = (rot / (PI / 2.0)).floor() as i32;
        let angle_left = rot - right_angles as f32 * (PI / 2.0);

        let sin = angle_left.sin();
        let tan = -(angle_left / 2.0).tan();

        let full_cos = rot.cos();
        let full_sin = rot.sin();

        let new_width =
            ((self.size.x as f32 * full_cos).abs() + (self.size.y as f32 * full_sin).abs()) as u32;
        let new_height =
            ((self.size.x as f32 * full_sin).abs() + (self.size.y as f32 * full_cos).abs()) as u32;
        let max_width = ((new_width.pow(2) as f32 + new_height.pow(2) as f32).sqrt() * 1.5) as u32;
        let mut pixels = vec![0; max_width.pow(2) as usize * 4];

        let margin_x = (max_width - self.size.x) / 2;
        let margin_y = (max_width - self.size.y) / 2;

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let old_idx = idx(x, y, self.size.x) as usize * 4;
                let new_idx = idx(x + margin_x, y + margin_y, max_width) as usize * 4;

                for i in 0..4 {
                    pixels[new_idx + i] = self.pixels[old_idx + i];
                }
            }
        }

        let mut temp_buffer = vec![0; max_width.pow(2) as usize * 4];

        for _ in 0..right_angles {
            for x in 0..max_width {
                for y in 0..max_width {
                    let new_x = max_width - 1 - y;
                    let new_y = x;

                    let old_idx = idx(x, y, max_width) as usize * 4;
                    let new_idx = idx(new_x, new_y, max_width) as usize * 4;

                    for i in 0..4 {
                        temp_buffer[new_idx + i] = pixels[old_idx + i];
                    }
                }
            }

            pixels = temp_buffer.clone();
        }

        // HORIZONTAL SKEW
        for y in 0..max_width {
            let dist_y = y as i32 - max_width as i32 / 2;
            let offset = (dist_y as f32 * tan) as i32;

            let range: Vec<u32> = if offset > 0 {
                (0..max_width).rev().collect()
            } else {
                (0..max_width).collect()
            };

            for x in range {
                let new_x = x as i32 + offset;

                if new_x >= 0 && new_x < max_width as i32 {
                    let old_idx = idx(x, y, max_width) as usize * 4;
                    let new_idx = idx(new_x as u32, y, max_width) as usize * 4;

                    if old_idx == new_idx {
                        continue;
                    }

                    for i in 0..4 {
                        pixels[new_idx + i] = pixels[old_idx + i];
                        pixels[old_idx + i] = 0;
                    }
                }
            }
        }

        // VERTICAL SKEW
        for x in 0..max_width {
            let dist_x = x as i32 - max_width as i32 / 2;
            let offset = (dist_x as f32 * sin) as i32;

            let range: Vec<u32> = if offset > 0 {
                (0..max_width).rev().collect()
            } else {
                (0..max_width).collect()
            };

            for y in range {
                let new_y = y as i32 + offset;

                if new_y >= 0 && new_y < max_width as i32 {
                    let old_idx = idx(x, y, max_width) as usize * 4;
                    let new_idx = idx(x, new_y as u32, max_width) as usize * 4;

                    if old_idx == new_idx {
                        continue;
                    }

                    for i in 0..4 {
                        pixels[new_idx + i] = pixels[old_idx + i];
                        pixels[old_idx + i] = 0;
                    }
                }
            }
        }

        // HORIZONTAL SKEW
        for y in 0..max_width {
            let dist_y = y as i32 - max_width as i32 / 2;
            let offset = (dist_y as f32 * tan) as i32;

            let range: Vec<u32> = if offset > 0 {
                (0..max_width).rev().collect()
            } else {
                (0..max_width).collect()
            };

            for x in range {
                let new_x = x as i32 + offset;

                if new_x >= 0 && new_x < max_width as i32 {
                    let old_idx = idx(x, y, max_width) as usize * 4;
                    let new_idx = idx(new_x as u32, y, max_width) as usize * 4;

                    if old_idx == new_idx {
                        continue;
                    }

                    for i in 0..4 {
                        pixels[new_idx + i] = pixels[old_idx + i];
                        pixels[old_idx + i] = 0;
                    }
                }
            }
        }

        let mut new_pixels = vec![0; (new_width * new_height) as usize * 4];

        let last_margin_x = (max_width - new_width) / 2;
        let last_margin_y = (max_width - new_height) / 2;

        for y in 0..new_height {
            for x in 0..new_width {
                let old_idx = idx(x + last_margin_x, y + last_margin_y, max_width) as usize * 4;
                let new_idx = idx(x, y, new_width) as usize * 4;

                for i in 0..4 {
                    new_pixels[new_idx + i] = pixels[old_idx + i];
                }
            }
        }

        let new_size = vector![new_width, new_height];
        Sprite::new(new_pixels, new_size)
    }
}
