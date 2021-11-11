#![warn(clippy::pedantic)]

use anyhow::Result;
use git2::{build::RepoBuilder, Cred, FetchOptions, PushOptions, RemoteCallbacks, Repository};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::args, fs, path::Path};
use tempfile::tempdir;

mod jellyfin;
mod radarr;
mod sonarr;
mod utils;

use jellyfin::Jellyfin;
use radarr::Radarr;
use sonarr::Sonarr;

#[derive(Serialize, Deserialize)]
struct VersionManifest {
    sonarr: HashMap<String, Version>,
    radarr: HashMap<String, Version>,
    jellyfin: HashMap<String, Version>,
}

#[derive(Serialize, Deserialize)]
struct Version {
    version: String,
    url: String,
    sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
}

trait ManifestUpdater {
    fn update(&self, manifest: &mut VersionManifest, dir: &Path, repo: &Repository) -> Result<()>;
}

fn clone(url: &str, ssh_key_path: &Path, out_path: &Path) -> Result<Repository, git2::Error> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, ssh_key_path, None)
    });

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    RepoBuilder::new()
        .fetch_options(fetch_opts)
        .clone(url, out_path)
}

fn push(repo: &Repository, ssh_key_path: &Path) -> Result<(), git2::Error> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, ssh_key_path, None)
    });

    let mut push_opts = PushOptions::new();
    push_opts.remote_callbacks(callbacks);

    repo.find_remote("origin")
        .unwrap()
        .push(&["refs/heads/main"], Some(&mut push_opts))
}

fn main() {
    let ssh_key_path = args().nth(1).expect("can't get ssh key path");
    let ssh_key_path = Path::new(&ssh_key_path);

    let dir = tempdir().expect("failed to create temp dir");

    let repo = clone(
        "git@github.com:winterqt/kaidame.git",
        ssh_key_path,
        dir.path(),
    )
    .expect("failed to clone");

    let mut manifest: VersionManifest = serde_json::from_str(
        &fs::read_to_string(dir.path().join("versions.json")).expect("failed to read versions"),
    )
    .unwrap();

    let software: &[&dyn ManifestUpdater; 3] = &[&Jellyfin, &Sonarr, &Radarr];

    for software in software {
        software
            .update(&mut manifest, dir.path(), &repo)
            .expect("failed to update");
    }

    push(&repo, ssh_key_path).expect("failed to push repo");
}
