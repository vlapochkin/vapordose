use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use reqwest::Client;

const GITHUB_RELEASE_URL: &str = "https://api.github.com/repos/acidicoala/SmokeAPI/releases/latest";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmokeApiVersionInfo {
    pub version: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct SmokeApiStatus {
    pub installed_version: String,
    pub latest_version: Option<String>,
    pub update_available: bool,
}

pub fn get_user_resources_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    Path::new(&home).join(".local/share/vapordose/resources")
}

pub fn get_installed_version() -> String {
    let version_file = get_user_resources_dir().join("version.json");
    if version_file.exists() {
        if let Ok(content) = fs::read_to_string(&version_file) {
            if let Ok(info) = serde_json::from_str::<SmokeApiVersionInfo>(&content) {
                return info.version;
            }
        }
    }
    // Fall back to the version shipped next to the binary (written by build_appimage.sh)
    let bundled = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("resources").join("version.txt")))
        .and_then(|p| fs::read_to_string(p).ok())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    match bundled {
        Some(v) => format!("{} (Bundled)", v),
        None => "unknown (Bundled)".to_string(),
    }
}

pub async fn check_for_updates() -> Result<SmokeApiStatus> {
    let current_version = get_installed_version();
    let client = Client::builder()
        .user_agent(concat!("VaporDose-SmokeAPI-Manager/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(8))
        .build()?;

    let resp = client.get(GITHUB_RELEASE_URL).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("GitHub API request failed with status: {}", resp.status());
    }

    let release_json: serde_json::Value = resp.json().await?;
    let latest_tag = release_json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let update_available = !latest_tag.is_empty() && !current_version.contains(&latest_tag);

    Ok(SmokeApiStatus {
        installed_version: current_version,
        latest_version: if latest_tag.is_empty() { None } else { Some(latest_tag) },
        update_available,
    })
}

pub async fn download_and_install_latest() -> Result<String> {
    let user_res_dir = get_user_resources_dir();
    fs::create_dir_all(&user_res_dir).context("Failed to create resources directory")?;

    let client = Client::builder()
        .user_agent(concat!("VaporDose-SmokeAPI-Manager/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let resp = client.get(GITHUB_RELEASE_URL).send().await?;
    let release_json: serde_json::Value = resp.json().await?;
    let tag_name = release_json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .context("Could not find tag_name in release")?
        .to_string();

    let assets = release_json
        .get("assets")
        .and_then(|a| a.as_array())
        .context("No assets found in release")?;

    let zip_asset = assets
        .iter()
        .find(|asset| {
            asset
                .get("name")
                .and_then(|n| n.as_str())
                .map(|name| name.ends_with(".zip"))
                .unwrap_or(false)
        })
        .context("No zip asset found in latest SmokeAPI release")?;

    let download_url = zip_asset
        .get("browser_download_url")
        .and_then(|u| u.as_str())
        .context("No browser_download_url found for asset")?;

    let bytes = client.get(download_url).send().await?.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor).context("Failed to parse downloaded zip file")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(user_res_dir.join(&outpath)).ok();
        } else {
            let file_name = outpath.file_name().unwrap_or_default().to_string_lossy();
            // Store target files directly in user_res_dir
            if file_name.contains("smoke_api") || file_name.contains("SmokeAPI") || file_name.ends_with(".so") || file_name.ends_with(".dll") || file_name.ends_with(".json") {
                let dest = user_res_dir.join(file_name.as_ref());
                let mut outfile = fs::File::create(&dest)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
    }

    // Write version info
    let info = SmokeApiVersionInfo {
        version: tag_name.clone(),
        updated_at: chrono_like_timestamp(),
    };
    let json_str = serde_json::to_string_pretty(&info)?;
    fs::write(user_res_dir.join("version.json"), json_str)?;

    Ok(tag_name)
}

fn chrono_like_timestamp() -> String {
    let now = std::time::SystemTime::now();
    format!("{:?}", now)
}
