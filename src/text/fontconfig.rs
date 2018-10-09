use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use xdg::BaseDirectories;

use xml::reader::{EventReader, XmlEvent};

use walkdir::WalkDir;

/// Locates fontconfig config
fn get_config() -> Option<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix("fontconfig").unwrap();
    xdg_dirs.find_config_file("fonts.conf").or_else(|| {
        let config = Path::new("/etc/fonts/fonts.conf");
        if config.exists() {
            Some(config.into())
        } else {
            None
        }
    })
}

fn parse_config(path: &Path) -> Vec<(Vec<String>, String)> {
    let config_file = File::open(path).unwrap();
    let parser = EventReader::new(config_file);
    let mut tracking_tags: Vec<String> = Vec::new();
    let mut xml_data: Vec<(Vec<String>, String)> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                tracking_tags.push(name.to_string());
            }
            Ok(XmlEvent::CData(data)) => {
                xml_data.push((tracking_tags.clone(), data));
            }
            Ok(XmlEvent::Characters(data)) => {
                xml_data.push((tracking_tags.clone(), data));
            }
            Ok(XmlEvent::EndElement { .. }) => {
                tracking_tags.pop();
            }
            Err(e) => panic!(e),
            _ => {}
        }
    }
    xml_data
}

/// Represents the main fontconfig config file
pub struct FontConfig {
    location: PathBuf,
    data: Vec<(Vec<String>, String)>,
}

impl FontConfig {
    /// Creates a new FontConfig object by looking for the fontconfig config file
    pub fn new() -> Result<FontConfig, ()> {
        let location = get_config().ok_or(())?;
        let data = parse_config(&location);
        Ok(FontConfig {
            location: location.to_path_buf(),
            data,
        })
    }

    /// Returns the location of the fontconfig config file being used
    pub fn get_location(&self) -> &Path {
        &self.location
    }

    /// Get the directories that contain fonts
    pub fn get_font_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        for entry in &self.data {
            if entry.0.last() == Some(&"dir".to_string()) {
                let path = PathBuf::from(entry.1.clone());
                if path.exists() {
                    dirs.push(path);
                }
            }
        }
        dirs
    }

    /// Return all fonts installed on the system
    pub fn get_fonts(&self) -> Result<Vec<PathBuf>, ::std::io::Error> {
        let mut fonts = Vec::new();
        for dir in self.get_font_dirs() {
            for file in WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|p| p.file_type().is_file())
            {
                let path = file.into_path();
                if let Some(extension) = path.extension() {
                    match extension.to_str() {
                        Some("ttf") | Some("otf") => fonts.push(path.clone()),
                        _ => {}
                    }
                }
            }
        }
        Ok(fonts)
    }

    /// Return all 'fonts.dir' files in font directories
    pub fn get_font_dir_files(&self) -> Result<Vec<PathBuf>, ::std::io::Error> {
        let mut fonts = Vec::new();
        for dir in self.get_font_dirs() {
            for file in WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|p| p.file_type().is_file())
            {
                let path = file.into_path();
                if let Some(file_name) = path.clone().file_name() {
                    if file_name.to_str() == Some("fonts.dir") {
                        fonts.push(path);
                    }
                }
            }
        }
        Ok(fonts)
    }

    /// Returns the paths of regular fonts belonging to a specific family installed on the system
    pub fn get_regular_family_fonts(&self, family: &str) -> Result<Vec<PathBuf>, ::std::io::Error> {
        let fonts_dir_files = self.get_font_dir_files()?;
        let mut fonts: Vec<PathBuf> = Vec::new();
        for dir in fonts_dir_files {
            let mut file = ::std::fs::File::open(dir.clone()).unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            for line in buf.lines().filter(|l| l.find("medium-r-normal").is_some()) {
                if let Some(split) = line.find(' ') {
                    let name = line[..split].to_string();
                    let settings = line[split..].to_string();
                    let mut char_buf = String::new();
                    for c in settings.chars() {
                        if c == ' ' || c == '-' {
                            char_buf.clear()
                        } else {
                            char_buf.push(c);
                            if char_buf == family {
                                let path = dir.with_file_name(name);
                                if !fonts.contains(&path) {
                                    fonts.push(path);
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(fonts)
    }
}
