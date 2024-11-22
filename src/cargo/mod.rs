use anyhow::{Error, Result};
use cargo_toml::{DepsSet, Manifest};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

mod metadata;
pub use metadata::*;

pub struct PackageDb {
    pkg_ids: HashMap<String, String>,
    deps: Vec<String>,
    build_deps: Vec<String>,
    dev_deps: Vec<String>,
}

impl PackageDb {
    pub fn load<P: AsRef<Path>>(cargo_file_path: P, metadata: &Metadata) -> Result<Self> {
        let pkg_ids = metadata
            .packages
            .iter()
            .cloned()
            .map(|p| (p.name, p.id))
            .collect();
        if !cargo_file_path.as_ref().exists() {
            return Err(Error::msg(format!(
                "Manifest file {} doesn't exist",
                cargo_file_path.as_ref().display()
            )));
        }
        println!("Loading manifest {}", cargo_file_path.as_ref().display());
        let toml_content = fs::read_to_string(cargo_file_path)?;
        //let manifest = Manifest::from_path(cargo_file_path.as_ref())?;
        let manifest = Manifest::from_str(&toml_content)?;
        let mut deps = if let Some(ref workspace) = manifest.workspace {
            dep_ids_from_deps_set(&workspace.dependencies, &pkg_ids)
        } else {
            dep_ids_from_deps_set(&manifest.dependencies, &pkg_ids)
        };
        deps.extend(metadata.workspace_default_members.iter().cloned());
        let build_deps = if manifest.workspace.is_none() {
            dep_ids_from_deps_set(&manifest.build_dependencies, &pkg_ids)
        } else {
            Vec::with_capacity(0)
        };
        let dev_deps = if manifest.workspace.is_none() {
            dep_ids_from_deps_set(&manifest.dev_dependencies, &pkg_ids)
        } else {
            Vec::with_capacity(0)
        };
        Ok(Self {
            pkg_ids,
            deps,
            build_deps,
            dev_deps,
        })
    }

    pub fn get_ids_for<S, I>(&self, pkg_names: I) -> Vec<String>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        pkg_names
            .into_iter()
            .map(|n| {
                self.pkg_ids
                    .get(n.as_ref())
                    .cloned()
                    .unwrap_or_else(|| String::from(n.as_ref()))
            })
            .collect()
    }

    #[allow(clippy::iter_overeager_cloned)]
    pub fn dep_ids(&self, exclude: &HashSet<String>) -> Vec<String> {
        self.deps
            .iter()
            .cloned()
            .filter(|d| !exclude.contains(d))
            .collect()
    }

    #[allow(clippy::iter_overeager_cloned)]
    pub fn dev_dep_ids(&self, exclude: &HashSet<String>) -> Vec<String> {
        self.dev_deps
            .iter()
            .cloned()
            .filter(|d| !exclude.contains(d))
            .collect()
    }

    #[allow(clippy::iter_overeager_cloned)]
    pub fn build_dep_ids(&self, exclude: &HashSet<String>) -> Vec<String> {
        self.build_deps
            .iter()
            .cloned()
            .filter(|d| !exclude.contains(d))
            .collect()
    }
}

#[inline]
#[allow(clippy::expect_fun_call)]
fn dep_ids_from_deps_set(set: &DepsSet, pkg_ids: &HashMap<String, String>) -> Vec<String> {
    set.keys()
        .map(|n| {
            pkg_ids
                .get(n)
                .expect(&format!("Could not find package {n}"))
        })
        .map(Clone::clone)
        .collect()
}
