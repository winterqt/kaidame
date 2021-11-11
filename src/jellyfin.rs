use crate::{
    utils::{add_and_commit, format_sha256},
    ManifestUpdater, Version, VersionManifest,
};
use anyhow::Result;
use git2::Repository;
use serde::Deserialize;
use std::{fs, path::Path};

pub struct Jellyfin;

impl ManifestUpdater for Jellyfin {
    fn update(&self, manifest: &mut VersionManifest, dir: &Path, repo: &Repository) -> Result<()> {
        let releases: Vec<Release> =
            ureq::get("https://api.github.com/repos/jellyfin/jellyfin/releases")
                .call()?
                .into_json()?;

        let latest_release = releases.iter().find(|r| !r.prerelease).unwrap();
        let latest_prerelease = releases.iter().find(|r| r.prerelease).unwrap();

        for branch in &[(latest_release, "stable"), (latest_prerelease, "stable-rc")] {
            let ver = &branch.0.tag_name[1..].replace("-", "~");

            if manifest
                .jellyfin
                .get(branch.1)
                .map_or(true, |v| ver != v.version)
            {
                let url = format!(
                "https://repo.jellyfin.org/releases/server/portable/{}/combined/jellyfin_{}.tar.gz",
                branch.1, ver
            );

                let sha256 = {
                    let body = ureq::get(&(url.clone() + ".sha256sum"))
                        .call()?
                        .into_string()?;

                    format_sha256(&hex::decode(body.split_whitespace().next().unwrap())?)
                };

                manifest.jellyfin.insert(
                    branch.1.to_string(),
                    Version {
                        version: ver.to_string(),
                        url,
                        sha256,
                        branch: None,
                    },
                );

                fs::write(dir.join("versions.json"), serde_json::to_vec(&manifest)?)?;

                add_and_commit(repo, &format!("jellyfin: update {} to {}", branch.1, ver))?;
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    prerelease: bool,
}
