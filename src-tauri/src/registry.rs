use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSoftware {
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub install_date: String,
    pub install_location: String,
    pub uninstall_string: String,
    pub icon_path: String,
    pub estimated_size: u64,
}

pub fn scan_installed_software() -> Vec<InstalledSoftware> {
    let mut software_list: Vec<InstalledSoftware> = Vec::new();

    // Registry paths for installed software
    let paths = [
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_CURRENT_USER,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
    ];

    for (hkey, path) in paths.iter() {
        if let Ok(key) = RegKey::predef(*hkey).open_subkey(path) {
            for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
                if let Ok(subkey) = key.open_subkey(&subkey_name) {
                    if let Some(software) = parse_software_entry(&subkey) {
                        // Avoid duplicates
                        if !software_list
                            .iter()
                            .any(|s| s.name == software.name && s.version == software.version)
                        {
                            software_list.push(software);
                        }
                    }
                }
            }
        }
    }

    // Sort by name
    software_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    software_list
}

fn parse_software_entry(key: &RegKey) -> Option<InstalledSoftware> {
    let name: String = key.get_value("DisplayName").ok()?;

    // Skip system components and updates
    if name.is_empty() || name.starts_with("KB") {
        return None;
    }

    // Filter out common system components, drivers, and runtimes
    let skip_terms = [
        "Security Update",
        "Hotfix",
        "Microsoft Visual C++",
        "Redistributable",
        ".NET",
        "Framework",
        "Driver",
        "SDK",
        "Language Pack",
        "MUI",
        "Service",
        "Update", // Be careful with this one, but usually safe for "Update for..."
        "Runtime",
        "PhysX",
        "DirectX",
        "Vulkan",
        "WinRT",
        "Intellisense",
        "Application Verifier",
        "Certification Kit",
        "Development Kit",
        "vs_",
        "WinAppDeploy",
        "Universal CRT",
        "WinDirStat",
        "vcpp",
    ];

    if skip_terms.iter().any(|term| name.contains(term)) {
        return None;
    }

    // Try multiple version sources
    let version = get_version_string(key);
    let publisher: String = key.get_value("Publisher").unwrap_or_default();
    let install_date: String = key.get_value("InstallDate").unwrap_or_default();
    let install_location: String = key.get_value("InstallLocation").unwrap_or_default();
    let uninstall_string: String = key.get_value("UninstallString").unwrap_or_default();
    let icon_path: String = key.get_value("DisplayIcon").unwrap_or_default();
    let estimated_size: u64 = key.get_value::<u32, _>("EstimatedSize").unwrap_or(0) as u64;

    Some(InstalledSoftware {
        name,
        version,
        publisher,
        install_date,
        install_location,
        uninstall_string,
        icon_path,
        estimated_size,
    })
}

fn get_version_string(key: &RegKey) -> String {
    // Try DisplayVersion first
    if let Ok(version) = key.get_value::<String, _>("DisplayVersion") {
        let cleaned = sanitize_version(&version);
        if !cleaned.is_empty() {
            return cleaned;
        }
    }

    // Try Version as a string
    if let Ok(version) = key.get_value::<String, _>("Version") {
        let cleaned = sanitize_version(&version);
        if !cleaned.is_empty() {
            return cleaned;
        }
    }

    // Try to build from VersionMajor, VersionMinor, VersionBuild
    let major: u32 = key.get_value("VersionMajor").unwrap_or(0);
    let minor: u32 = key.get_value("VersionMinor").unwrap_or(0);

    if major > 0 || minor > 0 {
        return format!("{}.{}", major, minor);
    }

    // Try MajorVersion, MinorVersion (alternative naming)
    let major: u32 = key.get_value("MajorVersion").unwrap_or(0);
    let minor: u32 = key.get_value("MinorVersion").unwrap_or(0);

    if major > 0 || minor > 0 {
        return format!("{}.{}", major, minor);
    }

    String::new()
}

fn sanitize_version(version: &str) -> String {
    let trimmed = version.trim();

    // Check if it looks like a valid version (should contain at least one digit)
    if trimmed.chars().any(|c| c.is_ascii_digit()) {
        // Remove any leading/trailing non-version characters
        trimmed.to_string()
    } else {
        // Invalid version like "<3" or other garbage
        String::new()
    }
}
