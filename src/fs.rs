use std::{
    env::set_current_dir,
    fs::{create_dir_all, read_dir, remove_file},
};

pub fn cd(path: &str) {
    match set_current_dir(path) {
        Err(err) => panic!("can not move to {:?}\n{:?}", path, err),
        _ => {}
    };
}

pub fn clear_dir(path: &str) {
    for item in read_dir(path).unwrap() {
        let path = item.unwrap().path();
        match remove_file(&path) {
            Err(err) => panic!("can not clear {:?}\n{:?}", path, err),
            _ => {}
        };
    }
}

pub fn mk_dir(path: &str) {
    match create_dir_all(path) {
        Err(err) => panic!("can not create {:?}\n{:?}", path, err),
        _ => {}
    };
}
