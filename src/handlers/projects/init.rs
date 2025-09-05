use crate::{file_system::StoredOnFileSystem, git, projects::Project};

pub fn handle(project_name: String, remote_url: String) {
    git::init(remote_url.to_owned());
    let project = Project::save_new(project_name.to_owned());
    git::commit_and_push(
        &[project.get_file_name().as_path()],
        &format!("Initialising the project: {project_name} with {remote_url} backend"),
    );
}
