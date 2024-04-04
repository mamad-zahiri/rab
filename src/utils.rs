use crate::config::Config;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn config() -> Config {
    Config::load(None).unwrap_or(Config::default())
}

pub fn get_system_max_brightness() -> u16 {
    let path = Path::new("/sys/class/backlight/intel_backlight/max_brightness");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("{:?}", err),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("{:?}", err),
        Ok(_) => data[..data.len() - 1].parse::<u16>().unwrap(),
    }
}

pub fn get_current_brightness() -> u16 {
    let path = Path::new("/sys/class/backlight/intel_backlight/brightness");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("{:?}", err),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("{:?}", err),
        Ok(_) => data[..data.len() - 1].parse::<u16>().unwrap(),
    }
}

pub fn set_brightness(value: u16) {
    let path = Path::new("/sys/class/backlight/intel_backlight/brightness");
    let max_br = config().max_brightness_level as i16;

    let mut cur_br = get_current_brightness() as i16;

    let step = if cur_br > value as i16 {
        config().step as i16 * -1i16
    } else {
        config().step as i16
    };

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => panic!("{:?}", err),
    };

    loop {
        cur_br += step;

        if cur_br < 0 || cur_br > max_br {
            break;
        }

        file.write_all(format!("{}\n", cur_br).as_bytes()).unwrap();

        if step < 0 {
            if cur_br <= value as i16 {
                break;
            }
        } else {
            if cur_br >= value as i16 {
                break;
            }
        }

        sleep(Duration::from_millis(10u64));
    }
}
