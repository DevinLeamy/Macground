use std::collections::HashMap;

use image::Rgba;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use reqwest;
use serde::Deserialize;

use crate::BackgroundImage;

pub trait Source {
    fn get_background(&self) -> BackgroundImage;
}

pub struct ColorSource {
    width: u32,
    height: u32,
    color: Rgba<u8>,
}

impl ColorSource {
    pub fn new(width: u32, height: u32, color: Rgba<u8>) -> Self {
        Self {
            width,
            height,
            color,
        }
    }
    pub fn random(width: u32, height: u32) -> Self {
        ColorSource::new(width, height, random_color())
    }
}

impl Source for ColorSource {
    fn get_background(&self) -> BackgroundImage {
        BackgroundImage::new(self.width, self.height, &self.color)
    }
}

pub fn random_color() -> Rgba<u8> {
    let mut rng = thread_rng();
    let r = rng.sample(Uniform::new(0, 255));
    let g = rng.sample(Uniform::new(0, 255));
    let b = rng.sample(Uniform::new(0, 255));

    [r, g, b, 255].into()
}

#[derive(Default)]
pub struct ImageSource {
    image_url: String,
    width: u32,
    height: u32,
}

impl ImageSource {
    pub fn new(width: u32, height: u32, image_url: String) -> Self {
        Self {
            width,
            height,
            image_url,
        }
    }
}

impl Source for ImageSource {
    fn get_background(&self) -> BackgroundImage {
        let response = reqwest::blocking::get(&self.image_url).unwrap();
        let image = image::load_from_memory(&response.bytes().unwrap()).unwrap();
        let image = image.resize_to_fill(
            self.width,
            self.height,
            image::imageops::FilterType::Nearest,
        );
        let buffer = image.into_rgba8();

        BackgroundImage::from(buffer)
    }
}

pub trait TextSource {
    fn source_text(&self) -> Vec<String>;
}

#[derive(Deserialize)]
struct RandomWordResponse {
    word: String,
}

/// [TextSource] for generating a random word.
#[derive(Default)]
pub struct RandomWordSource;

impl TextSource for RandomWordSource {
    fn source_text(&self) -> Vec<String> {
        let url = "https://random-word-api.herokuapp.com/word";

        let response = reqwest::blocking::get(url)
            .unwrap()
            .json::<RandomWordResponse>()
            .unwrap();

        vec![response.word]
    }
}

#[derive(Deserialize)]
struct QuoteSourceResponse {
    quote: HashMap<String, String>,
}

/// [TextSource] for generating a random word.
#[derive(Default)]
pub struct QuoteSource;

impl TextSource for QuoteSource {
    /// Returns quote as ["<quote>", "<author>"]
    fn source_text(&self) -> Vec<String> {
        let url = "https://zenquotes.io?api=random";

        let response = reqwest::blocking::get(url)
            .unwrap()
            .json::<QuoteSourceResponse>()
            .unwrap();

        vec![
            response.quote.get(&"q".to_string()).unwrap().to_owned(),
            response.quote.get(&"a".to_string()).unwrap().to_owned(),
        ]
    }
}
