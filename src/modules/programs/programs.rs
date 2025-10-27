use std::fs;
use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::eyre;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::{collections::HashMap, os::unix::process::CommandExt};
use xdg::BaseDirectories;

pub fn find_programs() -> Vec<Application> {
    let desktop_files = find_desktop_files();

    let directories = vec![PathBuf::from("/usr/bin")];
    // standard dirs: $XDG_DATA_HOME/applications, /usr/share/applications, ~/.local/share/applications
    let mut apps: Vec<Application> = vec![];
    for dir in directories
    // .chain(std::iter::once(&PathBuf::from("/usr/share/")))
    {
        if dir.is_dir()
            && let Ok(entries) = fs::read_dir(dir)
        {
            for e in entries.flatten() {
                // for each file in the directory
                // i.e. /usr/share/applications/example.desktop
                let p = e.path();

                let app = Application {
                    name: p
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .into(),
                    exec: p.to_str().unwrap_or_default().into(),
                    desktop_file_path: p.clone(),
                    icon: None,
                    comment: None,
                    categories: vec![],
                    terminal: true,
                    mime_types: vec![],
                };

                let mut found = false;
                for desktop_app in &desktop_files {
                    if desktop_app.exec.contains(&app.exec) {
                        found = true;
                    }
                    let truncated_name_1 = desktop_app.name.to_lowercase().replace(" ", "");
                    let truncated_name_2 = app
                        .name
                        .to_lowercase()
                        .replace(" ", "")
                        .replace("-", "")
                        .replace("_", "");

                    if truncated_name_1 == truncated_name_2 {
                        found = true;
                    }
                }
                if !found {
                    apps.push(app);
                }
            }
        }
    }
    apps
}

use nucleo::{Config, Matcher};

use crate::common::module_state::ScoredResult;
use crate::modules::applications::desktop::{Application, find_desktop_files};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_desktop_files() {
        let files: Vec<Application> = find_programs();
        assert!(!files.is_empty());
        for f in files {
            println!("{:?}", f.name);
        }
    }
}
