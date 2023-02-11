use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct RawOptions {
    /// Url of a background image
    #[arg(long)]
    pub background_image: Option<String>,
    /// Flag to set the background to a random image
    #[arg(long)]
    pub random_image: bool,
    /// Color of the background, if no image is set.
    /// Accepts: "<color-name>" | "rgb(...)" | "#FFAAEE" | "hsl(...)" | "random"
    #[arg(long)]
    pub color: Option<String>,
    /// Message to display to the screen
    #[arg(long)]
    pub message: Option<String>,
    /// Random quote to display to the screen
    #[arg(long)]
    pub random_quote: bool,
    /// Random would to display to the screen
    #[arg(long)]
    pub random_word: bool,
    /// Color of the text, if any is displayed
    /// Accepts: "<color-name>" | "rgb(...)" | "#FFAAEE" | "hsl(...)" | "random"
    #[arg(long)]
    pub text_color: Option<String>,
    /// Size of the text characters in pixels, defaults to filling
    /// the text's parent.
    #[arg(long)]
    pub text_size: Option<u32>,
    // Path to an otf or ttf font
    // #[arg(long)]
    // pub font: Option<String>,
}

// save layouts.
#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub background: BackgroundOptions,
    pub text: TextOptions,
    pub font: FontOptions,
}

impl Options {
    pub fn from(raw_options: RawOptions) -> Self {
        let mut background = BackgroundOptions::Color("random".to_string());
        let mut text = TextOptions::RandomWord;
        let mut font = FontOptions {
            font_path: None,
            color: "white".to_string(),
            font_size: None, // Fill the parent
        };

        if let Some(color) = raw_options.color {
            background = BackgroundOptions::Color(color);
        } else if raw_options.random_image {
            background = BackgroundOptions::RandomImage;
        } else if let Some(url) = raw_options.background_image {
            background = BackgroundOptions::Url(url);
        };

        if let Some(message) = raw_options.message {
            text = TextOptions::Message(message);
        } else if raw_options.random_quote {
            text = TextOptions::RandomQuote;
        } else if raw_options.random_word {
            text = TextOptions::RandomWord;
        }

        if let Some(size) = raw_options.text_size {
            font.font_size = Some(size);
        }
        font.font_path = None; // raw_options.font;
        if let Some(font_color) = raw_options.text_color {
            font.color = font_color;
        }

        Self {
            background,
            text,
            font,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackgroundOptions {
    Url(String),
    RandomImage,
    Color(String), // "random" / "red", "green", "teal" / "rgb(20, 48, 200)" / "#FE7789"
}

/// Contains the options for the raw text to be displayed. Does
/// not include text styling. For styling, using [`FontOptions`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextOptions {
    Message(String),
    RandomQuote,
    RandomWord,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FontOptions {
    /// Relative path to the font
    pub font_path: Option<String>,
    pub color: String,
    /// Size of the font in pixels
    pub font_size: Option<u32>,
}
