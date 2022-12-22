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
    pub font_path: PathBuf,
    pub text: String,
    pub text_scale: f32,
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
                config.context_bounds.height / 3.0,
            ),
            ..Default::default()
        },
        &[SectionText {
            // TODO: make this configurable
            font_id: FontId(0),
            text: config.text.as_str(),
            scale: PxScale::from(100.0), // Pixel-height of the text
            ..Default::default()
        }],
    );

    glyphs
}

/// A glyph positioned on a screen.
pub struct PositionedGlyph(
    /// Outlined glyph to be drawn.
    OutlinedGlyph,
    /// Screen position where the glyph is to be drawn.
    Point,
);

pub fn draw_text<'a>(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, text_config: TextConfig) {
    let glyphs = generate_glyphs(text_config.clone());
    let font = (*FONT_LOADER).font(text_config.font_path.as_path());

    let mut text_glyphs: Vec<PositionedGlyph> = vec![];

    // Collect all the positioned glyphs inside of the text
    for section_glyph in glyphs {
        let position = section_glyph.glyph.position;
        let glyph = font.outline_glyph(section_glyph.glyph).unwrap();

        text_glyphs.push(PositionedGlyph(glyph, position));
    }

    let mut global_max_y = 0u32;
    /*
    We first determine the highest "y" point in all of the text. Then, for each
    glyph that we draw, we determine it's highest "y" point. We then take the difference
    of the global max y and the local max y. Since y values grow downwards, this tells us the
    how much we need to offset the y value of each pixel in the glyph we are drawning such that
    all characters end up with the same highest y value. Aesthetically, this means that all
    characters will share a common baseline (like a line on a piece of paper).
     */
    for PositionedGlyph(glyph, _position) in text_glyphs.iter() {
        let bounds = glyph.px_bounds();
        global_max_y = u32::max(global_max_y, bounds.max.y as u32);
    }

    for PositionedGlyph(glyph, position) in text_glyphs.iter() {
        let bounds = glyph.px_bounds();
        println!("Bounds: {:?}", bounds);
        let glyph_max_y = bounds.max.y as u32;

        let offset = 0; // global_max_y - glyph_max_y;

        println!("Offset: {offset}");

        let mut max_pos = 0.0;

        glyph.draw(|x, y, coverage| {
            let alpha = (255.0 * coverage) as u8;
            let x_corrected = (bounds.min.x + x as f32) as u32;
            let y_corrected = (bounds.min.y + (y + offset) as f32) as u32;

            let text_color = &[255u8, 255u8, 255u8, alpha];
            let text_pixel = Pixel::from_slice(text_color);

            max_pos = f32::max(max_pos, y as f32);

            image
                .get_pixel_mut(x_corrected, y_corrected)
                .blend(text_pixel);
        });

        println!("max: {max_pos}");
    }
}

pub fn font_path(name: &str) -> PathBuf {
    let path = format!("{}/assets/fonts/{name}", env!("CARGO_MANIFEST_DIR"));
    PathBuf::from(path)
}

fn draw_glyphs(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, text: &str) {
    // Create the positioned glyphs
    // Draw the glyphs on the screen
    let layout = Layout::default_single_line();
    let glyphs = layout.calculate_glyphs(
        &[(*FONT_LOADER).font(font_path("font1.otf").as_path())],
        &SectionGeometry {
            screen_position: (0.0, 0.0),
            ..Default::default()
        },
        &[SectionText {
            // TODO: make this configurable
            font_id: FontId(0),
            text,
            scale: PxScale::from(400.0), // Pixel-height of the text
            ..Default::default()
        }],
    );
    // for glyph in glyphs {
    //     glyph.
    // }
}
