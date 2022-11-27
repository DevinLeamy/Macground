/**
 * This module is responsible for formatting and position text
 * within the bounds of the screen
 */
use std::default::Default;

use glyph_brush_layout::{*, ab_glyph::*}; 
use image::{ImageBuffer, Rgba, Pixel};

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
    fonts: Vec<FontRef<'a>>
}
impl<'a> FontLoader<'a> {
    // TODO: make private
    pub const FONTS: [&[u8]; 7] = [
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font1.otf"),
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font2.ttf"),
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font3.ttf"),
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font4.ttf"),
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font5.otf"),
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font6.otf"),
        include_bytes!("/Users/Devin/Desktop/background/motto/assets/fonts/font7.ttf"),
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
            ]
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
            font: "font1.otf".to_string(),
            text: "Motto".to_string(),
            text_scale: 40.0,
            context_bounds: Bounds::default(),
        }
    }
}

pub fn generate_glyphs(config: TextConfig) -> Vec<SectionGlyph> {
    let font_loader = FontLoader::init();

    let glyphs: Vec<SectionGlyph> = Layout::default().calculate_glyphs(
        font_loader.fonts(),
        &SectionGeometry {
            // TODO: set to text position
            screen_position: (
              config.context_bounds.width / 2.0,
              config.context_bounds.height / 2.0
            ),
            bounds: (
                config.context_bounds.width,
                config.context_bounds.height,
            )
        },
        &[
            SectionText {
                // TODO: make this configurable
                font_id: FontId(1),
                text: config.text.as_str(),
                scale: PxScale::from(config.text_scale),
                ..Default::default()
            }
        ],
    );

    
    glyphs
}

// pub struct TextBoxConfig  {
//     pub glyphs: PositionedGlyph,
//     pub position: Bounds,
// }

pub struct PositionedGlyph<'a> {
    pub glyph: OutlinedGlyph,
    pub position: Point,
    pub font: FontRef<'a>
}


pub fn draw_text<'a>(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, text_config: TextConfig) {
    let glyphs = generate_glyphs(text_config.clone());
    let font_loader = FontLoader::init();
    let fonts = font_loader.fonts();
    let font = fonts[1].clone();

    // maximum y position that is drawn
    let global_max_y = 200u32;

    let mut text_glyphs: Vec<PositionedGlyph<'a>> = vec![];

    // Collect all the positioned glyphs inside of the text
    for section_glyph in glyphs { 
        let glyph = section_glyph.glyph;
        let position = glyph.position;
        let outline_glyph = font.outline_glyph(glyph.clone()).unwrap();

        text_glyphs.push(PositionedGlyph { 
            glyph: outline_glyph,
            position: position,
            font: font.clone(),
        });
    }

    let mut global_max_y = 0u32;
    for PositionedGlyph { glyph, position, font } in text_glyphs.iter() {
        glyph.draw(|_x, y, _coverage| {
            global_max_y = u32::max(global_max_y, y);
        });
    }

    for PositionedGlyph { glyph, position, font } in text_glyphs.iter() {
        let mut glyph_max_y = 0u32;
        glyph.draw(|_x, y, _coverage| {
            glyph_max_y = u32::max(glyph_max_y, y);
        });

        let offset = global_max_y - glyph_max_y;

        glyph.draw(|x, y, coverage| {
            let color = (255.0 * coverage) as u8;
            let x_corrected = (position.x + x as f32) as u32;
            let y_corrected = (position.y + (y + offset) as f32) as u32;

            let text_color = &[255u8, 255u8, 255u8, color];

            let text_pixel = Pixel::from_slice(text_color);

            image.get_pixel_mut(x_corrected, y_corrected).blend(text_pixel);
        });
    }
}



