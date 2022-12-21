use std::default::Default;

use glyph_brush_layout::{ab_glyph::*, *};
use image::Rgba;
use rand::{distributions::Uniform, thread_rng, Rng};

const WIDTH: u32 = 3840;
const HEIGHT: u32 = 2160;

/**
 * TODO:
 * - Allow window to set to be specified
 * - Take in configuration as user input
 */

pub struct BackgroundConfig {
    pub color: Rgba<u8>,
    pub message: String,
    pub font_name: &'static str,
    pub text_color: Rgba<u8>,
    pub width: u32,
    pub height: u32,
}

impl Default for BackgroundConfig {
    fn default() -> BackgroundConfig {
        BackgroundConfig {
            color: random_color(),
            message: "Motto".to_string(),
            font_name: "font2.ttf",
            text_color: [0u8, 0u8, 0u8, 255u8].into(),
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

fn random_color() -> Rgba<u8> {
    let mut rng = thread_rng();
    let r = rng.sample(Uniform::new(0, 255));
    let g = rng.sample(Uniform::new(0, 255));
    let b = rng.sample(Uniform::new(0, 255));

    [r, g, b, 255].into()
}
