use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FossApp {
    pub name: String,
    pub description: String,
    pub website: String,
    pub download_url: String,
    pub license: String,
    pub category: String,
    pub icon: String,
    pub winget_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FossAlternative {
    pub paid_software: String,
    pub alternatives: Vec<FossApp>,
}

/// Returns FOSS alternatives database
pub fn get_foss_database() -> HashMap<String, Vec<FossApp>> {
    let mut db: HashMap<String, Vec<FossApp>> = HashMap::new();

    // Video Editing
    db.insert(
        "Adobe Premiere Pro".to_string(),
        vec![
            FossApp {
                name: "DaVinci Resolve".to_string(),
                description:
                    "Professional video editing, color correction, and audio post-production"
                        .to_string(),
                website: "https://www.blackmagicdesign.com/products/davinciresolve".to_string(),
                download_url: "https://www.blackmagicdesign.com/products/davinciresolve"
                    .to_string(),
                license: "Freemium".to_string(),
                category: "Video Editing".to_string(),
                icon: "🎬".to_string(),
                winget_id: Some("BlackmagicDesign.DaVinciResolve".to_string()),
            },
            FossApp {
                name: "Kdenlive".to_string(),
                description: "Powerful multi-track video editor with a sleek interface".to_string(),
                website: "https://kdenlive.org".to_string(),
                download_url: "https://kdenlive.org/en/download/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Video Editing".to_string(),
                icon: "🎥".to_string(),
                winget_id: Some("KDE.Kdenlive".to_string()),
            },
            FossApp {
                name: "Shotcut".to_string(),
                description: "Cross-platform video editor with wide format support".to_string(),
                website: "https://shotcut.org".to_string(),
                download_url: "https://shotcut.org/download/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Video Editing".to_string(),
                icon: "📹".to_string(),
                winget_id: Some("Meltytech.Shotcut".to_string()),
            },
        ],
    );

    // Image Editing
    db.insert(
        "Adobe Photoshop".to_string(),
        vec![
            FossApp {
                name: "GIMP".to_string(),
                description: "Full-featured image editor with extensive plugin support".to_string(),
                website: "https://www.gimp.org".to_string(),
                download_url: "https://www.gimp.org/downloads/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Image Editing".to_string(),
                icon: "🖼️".to_string(),
                winget_id: Some("GIMP.GIMP".to_string()),
            },
            FossApp {
                name: "Krita".to_string(),
                description: "Digital painting and illustration focused application".to_string(),
                website: "https://krita.org".to_string(),
                download_url: "https://krita.org/en/download/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Image Editing".to_string(),
                icon: "🎨".to_string(),
                winget_id: Some("KDE.Krita".to_string()),
            },
        ],
    );

    // Office Suite
    db.insert(
        "Microsoft Office".to_string(),
        vec![
            FossApp {
                name: "LibreOffice".to_string(),
                description: "Complete office suite with Writer, Calc, Impress, and more"
                    .to_string(),
                website: "https://www.libreoffice.org".to_string(),
                download_url: "https://www.libreoffice.org/download/download/".to_string(),
                license: "MPL-2.0".to_string(),
                category: "Office".to_string(),
                icon: "📝".to_string(),
                winget_id: Some("TheDocumentFoundation.LibreOffice".to_string()),
            },
            FossApp {
                name: "OnlyOffice".to_string(),
                description: "Modern office suite with high MS Office compatibility".to_string(),
                website: "https://www.onlyoffice.com".to_string(),
                download_url: "https://www.onlyoffice.com/download-desktop.aspx".to_string(),
                license: "AGPL-3.0".to_string(),
                category: "Office".to_string(),
                icon: "📊".to_string(),
                winget_id: Some("ONLYOFFICE.DesktopEditors".to_string()),
            },
        ],
    );

    db.insert(
        "Microsoft 365".to_string(),
        db.get("Microsoft Office").unwrap().clone(),
    );

    // Archive Tools
    db.insert(
        "WinRAR".to_string(),
        vec![
            FossApp {
                name: "7-Zip".to_string(),
                description: "High compression ratio archiver with wide format support".to_string(),
                website: "https://www.7-zip.org".to_string(),
                download_url: "https://www.7-zip.org/download.html".to_string(),
                license: "LGPL-2.1".to_string(),
                category: "Utilities".to_string(),
                icon: "📦".to_string(),
                winget_id: Some("7zip.7zip".to_string()),
            },
            FossApp {
                name: "PeaZip".to_string(),
                description: "Feature-rich archive manager with strong encryption".to_string(),
                website: "https://peazip.github.io".to_string(),
                download_url: "https://peazip.github.io/peazip-64bit.html".to_string(),
                license: "LGPL-3.0".to_string(),
                category: "Utilities".to_string(),
                icon: "🗜️".to_string(),
                winget_id: Some("Giorgiotani.Peazip".to_string()),
            },
        ],
    );

    // Code Editors
    db.insert(
        "Sublime Text".to_string(),
        vec![
            FossApp {
                name: "VS Code".to_string(),
                description: "Powerful code editor with extensive extension ecosystem".to_string(),
                website: "https://code.visualstudio.com".to_string(),
                download_url: "https://code.visualstudio.com/Download".to_string(),
                license: "MIT".to_string(),
                category: "Development".to_string(),
                icon: "💻".to_string(),
                winget_id: Some("Microsoft.VisualStudioCode".to_string()),
            },
            FossApp {
                name: "Notepad++".to_string(),
                description: "Lightweight, fast text and code editor for Windows".to_string(),
                website: "https://notepad-plus-plus.org".to_string(),
                download_url: "https://notepad-plus-plus.org/downloads/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Development".to_string(),
                icon: "📄".to_string(),
                winget_id: Some("Notepad++.Notepad++".to_string()),
            },
        ],
    );

    // Vector Graphics
    db.insert(
        "Adobe Illustrator".to_string(),
        vec![FossApp {
            name: "Inkscape".to_string(),
            description: "Professional vector graphics editor".to_string(),
            website: "https://inkscape.org".to_string(),
            download_url: "https://inkscape.org/release/".to_string(),
            license: "GPL-3.0".to_string(),
            category: "Design".to_string(),
            icon: "✏️".to_string(),
            winget_id: Some("Inkscape.Inkscape".to_string()),
        }],
    );

    // Media Players
    db.insert(
        "iTunes".to_string(),
        vec![FossApp {
            name: "VLC Media Player".to_string(),
            description: "Plays virtually any media format without codecs".to_string(),
            website: "https://www.videolan.org".to_string(),
            download_url: "https://www.videolan.org/vlc/".to_string(),
            license: "GPL-2.0".to_string(),
            category: "Media".to_string(),
            icon: "🎵".to_string(),
            winget_id: Some("VideoLAN.VLC".to_string()),
        }],
    );

    // 3D Modeling
    db.insert(
        "3ds Max".to_string(),
        vec![FossApp {
            name: "Blender".to_string(),
            description: "Complete 3D creation suite with modeling, animation, and rendering"
                .to_string(),
            website: "https://www.blender.org".to_string(),
            download_url: "https://www.blender.org/download/".to_string(),
            license: "GPL-3.0".to_string(),
            category: "3D".to_string(),
            icon: "🧊".to_string(),
            winget_id: Some("BlenderFoundation.Blender".to_string()),
        }],
    );

    db.insert("Maya".to_string(), db.get("3ds Max").unwrap().clone());
    db.insert("Cinema 4D".to_string(), db.get("3ds Max").unwrap().clone());

    // Password Managers
    db.insert(
        "LastPass".to_string(),
        vec![
            FossApp {
                name: "Bitwarden".to_string(),
                description: "Secure, open-source password manager".to_string(),
                website: "https://bitwarden.com".to_string(),
                download_url: "https://bitwarden.com/download/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Security".to_string(),
                icon: "🔐".to_string(),
                winget_id: Some("Bitwarden.Bitwarden".to_string()),
            },
            FossApp {
                name: "KeePassXC".to_string(),
                description: "Cross-platform password manager with local storage".to_string(),
                website: "https://keepassxc.org".to_string(),
                download_url: "https://keepassxc.org/download/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Security".to_string(),
                icon: "🔑".to_string(),
                winget_id: Some("KeePassXCTeam.KeePassXC".to_string()),
            },
        ],
    );

    db.insert("1Password".to_string(), db.get("LastPass").unwrap().clone());

    // Audio Editing
    db.insert(
        "Adobe Audition".to_string(),
        vec![FossApp {
            name: "Audacity".to_string(),
            description: "Multi-track audio editor and recorder".to_string(),
            website: "https://www.audacityteam.org".to_string(),
            download_url: "https://www.audacityteam.org/download/".to_string(),
            license: "GPL-3.0".to_string(),
            category: "Audio".to_string(),
            icon: "🎙️".to_string(),
            winget_id: Some("Audacity.Audacity".to_string()),
        }],
    );

    // Screen Capture
    db.insert(
        "Snagit".to_string(),
        vec![
            FossApp {
                name: "ShareX".to_string(),
                description: "Powerful screenshot and screen recording tool".to_string(),
                website: "https://getsharex.com".to_string(),
                download_url: "https://getsharex.com/downloads".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Utilities".to_string(),
                icon: "📸".to_string(),
                winget_id: Some("ShareX.ShareX".to_string()),
            },
            FossApp {
                name: "Greenshot".to_string(),
                description: "Light-weight screenshot tool with annotation".to_string(),
                website: "https://getgreenshot.org".to_string(),
                download_url: "https://getgreenshot.org/downloads/".to_string(),
                license: "GPL-3.0".to_string(),
                category: "Utilities".to_string(),
                icon: "🖼️".to_string(),
                winget_id: Some("Greenshot.Greenshot".to_string()),
            },
        ],
    );

    db
}

/// Get all FOSS apps for browsing
pub fn get_all_foss_apps() -> Vec<FossApp> {
    let db = get_foss_database();
    let mut all_apps: Vec<FossApp> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for (_, apps) in db.iter() {
        for app in apps {
            if !seen.contains(&app.name) {
                seen.insert(app.name.clone());
                all_apps.push(app.clone());
            }
        }
    }

    all_apps.sort_by(|a, b| a.name.cmp(&b.name));
    all_apps
}

/// Find FOSS alternatives for a given software name
pub fn find_alternatives(software_name: &str) -> Option<Vec<FossApp>> {
    let db = get_foss_database();
    let software_lower = software_name.to_lowercase();

    // Skip software that's already FOSS or shouldn't be matched
    let foss_software = [
        "git",
        "python",
        "node",
        "rust",
        "go",
        "java",
        "ruby",
        "perl",
        "firefox",
        "vlc",
        "gimp",
        "blender",
        "inkscape",
        "libreoffice",
        "7-zip",
        "notepad++",
        "vs code",
        "visual studio code",
        "audacity",
        "obs",
        "handbrake",
        "filezilla",
        "putty",
        "krita",
        "kdenlive",
        "bitwarden",
        "keepass",
        "thunderbird",
        "brave",
        "chromium",
    ];

    for foss in foss_software.iter() {
        if software_lower.contains(foss) || foss.contains(&software_lower) {
            return None;
        }
    }

    // Exact match first
    if let Some(alts) = db.get(software_name) {
        return Some(alts.clone());
    }

    // Fuzzy match - only if both strings are long enough to avoid false positives
    // Require the search term to be at least 5 chars and match at word boundaries
    if software_lower.len() >= 5 {
        for (key, alts) in db.iter() {
            let key_lower = key.to_lowercase();

            // Check if the software name starts with or ends with the key (word boundary match)
            if software_lower.starts_with(&key_lower)
                || software_lower.ends_with(&key_lower)
                || key_lower.starts_with(&software_lower)
                || key_lower.ends_with(&software_lower)
            {
                return Some(alts.clone());
            }

            // Check for word-based containment (e.g., "Adobe Photoshop CC" matches "Adobe Photoshop")
            let software_words: Vec<&str> = software_lower.split_whitespace().collect();
            let key_words: Vec<&str> = key_lower.split_whitespace().collect();

            // If key words are a subset of software words, it's a match
            if key_words
                .iter()
                .all(|kw| software_words.iter().any(|sw| sw.contains(kw)))
            {
                return Some(alts.clone());
            }
        }
    }

    None
}
