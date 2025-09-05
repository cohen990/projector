use std::path::Path;

use crate::{file_system, git};

pub fn handle() {
    let removed_files = file_system::clear_project();
    let mapped: Vec<&Path> = removed_files.iter().map(|x| x.as_path()).collect();

    git::commit_and_push(&mapped, "Wiping the project");
}
