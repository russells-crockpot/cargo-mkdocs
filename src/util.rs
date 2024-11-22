use anyhow::Result;
use cargo_lock::SourceId;
use cargo_toml::DependencyDetail;
use std::path::absolute;

#[allow(clippy::expect_fun_call)]
pub fn get_pkgid<S: AsRef<str>>(
    name: S,
    details: DependencyDetail,
    maybe_source_id: Option<SourceId>,
) -> Result<String> {
    let name = name.as_ref();
    let source_id = if let Some(source_id) = maybe_source_id {
        source_id
    } else {
        let path_str = details
            .path
            .expect(&format!("cannot determine source id of {name:}"));
        let path = absolute(path_str)?;
        SourceId::for_path(&path)?
    };
    todo!();
}
