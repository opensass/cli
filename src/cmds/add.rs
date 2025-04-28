use crate::utils::fs::{copy_relevant_files, update_cargo_toml, update_pub_file};
use crates_io_api::SyncClient;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::{fs, io::Cursor, path::Path};
use tar::Archive;
use tempfile::tempdir;
use tracing::info;

pub fn run_add(crate_name: &str, feature: &str) -> anyhow::Result<()> {
    let client = SyncClient::new(
        "opensass (via crates_io_api)",
        std::time::Duration::from_millis(1000),
    )?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.cyan} {msg}")
            .expect("valid spinner"),
    );

    spinner.set_message(format!("🔍 Fetching crate: {crate_name}"));
    let crate_info = client.get_crate(crate_name)?;
    let latest_version = &crate_info.versions[0].num;
    spinner.set_message(format!("📦 Found latest version: {latest_version}"));

    let dir = tempdir()?;
    let url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        crate_name, latest_version
    );

    spinner.set_message("⬇️ Downloading crate...");
    let response = get(&url)?;
    let bytes = response.bytes()?;

    spinner.set_message("📦 Unpacking crate...");
    let tar = GzDecoder::new(Cursor::new(bytes));
    let mut archive = Archive::new(tar);
    archive.unpack(&dir)?;

    let extracted_path = fs::read_dir(&dir)?
        .filter_map(Result::ok)
        .find(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .ok_or_else(|| anyhow::anyhow!("Failed to locate unpacked crate folder"))?;

    let source_path = extracted_path.join("src");
    let current_project_src = Path::new("src");

    let new_crate_name = crate_name.replace('-', "_");

    spinner.set_message("📁 Copying relevant source files...");
    let _ = copy_relevant_files(&source_path, current_project_src, &new_crate_name, feature)?;

    spinner.set_message("🧩 Updating lib.rs...");
    update_pub_file(
        current_project_src.join("lib.rs"),
        &[new_crate_name.to_string()],
    )?;

    spinner.set_message("🛠️ Updating Cargo.toml...");
    update_cargo_toml(
        extracted_path.join("Cargo.toml"),
        Path::new("Cargo.toml"),
        feature,
    )?;

    spinner.finish_with_message("✅ Component added successfully!");

    info!(
        "Component `{}` with feature `{}` added to project",
        crate_name, feature
    );
    Ok(())
}
