use crate::foss_db::{self, FossApp};
use crate::registry::{self, InstalledSoftware};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareWithAlternatives {
    pub software: InstalledSoftware,
    pub has_alternatives: bool,
    pub alternatives: Vec<FossApp>,
}

/// Get all installed software with FOSS alternative info
#[tauri::command]
pub fn get_installed_software() -> Vec<SoftwareWithAlternatives> {
    let software_list = registry::scan_installed_software();

    software_list
        .into_iter()
        .map(|software| {
            let alternatives = foss_db::find_alternatives(&software.name).unwrap_or_default();
            SoftwareWithAlternatives {
                has_alternatives: !alternatives.is_empty(),
                software,
                alternatives,
            }
        })
        .collect()
}

/// Get FOSS alternatives for a specific software
#[tauri::command]
pub fn get_foss_alternatives(software_name: String) -> Vec<FossApp> {
    foss_db::find_alternatives(&software_name).unwrap_or_default()
}

/// Get all available FOSS apps
#[tauri::command]
pub fn get_all_foss_apps() -> Vec<FossApp> {
    foss_db::get_all_foss_apps()
}

/// Uninstall software using its built-in uninstaller
#[tauri::command]
pub async fn uninstall_software(uninstall_string: String) -> Result<String, String> {
    if uninstall_string.is_empty() {
        return Err("No uninstall command available for this software".to_string());
    }

    let uninstall_str = uninstall_string.trim();

    // Special handling for MsiExec commands
    if uninstall_str.to_lowercase().contains("msiexec") {
        return run_msiexec_uninstall(uninstall_str);
    }

    // Parse the uninstall command
    let (program, args) = parse_uninstall_command(uninstall_str);

    // Check if the program exists
    let program_path = std::path::Path::new(&program);
    if !program_path.exists() && !program.to_lowercase().ends_with(".exe") {
        // Try with .exe extension
        let with_exe = format!("{}.exe", program);
        if std::path::Path::new(&with_exe).exists() {
            return run_uninstaller(&with_exe, &args);
        }
    }

    run_uninstaller(&program, &args)
}

fn run_uninstaller(program: &str, args: &[String]) -> Result<String, String> {
    // First try without elevation
    match Command::new(program).args(args).spawn() {
        Ok(_) => Ok("Uninstaller launched successfully".to_string()),
        Err(e) => {
            // If we get "requires elevation" error (740), try with runas
            if e.raw_os_error() == Some(740) {
                return run_elevated(program, args);
            }
            if e.kind() == std::io::ErrorKind::NotFound {
                Err(format!(
                    "Uninstaller not found: {}. The software may have been partially removed.",
                    program
                ))
            } else {
                Err(format!("Failed to launch uninstaller: {}", e))
            }
        }
    }
}

fn run_elevated(program: &str, args: &[String]) -> Result<String, String> {
    // Use PowerShell Start-Process with -Verb RunAs for elevation
    let args_str = args.join(" ");
    let ps_command = format!(
        "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
        program.replace("'", "''"),
        args_str.replace("'", "''")
    );

    match Command::new("powershell")
        .args(["-Command", &ps_command])
        .spawn()
    {
        Ok(_) => Ok("Uninstaller launched with administrator privileges".to_string()),
        Err(e) => Err(format!(
            "Failed to launch uninstaller with elevation: {}",
            e
        )),
    }
}

fn run_msiexec_uninstall(cmd: &str) -> Result<String, String> {
    // Extract the product code or path from the MsiExec command
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    // Try normal first, then elevated if needed
    match Command::new("msiexec").args(&args).spawn() {
        Ok(_) => Ok("Windows Installer launched".to_string()),
        Err(e) => {
            if e.raw_os_error() == Some(740) {
                run_elevated("msiexec", &args)
            } else {
                Err(format!("Failed to launch Windows Installer: {}", e))
            }
        }
    }
}

/// Download/open FOSS app website
#[tauri::command]
pub fn download_foss_app(url: String) -> Result<String, String> {
    // Opens the download URL in the default browser
    // The actual download is handled by the browser
    match open::that(&url) {
        Ok(_) => Ok("Opening download page...".to_string()),
        Err(e) => Err(format!("Failed to open URL: {}", e)),
    }
}

/// Winget package info returned from search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WingetPackage {
    pub id: String,
    pub name: String,
    pub version: String,
}

/// Check if winget is available on the system
#[tauri::command]
pub fn check_winget_available() -> bool {
    Command::new("winget")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Search for packages in winget
#[tauri::command]
pub async fn search_winget(query: String) -> Result<Vec<WingetPackage>, String> {
    let output = Command::new("winget")
        .args(["search", &query, "--accept-source-agreements"])
        .output()
        .map_err(|e| format!("Failed to run winget search: {}", e))?;

    if !output.status.success() {
        return Err("Winget search failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages = parse_winget_output(&stdout);
    Ok(packages)
}

/// Parse winget search output into structured data
fn parse_winget_output(output: &str) -> Vec<WingetPackage> {
    let mut packages = Vec::new();
    let lines: Vec<&str> = output.lines().collect();

    // Find the header line to determine column positions
    let header_idx = lines
        .iter()
        .position(|line| line.contains("Name") && line.contains("Id"));
    if header_idx.is_none() {
        return packages;
    }

    let header_idx = header_idx.unwrap();
    let header = lines[header_idx];

    // Find column positions from header
    let id_pos = header.find("Id").unwrap_or(0);
    let version_pos = header.find("Version").unwrap_or(header.len());

    // Skip header and separator line
    for line in lines.iter().skip(header_idx + 2) {
        if line.trim().is_empty() {
            continue;
        }

        // Parse fixed-width columns
        let name = line.get(0..id_pos).unwrap_or("").trim().to_string();
        let id = line
            .get(id_pos..version_pos)
            .unwrap_or("")
            .trim()
            .to_string();
        let version = line.get(version_pos..).unwrap_or("").trim().to_string();

        if !id.is_empty() && !name.is_empty() {
            packages.push(WingetPackage {
                id,
                name,
                version: version.split_whitespace().next().unwrap_or("").to_string(),
            });
        }
    }

    packages
}

/// Install a package using winget
#[tauri::command]
pub async fn install_winget(package_id: String) -> Result<String, String> {
    // Use PowerShell to run winget with elevation if needed
    let ps_command = format!(
        "Start-Process -FilePath 'winget' -ArgumentList 'install --id {} --accept-package-agreements --accept-source-agreements' -Verb RunAs -Wait",
        package_id.replace("'", "''")
    );

    match Command::new("powershell")
        .args(["-Command", &ps_command])
        .spawn()
    {
        Ok(_) => Ok(format!("Installing {}...", package_id)),
        Err(e) => Err(format!("Failed to start installation: {}", e)),
    }
}

// ============================================
// Winget.run API Integration
// ============================================

/// Package info from winget.run API for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPackage {
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub description: String,
    pub license: String,
    pub tags: Vec<String>,
    pub latest_version: String,
}

/// Raw API response structures
#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "Packages")]
    packages: Vec<RawPackage>,
    #[serde(rename = "Total")]
    total: u32,
}

#[derive(Debug, Deserialize)]
struct RawPackage {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Latest")]
    latest: Option<PackageLatest>,
    #[serde(rename = "Versions")]
    versions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct PackageLatest {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Publisher")]
    publisher: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "License")]
    license: Option<String>,
    #[serde(rename = "Tags")]
    tags: Option<Vec<String>>,
}

/// Fetch packages from winget.run API
#[tauri::command]
pub async fn fetch_winget_api(
    page: Option<u32>,
    per_page: Option<u32>,
) -> Result<(Vec<ApiPackage>, u32), String> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(50);

    let url = format!(
        "https://api.winget.run/v2/packages?page={}&take={}",
        page, per_page
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch packages: {}", e))?;

    let api_response: ApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    let packages: Vec<ApiPackage> = api_response
        .packages
        .into_iter()
        .filter_map(|pkg| {
            let latest = pkg.latest?;
            Some(ApiPackage {
                id: pkg.id,
                name: latest.name.unwrap_or_default(),
                publisher: latest.publisher.unwrap_or_default(),
                description: latest.description.unwrap_or_default(),
                license: latest.license.unwrap_or_else(|| "Unknown".to_string()),
                tags: latest.tags.unwrap_or_default(),
                latest_version: pkg
                    .versions
                    .and_then(|v| v.first().cloned())
                    .unwrap_or_default(),
            })
        })
        .filter(|pkg| !pkg.name.is_empty())
        .collect();

    Ok((packages, api_response.total))
}

/// Search packages from winget.run API
#[tauri::command]
pub async fn search_winget_api(query: String) -> Result<Vec<ApiPackage>, String> {
    let url = format!(
        "https://api.winget.run/v2/packages?query={}",
        urlencoding::encode(&query)
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to search packages: {}", e))?;

    let api_response: ApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    let packages: Vec<ApiPackage> = api_response
        .packages
        .into_iter()
        .filter_map(|pkg| {
            let latest = pkg.latest?;
            Some(ApiPackage {
                id: pkg.id,
                name: latest.name.unwrap_or_default(),
                publisher: latest.publisher.unwrap_or_default(),
                description: latest.description.unwrap_or_default(),
                license: latest.license.unwrap_or_else(|| "Unknown".to_string()),
                tags: latest.tags.unwrap_or_default(),
                latest_version: pkg
                    .versions
                    .and_then(|v| v.first().cloned())
                    .unwrap_or_default(),
            })
        })
        .filter(|pkg| !pkg.name.is_empty())
        .collect();

    Ok(packages)
}

fn parse_uninstall_command(cmd: &str) -> (String, Vec<String>) {
    let cmd = cmd.trim();

    // Handle quoted paths (e.g., "C:\Program Files\App\uninstall.exe" /silent)
    if cmd.starts_with('"') {
        if let Some(end_quote) = cmd[1..].find('"') {
            let program = cmd[1..end_quote + 1].to_string();
            let rest = &cmd[end_quote + 2..];
            let args: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
            return (program, args);
        }
    }

    // Handle paths with spaces but no quotes by looking for .exe
    if let Some(exe_pos) = cmd.to_lowercase().find(".exe") {
        let program = cmd[..exe_pos + 4].to_string();
        let rest = &cmd[exe_pos + 4..];
        let args: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
        return (program, args);
    }

    // Simple case: just split on whitespace
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return (cmd.to_string(), vec![]);
    }

    (
        parts[0].to_string(),
        parts[1..].iter().map(|s| s.to_string()).collect(),
    )
}
