use std::{
    env,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use git2::{Cred, ObjectType, PushOptions, RemoteCallbacks, Signature, Time};

pub fn init(remote_url: String) {
    let repository = git2::Repository::init(std::env::current_dir().unwrap()).unwrap();
    repository.remote_set_url("origin", &remote_url).unwrap();
}

pub fn commit_and_push(paths: &[&Path], message: &str) {
    let repository = git2::Repository::open(std::env::current_dir().unwrap()).unwrap();
    for path in paths {
        repository.index().unwrap().add_path(path).unwrap();
    }
    repository.index().unwrap().write().unwrap();

    let signature = Signature::new(
        "projector",
        "projector",
        &Time::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            0,
        ),
    )
    .unwrap();
    let tree_oid = repository.index().unwrap().write_tree().unwrap();
    let tree = repository
        .find_object(tree_oid, Some(ObjectType::Tree))
        .unwrap()
        .into_tree()
        .unwrap();
    repository
        .commit(Some("HEAD"), &signature, &signature, message, &tree, &[])
        .unwrap();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, username, _| {
        Cred::ssh_key(
            username.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    repository
        .find_remote("origin")
        .unwrap()
        .push::<String>(&[], Some(&mut push_options))
        .unwrap();
}
