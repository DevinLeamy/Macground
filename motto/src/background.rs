use std::default::Default;
use std::path::PathBuf;

use image::Rgba;

use crate::text::font_path;

const WIDTH: u32 = 3840;
const HEIGHT: u32 = 2160;

/**
 * TODO:
 * - Allow window to set to be specified
 * - Take in configuration as user input
 */

pub struct BackgroundConfig {
    /// Color of the background
    pub color: Rgba<u8>,
    /// Message to be displayed
    pub message: String,
    /// Name of the text font (perhaps change to path?)
    pub font_path: PathBuf,
    /// Color of the text
    pub text_color: Rgba<u8>,
    /// Width of the background
    pub width: u32,
    /// Height of the background
    pub height: u32,
}

impl Default for BackgroundConfig {
    fn default() -> BackgroundConfig {
        BackgroundConfig {
            color: crate::random_color(),
            message: "Motto".to_string(),
            font_path: font_path("font2.ttf"),
            text_color: [0u8, 0u8, 0u8, 255u8].into(),
            // Perhaps we can retrieve the window dimensions using osascript?
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
