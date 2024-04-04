use image::imageops::grayscale;
use image::GenericImageView;
use image::{io::Reader as ImageReader, DynamicImage};
use std::process::{Command, Output};

use crate::utils::config;

pub fn read(path: &str) -> Option<DynamicImage> {
    match ImageReader::open(path) {
        Ok(bytes) => match bytes.decode() {
            Ok(img) => Some(img),
            Err(err) => panic!("can not decode image\n{:?}", err),
        },
        Err(err) => panic!("can not read image \n{:?}", err),
    }
}

pub fn take() -> Result<Output, std::io::Error> {
    let cmd = format!(
        "mplayer tv:// -tv device=/dev/video{} -frames 1 -vo jpeg",
        config().camera,
    );

    Command::new("sh").arg("-c").arg(cmd).output()
}

pub fn analyze(img: &DynamicImage) -> u16 {
    let cfg = config();

    let img_dim = img.dimensions();
    let pixels_count = (img_dim.0 * img_dim.1) as f64;

    let mut brightness_sum = 0f64;

    for pixel in grayscale(img).to_vec() {
        brightness_sum += pixel as f64;
    }

    let max = cfg.max_brightness_level;
    let min = cfg.min_brightness_level;

    let mean = brightness_sum / pixels_count;

    let out = min as f64 + (mean / 255f64) * (max - min) as f64;

    out as u16
}
