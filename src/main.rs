mod config;
mod fs;
mod image;
mod utils;

use std::thread::sleep;
use std::time::Duration;

use utils::set_brightness;

use crate::config::MY_TMP;
use crate::fs::{cd, clear_dir, mk_dir};
use crate::image as img;

fn setup_my_tmp() {
    // Setup my tmp working directory
    mk_dir(MY_TMP);

    // Move to working directory
    cd(MY_TMP);

    // Clear working directory
    clear_dir(MY_TMP);
}

fn main() {
    setup_my_tmp();

    loop {
        // Take a picture
        match img::take() {
            Err(err) => panic!("take image: error: {:?}", err),
            Ok(_) => {}
        }

        let img = img::read("00000001.jpg");
        clear_dir(MY_TMP);
        let brightness = img::analyze(&img.unwrap());

        set_brightness(brightness);

        sleep(Duration::from_secs(utils::config().delay as u64));
    }
}
