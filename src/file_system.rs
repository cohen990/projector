use std::fs::{File, read_dir, remove_file};

use serde::{Serialize, de::DeserializeOwned};

pub fn read_to_struct<T: DeserializeOwned>(file_name: &str) -> T {
    let target: T;
    {
        let file = File::options().read(true).open(file_name).unwrap();
        target = serde_yaml::from_reader(&file).unwrap();
    }
    target
}

pub fn save_to_new_file<T: Serialize>(file_name: &str, serializable: T) {
    let file = File::create_new(file_name).unwrap();
    let _ = serde_yaml::to_writer(file, &serializable);
}

pub fn update_file<T: Serialize>(file_name: &str, serializable: T) {
    let file = File::options()
        .write(true)
        .truncate(true)
        .open(file_name)
        .unwrap();
    let _ = serde_yaml::to_writer(file, &serializable);
}

pub fn clear_project() {
    let members = read_dir(std::env::current_dir().unwrap()).unwrap();
    for member in members {
        let _ = match member {
            Ok(inner) => {
                let path = inner.path();
                match path.extension() {
                    Some(extension) => {
                        if extension == "yaml" {
                            let _ = remove_file(path);
                        }
                    }
                    None => (),
                }
            }
            Err(_) => todo!(),
        };
    }
}
