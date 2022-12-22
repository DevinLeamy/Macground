#[allow(unused_variables)]
use lazy_static::lazy_static;
use std::path::PathBuf;
/**
 * This module is responsible for formatting and position text
 * within the bounds of the screen
 */
use std::{collections::HashMap, default::Default, path::Path};

use glyph_brush_layout::{ab_glyph::*, *};
use image::{ImageBuffer, Pixel, Rgba};

use crate::BackgroundImage;

/*
 * TODO:
 * - Allow loading fonts at runtime
 * - Store fonts in a better way
 *  - Avoid conflicts between `FontRef` and `FontId`
 * - Allow text to be left/right/center aligned
 *  - Include in text config
 * - Allow configuration of font size
 * - Create default for TextConfig
 *
 */

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
        println!("{:?}", path);
        let bytes = std::fs::read(path).unwrap();
        self.fonts_raw.push(bytes);
    }

    pub fn font(&'a self, path: &Path) -> FontRef<'a> {
        FontRef::try_from_slice(&self.fonts_raw[0]).unwrap()
    }
}

/**
 * Bounds that the text must lie within.
 */
#[derive(Default, Clone)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone)]
pub struct TextConfig {
    /// Path to the font used to display the text
    pub font_path: PathBuf,
    /// Text to display
    pub text: String,
    /// Scale of text in pixels
    pub text_scale: f32,
    /// Bounds of the window on which the text is drawn
    pub context_bounds: Bounds,
}

impl Default for TextConfig {
    fn default() -> TextConfig {
        TextConfig {
            font_path: font_path("font1.otf"),
            text: "Motto".to_string(),
            text_scale: 40.0,
            context_bounds: Bounds::default(),
        }
    }
}

pub fn generate_glyphs(config: TextConfig) -> Vec<SectionGlyph> {
    let layout = Layout::SingleLine {
        h_align: HorizontalAlign::Center,
        v_align: VerticalAlign::Center,
        line_breaker: BuiltInLineBreaker::default(),
    };

    let glyphs: Vec<SectionGlyph> = layout.calculate_glyphs(
        &[(*FONT_LOADER).font(config.font_path.as_path())],
        &SectionGeometry {
            screen_position: (
                config.context_bounds.width / 2.0,
                config.context_bounds.height / 2.0,
            ),
            ..Default::default()
        },
        &[SectionText {
            font_id: FontId(0),
            text: config.text.as_str(),
            scale: PxScale::from(config.text_scale), // Pixel-height of the text
            ..Default::default()
        }],
    );

    glyphs
}

pub fn draw_text<'a>(image: &mut BackgroundImage, text_config: TextConfig) {
    let glyphs = generate_glyphs(text_config.clone());
    let font = (*FONT_LOADER).font(text_config.font_path.as_path());

    for section_glyph in glyphs {
        let raw_glyph = section_glyph.glyph;
        if let Some(glyph) = font.outline_glyph(raw_glyph.clone()) {
            let bounds = glyph.px_bounds();

            glyph.draw(|x, y, coverage| {
                let alpha = (255.0 * coverage) as u8;
                let x_corrected = (bounds.min.x + x as f32) as u32;
                let y_corrected = (bounds.min.y + y as f32) as u32;

                let text_color = &[255u8, 255u8, 255u8, alpha];
                let text_pixel = Pixel::from_slice(text_color);

                image.set_pixel(x_corrected, y_corrected, text_pixel);
            });
        } else {
            println!("Could not outline glyph {:?}", raw_glyph);
        }
    }
}

pub fn font_path(name: &str) -> PathBuf {
    let path = format!("{}/assets/fonts/{name}", env!("CARGO_MANIFEST_DIR"));
    PathBuf::from(path)
}
