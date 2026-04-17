use std::f32::consts::PI;

use image::{ImageError, ImageReader};
use nalgebra::{Vector2, vector};

use crate::idx;

pub struct Sprite {
    pub pixels: Vec<u8>,
    pub size: Vector2<u32>
}
impl Sprite {
    pub fn from(path: &str) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?.to_rgba8();

        let (width, height) = img.dimensions();

        let mut pixels = Vec::with_capacity((width * height) as usize);

        for pixel in img.pixels()
        {
            for color in pixel.0
            {
                pixels.push(color);
            }
        }

        let size = vector![width, height];

        Ok(Self {
            pixels,
            size
        })
    }
    pub fn new(pixels: Vec<u8>, size: Vector2<u32>) -> Self {
        Self {
            pixels,
            size
        }
    }
    pub fn scale(&self, size: Vector2<u32>) -> Sprite {
        let mut pixels = Vec::with_capacity((size.x * size.y) as usize);

        let factor_x = self.size.x as f32 / size.x as f32;
        let factor_y = self.size.y as f32 / size.y as f32;

        /*
        if rot == 0 {
            for sprite_y in 0..size.y {
                for sprite_x in 0..size.x {
                    self.scale_step(factor_x, factor_y, sprite_x, sprite_y, &mut pixels);
                }
            }
        }

        if rot == 1 {
            for sprite_x in 0..size.x {
                for sprite_y in (0..size.y).rev() {
                    self.scale_step(factor_x, factor_y, sprite_x, sprite_y, &mut pixels);
                }
            }
        }

        if rot == 2 {
            for sprite_y in (0..size.y).rev() {
                for sprite_x in (0..size.x).rev(){
                    self.scale_step(factor_x, factor_y, sprite_x, sprite_y, &mut pixels);
                }
            }
        }

        if rot == 3 {
            for sprite_x in (0..size.x).rev() {
                for sprite_y in 0..size.y {
                    self.scale_step(factor_x, factor_y, sprite_x, sprite_y, &mut pixels);
                }
            }
        }
        */
        for sprite_y in 0..size.y {
            for sprite_x in 0..size.x {
                 self.scale_step(factor_x, factor_y, sprite_x, sprite_y, &mut pixels);
            }
        }

        Sprite::new(pixels, size)
    }
    fn scale_step(&self, factor_x: f32, factor_y: f32, sprite_x: u32, sprite_y: u32, pixels: &mut Vec<u8>) {
        let scaled_x = (sprite_x as f32 * factor_x) as u32;
        let scaled_y = (sprite_y as f32 * factor_y) as u32;

        let idx = idx(scaled_x, scaled_y, self.size.x) as usize * 4;

        for i in 0..4
        {
            pixels.push(self.pixels[idx + i]);
        }
    }
    pub fn rotate(&self, rot: f32) -> Sprite {
        // skew horizontal first
        // then vertical
        // and horizontal again, but in opposite dir
        // i'll store the positions as vectors so it'll be easier to manipulate

        let cos = rot.cos();
        let sin = rot.sin();

        // aabb for the rotated rectangle
        let new_width = ((self.size.x as f32 * cos).abs() + (self.size.y as f32 * sin).abs()) as u32;
        let new_height = ((self.size.x as f32 * sin).abs() + (self.size.y as f32 * cos).abs()) as u32;

        let mut new_pixels = vec![0; (new_width * new_height) as usize * 4];

        let margin_x = (new_width - self.size.x) / 2;
        let margin_y = (new_height - self.size.y) / 2;

        // COPY FIRST
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let og_idx = idx(x, y, self.size.x) as usize * 4;
                let new_idx = idx(x + margin_x, y + margin_y, new_width) as usize * 4;

                for i in 0..4 {
                    new_pixels[new_idx + i] = self.pixels[og_idx + i];
                }
            }
        }

        Sprite::new(new_pixels, vector![new_width, new_height])
    }
}