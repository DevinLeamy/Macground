#[allow(unused_variables)]
use lazy_static::lazy_static;
use std::path::PathBuf;

use std::{collections::HashMap, default::Default, path::Path};

use glyph_brush_layout::{ab_glyph::*, *};
use image::{Pixel, Rgba};

use crate::BackgroundImage;

lazy_static! {
    pub static ref FONT_LOADER: FontLoader<'static> = FontLoader::new();
}

pub struct FontLoader<'a> {
    /// Raw bytes of loaded fonts
    fonts_raw: Vec<Vec<u8>>,
    /// Mapping of font paths to font refs
    fonts: HashMap<String, FontRef<'a>>,
}
impl<'a> FontLoader<'a> {
    pub fn new() -> FontLoader<'a> {
        let mut loader = Self {
            fonts_raw: vec![],
            fonts: HashMap::new(),
        };

        loader.load_font(font_path("font1.otf").as_path());

        loader
    }

    pub fn load_font(&mut self, path: &Path) -> () {
        let bytes = std::fs::read(path).unwrap();
        self.fonts_raw.push(bytes);
    }

    pub fn font(&'a self, _path: &Path) -> FontRef<'a> {
        FontRef::try_from_slice(&self.fonts_raw[0]).unwrap()
    }
}

#[derive(Clone)]
pub struct TextConfig {
    /// Path to the font used to display the text
    pub font_path: PathBuf,
    /// Scale of text in pixels
    pub text_scale: f32,
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
            font_path: font_path("font1.otf"),
            text_scale: 40.0,
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
    let glyphs: Vec<SectionGlyph> = text_style.layout.calculate_glyphs(
        &[(*FONT_LOADER).font(text_style.font_path.as_path())],
        &SectionGeometry {
            bounds: (textbox.width as f32, textbox.height as f32),
            ..Default::default()
        },
        &[SectionText {
            font_id: FontId(0),
            text: textbox.text.as_str(),
            scale: PxScale::from(text_style.text_scale), // Pixel-height of the text
            ..Default::default()
        }],
    );

    let mut outlined_glyphs = vec![];
    let font = (*FONT_LOADER).font(text_style.font_path.as_path());

    for section_glyph in glyphs {
        let raw_glyph = section_glyph.glyph;
        if let Some(glyph) = font.outline_glyph(raw_glyph.clone()) {
            outlined_glyphs.push(glyph);
        }
    }

    outlined_glyphs
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

pub fn font_path(name: &str) -> PathBuf {
    let path = format!("{}/assets/fonts/{name}", env!("CARGO_MANIFEST_DIR"));
    PathBuf::from(path)
}
