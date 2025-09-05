use std::{
    fs::{File, read_dir, remove_file},
    path::PathBuf,
};

use serde::{Serialize, de::DeserializeOwned};

pub trait StoredOnFileSystem {
    fn get_file_name(&self) -> PathBuf;
}

pub fn read_to_struct<T: DeserializeOwned>(file_name: &str) -> T {
    let target: T;
    {
        let file = File::options().read(true).open(file_name).unwrap();
        target = serde_yaml::from_reader(&file).unwrap();
    }
    target
}

pub fn save_to_new_file<T: Serialize + StoredOnFileSystem>(to_save: &T) {
    let file = File::create_new(to_save.get_file_name()).unwrap();
    let _ = serde_yaml::to_writer(file, &to_save);
}

pub fn update_file<T: Serialize + StoredOnFileSystem>(to_update: &T) {
    let file = File::options()
        .write(true)
        .truncate(true)
        .open(to_update.get_file_name())
        .unwrap();
    let _ = serde_yaml::to_writer(file, &to_update);
}

pub fn clear_project() -> Vec<PathBuf> {
    let members = read_dir(std::env::current_dir().unwrap()).unwrap();
    let mut removed_members: Vec<PathBuf> = vec![];
    for member in members {
        let _ = match member {
            Ok(inner) => {
                let path = inner.path();
                match path.extension() {
                    Some(extension) => {
                        if extension == "yaml" {
                            let _ = remove_file(&path);
                            removed_members.push(path);
                        }
                    }
                    None => (),
                }
            }
            Err(_) => todo!(),
        };
    }
    removed_members
}
