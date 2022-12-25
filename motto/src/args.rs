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
}

// save layouts.
#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub background: BackgroundOptions,
    pub text: TextOptions,
    // font: FontOptions,
}

impl Options {
    pub fn from(raw_options: RawOptions) -> Self {
        let mut background = BackgroundOptions::Color("random".to_string());

        if let Some(color) = raw_options.color {
            background = BackgroundOptions::Color(color);
        } else if raw_options.random_image {
            background = BackgroundOptions::RandomImage;
        } else if let Some(url) = raw_options.background_image {
            background = BackgroundOptions::Url(url);
        };

        let mut text = TextOptions::RandomWord;

        if let Some(message) = raw_options.message {
            text = TextOptions::Message(message);
        } else if raw_options.random_quote {
            text = TextOptions::RandomQuote;
        } else if raw_options.random_word {
            text = TextOptions::RandomWord;
        }

        Self { background, text }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackgroundOptions {
    Url(String),
    RandomImage,
    Color(String), // "random" / "red", "green", "teal" / "rgb(20, 48, 200)" / "#FE7789"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextOptions {
    Message(String),
    RandomQuote, // Requires an API key (currently)
    RandomWord,
}

// #[derive(Serialize, Deserialize, Default, Clone)]
// struct FontOptions {
//     font: String,
//     color: Rgba<u8>,
//     font_size: u32, // Represents a PxScale
// }
