use clap::Parser;
use image::Rgba;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct RawOptions {
    #[arg(short, long)]
    pub background_image: Option<String>,
    #[arg(short, long)]
    pub random_image: bool,
    #[arg(short, long)]
    pub color: Option<String>,
    #[arg(short, long)]
    pub message: Option<String>,
    #[arg(short, long)]
    pub random_quote: bool,
    #[arg(short, long)]
    pub random_word: bool,
    #[arg(short, long)]
    pub text_color: Option<String>,
    #[arg(short, long)]
    pub text_size: Option<u32>,
    #[arg(short, long)]
    pub font: Option<String>,
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
            font: "font1.otf".to_string(),
            color: [255, 255, 255, 255],
            font_size: 200,
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
            font.font_size = size;
        }
        if let Some(font_name) = raw_options.font {
            font.font = font_name;
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
    RandomQuote, // Requires an API key (currently)
    RandomWord,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FontOptions {
    pub font: String,
    pub color: [u8; 4],
    pub font_size: u32, // Represents a PxScale
}
