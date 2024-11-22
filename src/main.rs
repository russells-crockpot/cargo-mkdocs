use anyhow::{Error, Result};
mod cargo;
mod opts;
use cargo::{Metadata, PackageDb};
use opts::Opts;
use std::process::Command;

fn main() -> Result<()> {
    let opts = Opts::default();
    if !opts.manifest_file.exists() {
        return Err(Error::msg(format!(
            "Manifest file {} doesn't exist",
            opts.manifest_file.display()
        )));
    }
    let mut root_dir = opts.manifest_file.to_path_buf();
    root_dir.pop();
    if root_dir.parent().is_none() {
        root_dir = ".".into();
    }
    let metadata = Metadata::load(root_dir)?;
    let pkg_db = PackageDb::load(&opts.manifest_file, &metadata)?;
    let exclude = opts.exclude.into_iter().collect();
    let mut pkgs = pkg_db.dep_ids(&exclude);
    if opts.dev_dependencies {
        pkgs.extend(pkg_db.dev_dep_ids(&exclude));
    }
    if opts.build_dependencies {
        pkgs.extend(pkg_db.build_dep_ids(&exclude));
    }
    pkgs.extend(pkg_db.get_ids_for(&opts.include));
    if pkgs.is_empty() {
        println!("No packages to document.");
        return Ok(());
    }
    let mut command = Command::new("cargo");
    command.arg("doc").arg("--no-deps");
    let num_pkgs = pkgs.len();
    pkgs.into_iter().for_each(|p| {
        command.arg("-p").arg(p);
    });
    println!("Generating documentation");
    let output = command.output()?;
    if !output.status.success() {
        if let Ok(stderr_text) = String::from_utf8(output.stderr) {
            eprintln!("{}", stderr_text);
        }
        Err(Error::msg(
            "cargo doc command returned with status {output.status}",
        ))
    } else {
        println!("Generated docs for {num_pkgs} packages.");
        Ok(())
    }
}
