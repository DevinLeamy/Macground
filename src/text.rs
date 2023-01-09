#[allow(unused_variables)]
use lazy_static::lazy_static;
// use std::path::PathBuf;
use std::sync::Mutex;

use std::{collections::HashMap, default::Default, path::Path};

use glyph_brush_layout::{ab_glyph::*, *};
use image::{Pixel, Rgba};

use crate::BackgroundImage;

lazy_static! {
    pub static ref FONT_LOADER: FontLoader = FontLoader::new();
}

pub struct FontLoader {
    /// Mapping of font paths to font refs
    fonts: Mutex<HashMap<String, FontRef<'static>>>,
}
impl FontLoader {
    pub fn new() -> FontLoader {
        // Load the default font
        let default_font = include_bytes!("../assets/fonts/font1.otf");
        let font_ref = FontRef::try_from_slice(default_font).unwrap();

        let loader = Self {
            fonts: Mutex::new(HashMap::new()),
        };

        loader
            .fonts
            .lock()
            .unwrap()
            .insert("default".to_string(), font_ref);

        loader
    }

    pub fn load_font(&self, name: String, path: &Path) -> () {
        // Create a static reference to the font using Vec<_>.leak()
        let bytes = std::fs::read(path).unwrap().leak();
        let font_ref = FontRef::try_from_slice(bytes).unwrap();
        self.fonts.lock().unwrap().insert(name, font_ref);
    }

    pub fn font(&self, name: String) -> FontRef<'_> {
        self.fonts.lock().unwrap().get(&name).unwrap().clone()
    }
}

#[derive(Clone)]
pub enum TextSize {
    /// Text scale in pixels
    PxScale(f32),
    /// Fill the box containing the text
    FillParent,
}

#[derive(Clone)]
pub struct TextConfig {
    /// Name of the font (as found in the [`FontLoader`])
    pub font: String,
    /// Size of the text
    pub size: TextSize,
    /// Color of the text
    pub color: Rgba<u8>,
    /// Text layout
    pub layout: Layout<BuiltInLineBreaker>,
}

impl Default for TextConfig {
    fn default() -> TextConfig {
        let layout = Layout::Wrap {
            h_align: HorizontalAlign::Center,
            v_align: VerticalAlign::Center,
            line_breaker: BuiltInLineBreaker::UnicodeLineBreaker,
        };

        TextConfig {
            font: "default".to_string(),
            size: TextSize::FillParent,
            color: *Rgba::from_slice(&[255, 255, 255, 255]),
            layout,
        }
    }
}

pub struct TextBox {
    /// The text inside of the text box
    pub text: String,
    /// Width of the text box
    pub width: u32,
    /// Height of the text box
    pub height: u32,
    /// Configuration options for text
    pub style: TextConfig,
}

// Draws a textbox onto a background image at the given position
pub fn draw_textbox(image: &mut BackgroundImage, textbox: TextBox, screen_x: u32, screen_y: u32) {
    let glyphs = generate_textbox_glyphs(&textbox);
    draw_text(image, glyphs, textbox.style, screen_x, screen_y);
}

/// Generates outlined glyphs positioned at (0, 0) on the screen
pub fn generate_textbox_glyphs(textbox: &TextBox) -> Vec<OutlinedGlyph> {
    let text_style = &textbox.style;
    let font_ref = (*FONT_LOADER).font("default".to_string());

    match text_style.size {
        TextSize::PxScale(scale) => {
            let glyphs: Vec<SectionGlyph> = text_style.layout.calculate_glyphs(
                &[font_ref.clone()],
                &SectionGeometry {
                    bounds: (textbox.width as f32, textbox.height as f32),
                    ..Default::default()
                },
                &[SectionText {
                    font_id: FontId(0),
                    text: textbox.text.as_str(),
                    scale: PxScale::from(scale), // Pixel-height of the text
                    ..Default::default()
                }],
            );

            let mut outlined_glyphs = vec![];
            for section_glyph in glyphs {
                let raw_glyph = section_glyph.glyph;
                if let Some(glyph) = font_ref.outline_glyph(raw_glyph.clone()) {
                    outlined_glyphs.push(glyph);
                }
            }

            outlined_glyphs
        }
        TextSize::FillParent => {
            // Attempts to draw the text with the given text size. If it cannot draw it within the bounds,
            // None is returned.
            let attempt_text_size = |text_size: f32| -> Option<Vec<OutlinedGlyph>> {
                let glyphs: Vec<SectionGlyph> = text_style.layout.calculate_glyphs(
                    &[font_ref.clone()],
                    &SectionGeometry {
                        bounds: (textbox.width as f32, textbox.height as f32),
                        ..Default::default()
                    },
                    &[SectionText {
                        font_id: FontId(0),
                        text: textbox.text.as_str(),
                        scale: PxScale::from(text_size), // Pixel-height of the text
                        ..Default::default()
                    }],
                );

                let mut outlined_glyphs = vec![];
                for section_glyph in glyphs {
                    let raw_glyph = section_glyph.glyph;
                    if let Some(glyph) = font_ref.outline_glyph(raw_glyph.clone()) {
                        if within_bounds(&glyph, &textbox) {
                            outlined_glyphs.push(glyph);
                        } else {
                            return None;
                        }
                    }
                }

                Some(outlined_glyphs)
            };

            // Find the text size that will fill the text box by increasing and
            // decreasing the text size as required.
            //
            // TODO: This is a very slow approach to testing text sizes! Can definitely
            // be made faster through binary search and/or something else.

            let mut text_size = 20.0; // Some "random" starting text size
            while attempt_text_size(text_size + 1.0).is_some() {
                text_size += 1.0;
            }
            while attempt_text_size(text_size).is_none() {
                text_size -= 1.0;
            }

            attempt_text_size(text_size).unwrap()
        }
    }
}

/// Check if a glyph lines within the bounds of a textbox
pub fn within_bounds(glyph: &OutlinedGlyph, textbox: &TextBox) -> bool {
    let half_width = textbox.width as f32 / 2.0;
    let half_height = textbox.height as f32 / 2.0;
    let bounds = glyph.px_bounds();

    if bounds.min.x < -half_width || bounds.max.x >= half_width {
        false
    } else if bounds.min.y < -half_height || bounds.max.y >= half_height {
        false
    } else {
        true
    }
}

/// Draws text to the screen at a given screen position (top-left coordinates)
pub fn draw_text(
    image: &mut BackgroundImage,
    glyphs: Vec<OutlinedGlyph>,
    text_config: TextConfig,
    screen_x: u32,
    screen_y: u32,
) {
    for glyph in glyphs {
        let bounds = glyph.px_bounds();

        glyph.draw(|x, y, coverage| {
            // Offset (x, y) by the screen position of the text
            let x = screen_x + x;
            let y = screen_y + y;

            let alpha = (255.0 * coverage) as u8;
            let x_corrected = (bounds.min.x + x as f32) as u32;
            let y_corrected = (bounds.min.y + y as f32) as u32;

            let color = Rgba::from([
                text_config.color.0[0],
                text_config.color.0[1],
                text_config.color.0[2],
                alpha,
            ]);

            image.set_pixel(x_corrected, y_corrected, &color);
        });
    }
}

// pub fn font_path(name: &str) -> PathBuf {
//     let path = format!("{}/assets/fonts/{name}", env!("CARGO_MANIFEST_DIR"));
//     PathBuf::from(path)
// }
