use anyhow::{Error, Result};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Package {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Metadata {
    pub packages: Vec<Package>,
    pub target_directory: PathBuf,
    pub workspace_root: PathBuf,
    pub workspace_members: Vec<String>,
    pub workspace_default_members: Vec<String>,
}

impl Metadata {
    //TODO allow feature selection
    pub fn load<P: AsRef<Path>>(root_dir: P) -> Result<Self> {
        println!(
            "Getting metadata for crate at {}",
            root_dir.as_ref().display()
        );
        let output = Command::new("cargo")
            .current_dir(fs::canonicalize(root_dir)?)
            .args(["metadata", "--all-features", "--format-version", "1"])
            .output()?;
        if !output.status.success() {
            if let Ok(stderr_text) = String::from_utf8(output.stderr) {
                eprintln!("{}", stderr_text);
            }
            Err(Error::msg(
                "cargo metadata command returned with status {output.status}",
            ))
        } else {
            let metadata = serde_json::from_slice(&output.stdout)?;
            Ok(metadata)
        }
    }
}
