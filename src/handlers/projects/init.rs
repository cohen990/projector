use crate::projects::Project;

pub fn handle(project_name: String) {
    let _ = git2::Repository::init(std::env::current_dir().unwrap());
    Project::save_new(project_name);
}
