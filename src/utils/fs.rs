use std::{
    fs,
    path::{Path, PathBuf},
};
use toml_edit::DocumentMut;
use walkdir::WalkDir;

pub fn copy_relevant_files(
    src_dir: &Path,
    dest_dir: &Path,
    feature: &str,
) -> anyhow::Result<Vec<String>> {
    let mut copied = Vec::new();
    let feature_file = match feature {
        "dio" => "dioxus",
        "lep" => "leptos",
        _ => feature,
    };

    for entry in WalkDir::new(src_dir).into_iter().flatten() {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            if [
                "common.rs",
                "config.rs",
                &format!("{feature_file}.rs"),
                "countries.rs",
                "chart.rs",
            ]
            .contains(&file_name.as_ref())
            {
                let dest_path = dest_dir.join(file_name.to_string());
                fs::copy(path, &dest_path)?;
                copied.push(file_name.to_string());
            }
        }
    }

    Ok(copied
        .into_iter()
        .map(|f| f.trim_end_matches(".rs").to_string())
        .collect())
}

pub fn update_lib_rs(lib_path: PathBuf, modules: &[String]) -> anyhow::Result<()> {
    let mut content = if lib_path.exists() {
        fs::read_to_string(&lib_path)?
    } else {
        String::new()
    };

    for module in modules {
        let mod_line = format!("pub mod {};", module);
        if !content.contains(&mod_line) {
            content.push_str(&format!("\n{mod_line}"));
        }
    }

    fs::write(lib_path, content)?;
    Ok(())
}

pub fn update_cargo_toml(from: PathBuf, to: &Path, _feature: &str) -> anyhow::Result<()> {
    let source_content = fs::read_to_string(&from)?;
    let from_doc: DocumentMut = source_content.parse()?;

    let mut dest_doc = if to.exists() {
        fs::read_to_string(to)?.parse()?
    } else {
        DocumentMut::new()
    };

    let mut gated_deps = std::collections::HashSet::new();
    if let Some(features) = from_doc.get("features") {
        if let Some(features_table) = features.as_table() {
            for (_feature_name, deps_array) in features_table.iter() {
                if let Some(array) = deps_array.as_array() {
                    for item in array.iter().filter_map(|v| v.as_str()) {
                        if let Some(dep_name) = item.strip_prefix("dep:") {
                            gated_deps.insert(dep_name.to_string());
                        } else {
                            gated_deps.insert(item.to_string());
                        }
                    }
                }
            }
        }
    }

    if let Some(deps) = from_doc.get("dependencies") {
        let deps_table = deps.as_table().unwrap();

        for (name, val) in deps_table {
            let is_optional = val
                .get("optional")
                .and_then(|o| o.as_bool())
                .unwrap_or(false);

            if is_optional && gated_deps.contains(name) {
                continue;
            }

            dest_doc["dependencies"][name] = val.clone();
        }
    }

    fs::write(to, dest_doc.to_string())?;
    Ok(())
}
