use serde_derive::{Deserialize, Serialize};

use crate::utils::get_system_max_brightness;

pub const APP_NAME: &str = "rab";
pub const MY_TMP: &str = "/tmp/rab/";
pub const DEFAULT_CONFIG_PATH: &str = "/etc/rab.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Check if the config is initialized or not
    global_initialized: bool,

    // Number of the camera device
    pub camera: u8, // default=0

    // Delay between each light check in seconds
    pub delay: u16, // default=60

    // To prevent eye damage, increases the backlight smoothly
    pub step: u8, // default=10

    //
    pub black_white_threshold: u8, // default=80

    // Control the max and min level of brightness that `rab` should set
    pub max_brightness_level: u16, // default=(max level of your monitor)
    pub min_brightness_level: u16, // default=0
}

static mut GLOBAL_CONFIG: Config = Config {
    global_initialized: false,
    camera: 0,
    delay: 4,
    step: 5,
    black_white_threshold: 150,
    max_brightness_level: 0,
    min_brightness_level: 0,
};

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            global_initialized: self.global_initialized.clone(),
            camera: self.camera.clone(),
            delay: self.delay.clone(),
            step: self.step.clone(),
            black_white_threshold: self.black_white_threshold.clone(),
            max_brightness_level: self.max_brightness_level.clone(),
            min_brightness_level: self.min_brightness_level.clone(),
        }
    }
}

// impl Copy for Config {}

impl Default for Config {
    fn default() -> Config {
        unsafe {
            if GLOBAL_CONFIG.global_initialized {
                GLOBAL_CONFIG.clone()
            } else {
                GLOBAL_CONFIG.max_brightness_level = get_system_max_brightness();
                GLOBAL_CONFIG.global_initialized = true;

                GLOBAL_CONFIG.clone()
            }
        }
    }
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self, confy::ConfyError> {
        confy::load(APP_NAME, path.unwrap_or(DEFAULT_CONFIG_PATH))
    }
}
