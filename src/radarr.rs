use crate::{
    utils::{add_and_commit, calculate_sha256},
    ManifestUpdater, Version, VersionManifest,
};
use anyhow::Result;
use git2::Repository;
use serde::Deserialize;
use std::{fs, path::Path};

pub struct Radarr;

impl ManifestUpdater for Radarr {
    fn update(&self, manifest: &mut VersionManifest, dir: &Path, repo: &Repository) -> Result<()> {
        for branch in &["master", "develop", "nightly"] {
            let latest = ureq::get(&format!(
                "https://radarr.servarr.com/v1/update/{}/changes",
                branch
            ))
            .query("os", "linux")
            .query("arch", "x64")
            .query("runtime", "netcore")
            .call()?
            .into_json::<Vec<Change>>()?
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
                        url: latest.url.clone(),
                        sha256: calculate_sha256(ureq::get(&latest.url).call()?.into_reader()),
                        branch: Some((*branch).to_string()),
                    },
                );

                fs::write(dir.join("versions.json"), serde_json::to_vec(&manifest)?)?;

                add_and_commit(
                    repo,
                    &format!("radarr: update {} to {}", branch, latest.version),
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct Change {
    version: String,
    url: String,
}
