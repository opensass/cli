use anyhow::{Context, Result};
use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
use toml_edit::DocumentMut;
use walkdir::WalkDir;

pub fn copy_relevant_files(
    src_dir: &Path,
    dest_dir: &Path,
    crate_name: &str,
    feature: &str,
    no_cum: bool,
) -> Result<Vec<String>> {
    let mut copied = Vec::new();

    let feature_file = match feature {
        "dio" => "dioxus",
        "lep" => "leptos",
        _ => feature,
    };

    let crate_dir = dest_dir.join(crate_name);
    fs::create_dir_all(&crate_dir)
        .with_context(|| format!("Failed to create crate directory at {:?}", crate_dir))?;

    for entry in WalkDir::new(src_dir).into_iter().flatten() {
        let path = entry.path();

        if path.is_file() {
            let relative_path = path.strip_prefix(src_dir)?;
            let components: Vec<_> = relative_path.components().collect();

            let is_in_feature_dir = components
                .first()
                .map(|c| c.as_os_str() == feature_file)
                .unwrap_or(false);

            let file_name = path.file_name().unwrap().to_string_lossy();

            let should_copy = [
                "common.rs",
                "config.rs",
                "countries.rs",
                "chart.rs",
                &format!("{feature_file}.rs"),
            ]
            .contains(&file_name.as_ref());

            if should_copy && components.len() == 1 {
                let dest_file_name = file_name.to_string();
                let dest_path = crate_dir.join(&dest_file_name);
                copy_file_cum(path, &dest_path, no_cum, crate_name, feature_file)?;
                copied.push(dest_file_name.trim_end_matches(".rs").to_string());
            } else if is_in_feature_dir && components.len() == 2 {
                copied.retain(|m| m != feature_file);
                let feature_rs_path = crate_dir.join(format!("{feature_file}.rs"));
                if feature_rs_path.exists() {
                    fs::remove_file(&feature_rs_path)
                        .with_context(|| format!("Failed to delete {:?}", feature_rs_path))?;
                }
                let inner_file_name = file_name.to_string();
                let dest_path = crate_dir.join(&inner_file_name);
                copy_file_cum(path, &dest_path, no_cum, crate_name, feature_file)?;
                copied.push(inner_file_name.trim_end_matches(".rs").to_string());
            }
        }
    }

    let crate_file = dest_dir.join(format!("{crate_name}.rs"));
    update_pub_file(crate_file, &copied)?;

    Ok(copied)
}

fn copy_file_cum(
    src: &Path,
    dest: &Path,
    no_cum: bool,
    crate_name: &str,
    feature_name: &str,
) -> Result<()> {
    if no_cum {
        let file = File::open(src)?;
        let reader = BufReader::new(file);
        let mut dest_file = File::create(dest)?;

        for line in reader.lines() {
            let mut line = line?;
            let trimmed = line.trim_start();

            if trimmed.starts_with("//")
                || trimmed.starts_with("///")
                || trimmed.starts_with("//!")
                || trimmed.starts_with("/*!")
                || trimmed.starts_with("/**")
            {
                continue;
            }

            if trimmed.starts_with("use crate::") {
                if let Some(stripped) = line.strip_prefix("use crate::") {
                    if stripped.starts_with(feature_name) {
                        line = format!(
                            "use crate::{}::{}",
                            crate_name,
                            &stripped[feature_name.len() + 2..]
                        );
                    } else {
                        line = format!("use crate::{}::{}", crate_name, stripped);
                    }
                }
            }

            writeln!(dest_file, "{}", line)?;
        }
    } else {
        fs::copy(src, dest).with_context(|| format!("Failed to copy {:?} to {:?}", src, dest))?;
    }

    Ok(())
}

pub fn update_pub_file(lib_path: PathBuf, modules: &[String]) -> Result<()> {
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

pub fn update_cargo_toml(from: PathBuf, to: &Path, _feature: &str) -> Result<()> {
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
