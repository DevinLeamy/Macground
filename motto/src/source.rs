use image::Rgba;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use reqwest;

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
