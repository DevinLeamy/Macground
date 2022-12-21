#[allow(unused_variables)]
/**
 * This module is responsible for formatting and position text
 * within the bounds of the screen
 */
use std::default::Default;

use glyph_brush_layout::{ab_glyph::*, *};
use image::{ImageBuffer, Pixel, Rgba};

/**
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

pub struct FontLoader<'a> {
    fonts: Vec<FontRef<'a>>,
}
impl<'a> FontLoader<'a> {
    // TODO: make private
    pub const FONTS: [&[u8]; 7] = [
        include_bytes!("../assets/fonts/font1.otf"),
        include_bytes!("../assets/fonts/font3.ttf"),
        include_bytes!("../assets/fonts/font3.ttf"),
        include_bytes!("../assets/fonts/font4.ttf"),
        include_bytes!("../assets/fonts/font5.otf"),
        include_bytes!("../assets/fonts/font6.otf"),
        include_bytes!("../assets/fonts/font7.ttf"),
    ];

    fn init() -> FontLoader<'a> {
        FontLoader {
            fonts: vec![
                FontRef::try_from_slice(FontLoader::FONTS[0]).unwrap(),
                FontRef::try_from_slice(FontLoader::FONTS[1]).unwrap(),
                FontRef::try_from_slice(FontLoader::FONTS[2]).unwrap(),
                FontRef::try_from_slice(FontLoader::FONTS[3]).unwrap(),
                FontRef::try_from_slice(FontLoader::FONTS[4]).unwrap(),
                FontRef::try_from_slice(FontLoader::FONTS[5]).unwrap(),
                FontRef::try_from_slice(FontLoader::FONTS[6]).unwrap(),
            ],
        }
    }

    fn fonts(&self) -> &[FontRef<'a>] {
        self.fonts.as_slice()
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
    pub font: String,
    pub text: String,
    pub text_scale: f32,
    pub context_bounds: Bounds,
}

impl Default for TextConfig {
    fn default() -> TextConfig {
        TextConfig {
            font: "font4.otf".to_string(),
            text: "Motto".to_string(),
            text_scale: 40.0,
            context_bounds: Bounds::default(),
        }
    }
}

pub fn generate_glyphs(config: TextConfig) -> Vec<SectionGlyph> {
    let font_loader = FontLoader::init();
    let layout = Layout::SingleLine {
        h_align: HorizontalAlign::Center,
        v_align: VerticalAlign::Center,
        line_breaker: BuiltInLineBreaker::default(),
    };

    let glyphs: Vec<SectionGlyph> = layout.calculate_glyphs(
        font_loader.fonts(),
        &SectionGeometry {
            // TODO: set to text position
            screen_position: (
                config.context_bounds.width / 2.0,
                config.context_bounds.height / 3.0,
            ),
            bounds: (config.context_bounds.width, config.context_bounds.height),
        },
        &[SectionText {
            // TODO: make this configurable
            font_id: FontId(1),
            text: config.text.as_str(),
            scale: PxScale::from(400.0), // Pixel-height of the text
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
    let font_loader = FontLoader::init();
    let fonts = font_loader.fonts();
    let font = fonts[1].clone();

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
        glyph.draw(|_x, y, _coverage| {
            global_max_y = u32::max(global_max_y, y);
        });
    }

    for PositionedGlyph(glyph, position) in text_glyphs.iter() {
        let mut glyph_max_y = 0u32;
        let mut glyph_max_x = 0u32;
        glyph.draw(|x, y, _coverage| {
            glyph_max_y = u32::max(glyph_max_y, y);
            glyph_max_x = u32::max(glyph_max_x, x);
        });

        let offset = global_max_y - glyph_max_y;

        glyph.draw(|x, y, coverage| {
            let alpha = (255.0 * coverage) as u8;
            let x_corrected = (position.x + x as f32) as u32;
            let y_corrected = (position.y + (y + offset) as f32) as u32;

            let text_color = &[255u8, 255u8, 255u8, alpha];
            let text_pixel = Pixel::from_slice(text_color);

            image
                .get_pixel_mut(x_corrected, y_corrected)
                .blend(text_pixel);
        });
    }
}
