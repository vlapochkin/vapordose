use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub const PROTON_OVERRIDE_STR: &str = r#"WINEDLLOVERRIDES=\"steam_api64=n,b;steam_api=n,b\""#;

/// Find all `localconfig.vdf` files in all Steam `userdata` subdirectories.
pub fn find_localconfig_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    let home = std::env::var("HOME").unwrap_or_default();
    let search_roots = vec![
        Path::new(&home).join(".local/share/Steam/userdata"),
        Path::new(&home).join(".steam/root/userdata"),
        Path::new(&home).join(".steam/steam/userdata"),
        Path::new(&home).join(".var/app/com.valvesoftware.Steam/.local/share/Steam/userdata"),
    ];

    for root in search_roots {
        if root.exists() {
            for entry in WalkDir::new(&root).max_depth(3).into_iter().flatten() {
                if entry.file_name() == "localconfig.vdf" {
                    let path = entry.path().to_path_buf();
                    if let Ok(real) = fs::canonicalize(&path) {
                        if !files.contains(&real) {
                            files.push(real);
                        }
                    } else if !files.contains(&path) {
                        files.push(path);
                    }
                }
            }
        }
    }
    files
}

/// Applies launch options for a specific game AppID across all localconfig.vdf files.
pub fn apply_launch_options(appid: &str, override_str: &str) -> Result<bool> {
    let files = find_localconfig_files();
    if files.is_empty() {
        return Ok(false);
    }

    let mut handled_any = false;
    for file_path in files {
        if let Ok(content) = fs::read_to_string(&file_path) {
            let new_content = inject_launch_option_into_vdf(&content, appid, override_str);
            if new_content != content {
                let backup_path = file_path.with_extension("vdf.bak");
                if !backup_path.exists() {
                    fs::copy(&file_path, &backup_path).ok();
                }
                fs::write(&file_path, new_content)
                    .with_context(|| format!("Failed to update localconfig.vdf at {:?}", file_path))?;
            }
            handled_any = true;
        }
    }
    Ok(handled_any)
}

/// Steam rewrites `localconfig.vdf` from memory when it exits, so edits made while
/// it is running are silently lost. Callers should warn the user in that case.
pub fn is_steam_running() -> bool {
    let Ok(entries) = fs::read_dir("/proc") else { return false };
    for entry in entries.flatten() {
        if let Ok(comm) = fs::read_to_string(entry.path().join("comm")) {
            let comm = comm.trim();
            if comm == "steam" || comm == "steamwebhelper" {
                return true;
            }
        }
    }
    false
}

pub fn apply_proton_launch_options(appid: &str) -> Result<bool> {
    apply_launch_options(appid, PROTON_OVERRIDE_STR)
}

/// Removes launch options for a specific game AppID across all localconfig.vdf files.
pub fn remove_launch_options(appid: &str) -> Result<bool> {
    let files = find_localconfig_files();
    if files.is_empty() {
        return Ok(false);
    }

    let mut updated_any = false;
    for file_path in files {
        if let Ok(content) = fs::read_to_string(&file_path) {
            let new_content = remove_launch_option_from_vdf(&content, appid, "");
            if new_content != content {
                fs::write(&file_path, new_content)
                    .with_context(|| format!("Failed to clean localconfig.vdf at {:?}", file_path))?;
                updated_any = true;
            }
        }
    }
    Ok(updated_any)
}

/// Finds the inner block range `(open_brace_pos + 1, close_brace_pos)` of an app block `"<appid>"\n{ ... }`
fn find_app_block_range(vdf_content: &str, appid: &str) -> Option<(usize, usize)> {
    let apps_pos = vdf_content.find("\"apps\"")?;
    let search_slice = &vdf_content[apps_pos..];
    let key = format!("\"{}\"", appid);
    let key_rel = search_slice.find(&key)?;
    let key_pos = apps_pos + key_rel;

    let after_key = &vdf_content[key_pos + key.len()..];
    let open_brace_rel = after_key.find('{')?;
    let open_brace_pos = key_pos + key.len() + open_brace_rel;

    let mut depth = 0;
    for (i, ch) in vdf_content[open_brace_pos..].char_indices() {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                return Some((open_brace_pos + 1, open_brace_pos + i));
            }
        }
    }
    None
}

/// Helper function: injects `override_str` into the `LaunchOptions` string of `appid`.
pub fn inject_launch_option_into_vdf(vdf_content: &str, appid: &str, override_str: &str) -> String {
    if let Some((start_inner, end_inner)) = find_app_block_range(vdf_content, appid) {
        let block_str = &vdf_content[start_inner..end_inner];
        if block_str.contains(override_str) {
            return vdf_content.to_string(); // Already injected
        }

        let launch_opt_regex = regex::Regex::new(r#""LaunchOptions"\s+"((?:[^"\\]|\\.)*)""#).unwrap();
        if let Some(opt_cap) = launch_opt_regex.captures(block_str) {
            let current_opts = opt_cap.get(1).unwrap().as_str();
            let new_opts = build_combined_launch_options(current_opts, override_str);
            let replacement_str = format!("\"LaunchOptions\"\t\t\"{}\"", new_opts);
            let updated_block = launch_opt_regex.replace(block_str, replacement_str.as_str()).to_string();
            let mut result = vdf_content.to_string();
            result.replace_range(start_inner..end_inner, &updated_block);
            return result;
        } else {
            let new_opt_line = format!("\n\t\t\t\t\t\"LaunchOptions\"\t\t\"{} %command%\"\n\t\t\t\t", override_str);
            let updated_block = format!("{}{}", block_str, new_opt_line);
            let mut result = vdf_content.to_string();
            result.replace_range(start_inner..end_inner, &updated_block);
            return result;
        }
    } else {
        // AppID block doesn't exist under "apps". Inject appid block inside "apps"
        if let Some(apps_pos) = vdf_content.find("\"apps\"") {
            if let Some(open_brace_rel) = vdf_content[apps_pos..].find('{') {
                let open_pos = apps_pos + open_brace_rel + 1;
                let new_app_block = format!(
                    "\n\t\t\t\t\"{}\"\n\t\t\t\t{{\n\t\t\t\t\t\"LaunchOptions\"\t\t\"{} %command%\"\n\t\t\t\t}}",
                    appid, override_str
                );
                let mut result = vdf_content.to_string();
                result.insert_str(open_pos, &new_app_block);
                return result;
            }
        }
    }

    vdf_content.to_string()
}

/// Helper function: removes `override_str` from `LaunchOptions` of `appid`.
pub fn remove_launch_option_from_vdf(vdf_content: &str, appid: &str, override_str: &str) -> String {
    if let Some((start_inner, end_inner)) = find_app_block_range(vdf_content, appid) {
        let block_str = &vdf_content[start_inner..end_inner];
        let launch_opt_regex = regex::Regex::new(r#""LaunchOptions"\s+"((?:[^"\\]|\\.)*)""#).unwrap();

        if let Some(opt_cap) = launch_opt_regex.captures(block_str) {
            let current_opts = opt_cap.get(1).unwrap().as_str();
            let mut cleaned_opts = current_opts
                .replace("WINEDLLOVERRIDES=\"steam_api64=n,b;steam_api=n,b\"", "")
                .replace("WINEDLLOVERRIDES=\\\"steam_api64=n,b;steam_api=n,b\\\"", "");

            if !override_str.is_empty() {
                cleaned_opts = cleaned_opts.replace(override_str, "");
            }

            // Remove any LD_PRELOAD lines
            let re_ld = regex::Regex::new(r#"LD_PRELOAD="[^"]*"\s*"#).unwrap();
            cleaned_opts = re_ld.replace_all(&cleaned_opts, "").to_string();

            let cleaned_trimmed = cleaned_opts.replace("%command%", "").trim().to_string();

            let replacement = if cleaned_trimmed.is_empty() {
                "".to_string()
            } else {
                let final_opts = if current_opts.contains("%command%") && !cleaned_trimmed.contains("%command%") {
                    format!("{} %command%", cleaned_trimmed)
                } else {
                    cleaned_trimmed
                };
                format!("\"LaunchOptions\"\t\t\"{}\"", final_opts)
            };

            let updated_block = launch_opt_regex.replace(block_str, replacement.as_str()).to_string();
            let mut result = vdf_content.to_string();
            result.replace_range(start_inner..end_inner, &updated_block);
            return result;
        }
    }

    vdf_content.to_string()
}

fn build_combined_launch_options(current: &str, override_str: &str) -> String {
    let trimmed = current.trim();
    if trimmed.is_empty() {
        return format!("{} %command%", override_str);
    }

    if trimmed.contains("%command%") {
        format!("{} {}", override_str, trimmed)
    } else {
        format!("{} {} %command%", override_str, trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_new_appid() {
        let sample_vdf = r#"
"UserLocalConfigStore"
{
	"Software"
	{
		"Valve"
		{
			"Steam"
			{
				"apps"
				{
				}
			}
		}
	}
}
"#;
        let res = inject_launch_option_into_vdf(sample_vdf, "594650", PROTON_OVERRIDE_STR);
        assert!(res.contains("\"594650\""));
        assert!(res.contains("WINEDLLOVERRIDES="));
        assert!(res.contains("%command%"));
    }

    #[test]
    fn test_inject_existing_appid_without_launch_options() {
        let sample_vdf = r#"
"UserLocalConfigStore"
{
	"Software"
	{
		"Valve"
		{
			"Steam"
			{
				"apps"
				{
					"594650"
					{
						"LastPlayed"		"1700000000"
					}
				}
			}
		}
	}
}
"#;
        let res = inject_launch_option_into_vdf(sample_vdf, "594650", PROTON_OVERRIDE_STR);
        assert!(res.contains("\"LaunchOptions\""));
        assert!(res.contains("WINEDLLOVERRIDES="));
    }

    #[test]
    fn test_remove_launch_options() {
        let sample_vdf = r#"
"UserLocalConfigStore"
{
	"Software"
	{
		"Valve"
		{
			"Steam"
			{
				"apps"
				{
					"594650"
					{
						"LaunchOptions"		"WINEDLLOVERRIDES=\"steam_api64=n,b;steam_api=n,b\" %command%"
					}
				}
			}
		}
	}
}
"#;
        let res = remove_launch_option_from_vdf(sample_vdf, "594650", PROTON_OVERRIDE_STR);
        assert!(!res.contains("WINEDLLOVERRIDES"));
    }
}
