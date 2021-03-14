use crate::{
    utils::{add_and_commit, format_sha256},
    ManifestUpdater, Version, VersionManifest,
};
use anyhow::Result;
use git2::Repository;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

pub struct Sonarr;

impl ManifestUpdater for Sonarr {
    fn update(&self, manifest: &mut VersionManifest, dir: &Path, repo: &Repository) -> Result<()> {
        let releases: HashMap<String, Release> =
            ureq::get("https://services.sonarr.tv/v1/releases")
                .call()?
                .into_json()?;

        for branch in &["v3-stable", "v3-nightly", "v3-preview"] {
            let release = releases.get(*branch).unwrap();

            if manifest
                .sonarr
                .get(*branch)
                .map_or(true, |v| release.version != v.version)
            {
                manifest.sonarr.insert(
                    branch.to_string(),
                    Version {
                        version: release.version.clone(),
                        url: release.linux.manual.url.clone(),
                        sha256: format_sha256(&hex::decode(&release.linux.manual.hash)?),
                        branch: Some(release.branch.clone()),
                    },
                );

                fs::write(dir.join("versions.json"), serde_json::to_vec(&manifest)?)?;

                add_and_commit(
                    repo,
                    &format!("sonarr: update {} to {}", branch, release.version),
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct Release {
    version: String,
    branch: String,
    linux: Artifacts,
}

#[derive(Deserialize)]
struct Artifacts {
    manual: Artifact,
}

#[derive(Deserialize)]
struct Artifact {
    url: String,
    hash: String,
}
