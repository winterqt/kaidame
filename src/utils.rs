use git2::{Repository, Signature};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Read},
    path::Path,
};

pub fn calculate_sha256(mut r: impl Read + Send) -> String {
    let mut s = Sha256::new();

    io::copy(&mut r, &mut s).unwrap();

    format!("sha256-{}", base64::encode(s.finalize()))
}

pub fn format_sha256(hash: &[u8]) -> String {
    format!("sha256-{}", base64::encode(hash))
}

pub fn add_and_commit(repo: &Repository, msg: &str) -> Result<git2::Oid, git2::Error> {
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
