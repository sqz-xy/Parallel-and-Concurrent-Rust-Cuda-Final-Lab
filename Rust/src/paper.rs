use crate::{Vector3f};
use bmp::{Image, Pixel};

/* 
vec[x + y * width]
*/

pub struct Paper {
    // 1000 x 1000 pixels, each "pixel" is a vector which can be used for colour
    pixels: Image,
    pub position: Vector3f,
    pub width: f32,
    pub height: f32
}

impl Paper {
    pub fn new (p_pos: Vector3f, p_width: f32, p_height: f32) -> Paper {
        Paper {
            width: p_width,
            height: p_height,
            pixels: Image::new(p_width as u32 * 1000, p_height as u32 * 1000),
            position : p_pos
        }
    }

    // Clears the paper
    pub fn clear(&mut self) {
        for i in 0..1000 {
            for j in 0..1000 {
                self.set_pixel(i as f32, j as f32, Vector3f::new(255.0, 255.0, 255.0));
            }      
        }
    }

    // Colours are multiplied and divided by 255 to transform the colours from 0-1 scale
    // Sets pixel colour
    pub fn set_pixel(&mut self, p_x: f32, p_y: f32, p_colour: Vector3f) {
        let pixel = Pixel::new((p_colour.x * 255.0) as u8, (p_colour.y * 255.0) as u8, (p_colour.z * 255.0) as u8);
        self.pixels.set_pixel(p_x as u32, p_y as u32, pixel);
    }

    // Get pixel colour 
    pub fn get_pixel(&self, p_x: f32, p_y: f32) -> Vector3f {
        let pixel = self.pixels.get_pixel(p_x as u32, p_y as u32);
        return Vector3f::new(((pixel.r) as f32) / 255.0, ((pixel.g) as f32) / 255.0, ((pixel.b) as f32) / 255.0);
    }

    // Saves the paper as bmp
    pub fn save(&self, p_file_name: &str) {
        let _ = self.pixels.save(p_file_name);
    }
}