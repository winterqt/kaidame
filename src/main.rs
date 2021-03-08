use git2::{
    build::RepoBuilder, Cred, FetchOptions, PushOptions, RemoteCallbacks, Repository, Signature,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env::args,
    fs,
    io::{self, Read},
    path::Path,
};
use tempfile::tempdir;

#[derive(Serialize, Deserialize)]
struct VersionManifest {
    sonarr: Version,
    radarr: HashMap<String, Version>,
    jellyfin: HashMap<String, Version>,
}

#[derive(Serialize, Deserialize)]
struct Version {
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    sha256: String,
}

#[derive(Deserialize)]
struct Change {
    version: String,
    url: String,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    prerelease: bool,
}

fn calculate_sha256(mut r: impl Read + Send) -> String {
    let mut s = Sha256::new();

    io::copy(&mut r, &mut s).unwrap();

    format!("sha256-{}", base64::encode(s.finalize()))
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

fn add_and_commit(repo: &Repository, msg: &str) -> Result<git2::Oid, git2::Error> {
    let mut index = repo.index()?;

    index.add_path(Path::new("versions.json"))?;

    index.write()?;

    let sig = Signature::now("kaidame-updates", "funny@dontemail.me")?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        msg,
        &repo.find_tree(index.write_tree()?)?,
        &[&repo.find_commit(repo.refname_to_id("HEAD")?)?],
    )
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

    let sonarr = ureq::get("https://services.sonarr.tv/v1/update/phantom-develop/changes")
        .query("os", "linux")
        .query("version", &manifest.sonarr.version)
        .call()
        .unwrap()
        .into_json::<Vec<Change>>()
        .unwrap()
        .remove(0);

    if sonarr.version != manifest.sonarr.version {
        manifest.sonarr = Version {
            version: sonarr.version.clone(),
            url: None,
            sha256: calculate_sha256(
                ureq::get(&sonarr.url)
                    .call()
                    .expect("failed to request tarball")
                    .into_reader(),
            ),
        };

        fs::write(
            dir.path().join("versions.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .expect("failed to write manifest");

        add_and_commit(&repo, &format!("sonarr: update to {}", sonarr.version))
            .expect("failed to commit");
    }

    for branch in &["master", "develop", "nightly"] {
        let latest = ureq::get(&format!(
            "https://radarr.servarr.com/v1/update/{}/changes",
            branch
        ))
        .query("os", "linux")
        .query("arch", "x64")
        .query("runtime", "netcore")
        .call()
        .unwrap()
        .into_json::<Vec<Change>>()
        .unwrap()
        .remove(0);

        if manifest
            .radarr
            .get(*branch)
            .map_or(true, |v| latest.version != v.version)
        {
            manifest.radarr.insert(
                (*branch).to_string(),
                Version {
                    version: latest.version.clone(),
                    url: Some(latest.url.clone()),
                    sha256: calculate_sha256(
                        ureq::get(&latest.url)
                            .call()
                            .expect("failed to request tarball")
                            .into_reader(),
                    ),
                },
            );

            fs::write(
                dir.path().join("versions.json"),
                serde_json::to_vec(&manifest).unwrap(),
            )
            .expect("failed to write manifest");

            add_and_commit(
                &repo,
                &format!("radarr-{}: update to {}", branch, latest.version),
            )
            .expect("failed to commit");
        }
    }

    let jellyfin_releases = ureq::get("https://api.github.com/repos/jellyfin/jellyfin/releases")
        .call()
        .unwrap()
        .into_json::<Vec<Release>>()
        .unwrap();

    let latest_release = jellyfin_releases.iter().find(|r| !r.prerelease).unwrap();
    let latest_prerelease = jellyfin_releases.iter().find(|r| r.prerelease).unwrap();

    for branch in &[(latest_release, "stable"), (latest_prerelease, "stable-rc")] {
        let ver = &branch.0.tag_name[1..].replace("-", "~");

        if manifest
            .jellyfin
            .get(branch.1)
            .map_or(true, |v| ver != &v.version)
        {
            let url = format!(
                "https://repo.jellyfin.org/releases/server/linux/{}/combined/jellyfin_{}.tar.gz",
                branch.1, ver
            );

            let sha256 = {
                let body = ureq::get(&(url.clone() + ".sha256sum"))
                    .call()
                    .expect("failed to req")
                    .into_string()
                    .unwrap();

                format!(
                    "sha256-{}",
                    base64::encode(hex::decode(body.split_whitespace().next().unwrap()).unwrap())
                )
            };

            manifest.jellyfin.insert(
                branch.1.to_string(),
                Version {
                    version: ver.to_string(),
                    url: Some(url),
                    sha256,
                },
            );

            fs::write(
                dir.path().join("versions.json"),
                serde_json::to_vec(&manifest).unwrap(),
            )
            .expect("failed to write manifest");

            add_and_commit(&repo, &format!("jellyfin-{}: update to {}", branch.1, ver))
                .expect("failed to commit");
        }
    }

    push(&repo, ssh_key_path).expect("failed to push repo");
}
