use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Application {
    pub name: String,
    pub application_name: Option<String>,
    pub exec: String,
    pub icon: Option<String>,
    // pub icon_ascii: Option<String>,
    pub comment: Option<String>,
    pub categories: Vec<String>,
    pub terminal: bool,
    pub mime_types: Vec<String>,
}
impl Application {
    pub fn launch(&self) -> Result<(), std::io::Error> {
        // run the application using std::process::Command
        let exec_parts: Vec<&str> = self.exec.split_whitespace().collect();
        if exec_parts.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid exec command",
            ));
        }
        let command = exec_parts[0];
        let args = &exec_parts[1..];

        let mut cmd = std::process::Command::new(command);
        cmd.args(args);

        if self.terminal {
            // if terminal is true, run in a terminal emulator
            // use x-terminal-emulator if available, otherwise fallback to xterm
            let terminal_emulator =
                std::env::var("TERMINAL").unwrap_or_else(|_| "x-terminal-emulator".into());
            let mut terminal_cmd = std::process::Command::new(terminal_emulator);
            terminal_cmd.arg("-e").arg(command);
            for arg in args {
                terminal_cmd.arg(arg);
            }
            match terminal_cmd.spawn() {
                Ok(_) => Ok(()),
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to launch terminal: {}", e),
                    ));
                }
            }
        } else {
            match cmd.spawn() {
                Ok(_) => Ok(()),
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "Failed to launch application: {} \nExecutable Path: {}",
                            e, self.exec
                        ),
                    ));
                }
            }
        }
    }
}

pub fn find_desktop_files() -> Vec<Application> {
    let xdg = BaseDirectories::with_prefix("");
    let mut apps = Vec::new();

    // standard dirs: $XDG_DATA_HOME/applications, /usr/share/applications, ~/.local/share/applications
    for dir in xdg
        .get_data_dirs()
        .iter()
        .chain(std::iter::once(&xdg.get_data_home().unwrap()))
    // .chain(std::iter::once(&PathBuf::from("/usr/share/")))
    {
        let apps_dir = dir.join("applications");
        if apps_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(apps_dir) {
                for e in entries.flatten() {
                    // for each file in the directory
                    // i.e. /usr/share/applications/example.desktop
                    let p = e.path();
                    if p.extension().and_then(|s| s.to_str()) == Some("desktop") {
                        let app = parse_desktop_file(&p);
                        apps.push(app);
                    }
                }
            }
        }
    }
    apps
}

pub fn parse_desktop_file(path: &PathBuf) -> Application {
    // parse a .desktop file at path
    let content: String = fs::read_to_string(path).expect("Failed to read desktop file");

    // map to a hashmap of key-value pairs
    // only parse the [Desktop Entry] section for now
    // some desktop files have alternate sections like [Desktop Action ...]
    // we will ignore those for now
    let mut options: HashMap<String, String> = HashMap::new();
    for line in content.lines() {
        // ignore comments and empty lines
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        // only parse main section
        if line.starts_with('[') && line.ends_with(']') {
            if line != "[Desktop Entry]" {
                break; // only parse the main section
            } else {
                continue;
            }
        }
        let (k, v) = line.split_once('=').unwrap_or((line, ""));
        match k.trim() {
            // MimeType = <mime_type>;<mime_type>;...
            "MimeType" => {
                let types: Vec<String> = v.split(';').map(|s| s.trim().into()).collect();
                options.insert("MimeType".into(), types.join(";"));
                continue;
            }
            _ => {
                options.insert(k.trim().into(), v.trim().into());
            }
        }
    }

    let application_name: String = parse_executable_name(
        options
            .get("Exec")
            .cloned()
            .unwrap_or_else(|| "example-command".into())
            .as_str(),
    );
    let executable = parse_executable_args(
        options
            .get("Exec")
            .cloned()
            .unwrap_or_else(|| "example-command".into())
            .as_str(),
    );
    Application {
        name: options
            .get("Name")
            .cloned()
            .unwrap_or_else(|| "Unknown".into()),
        application_name: application_name.into(),
        exec: executable,
        icon: options.get("Icon").cloned(),
        comment: options.get("Comment").cloned(),
        categories: options
            .get("Categories")
            .map(|s| s.split(';').map(|s| s.trim().into()).collect())
            .unwrap_or_default(),
        terminal: options
            .get("Terminal")
            .map(|s| s == "true")
            .unwrap_or(false),
        mime_types: options
            .get("MimeType")
            .map(|s| s.split(';').map(|s| s.trim().into()).collect())
            .unwrap_or_default(),
    }
}

fn parse_executable_name(exec: &str) -> String {
    exec.split_whitespace().next().unwrap_or(exec).to_string()
}

fn parse_executable_args(exec: &str) -> String {
    // TODO! handle field codes like %U, %u, %F, %f, %i, %c, %k
    // for now, just return the command without field codes
    // this doesn't work for things like SQL lite browser
    let parts: Vec<&str> = exec.split_whitespace().collect();
    parts[0].to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_desktop_files() {
        let files: Vec<Application> = find_desktop_files();
        assert!(!files.is_empty());
        for f in files {
            println!("{:?}", f.name);
        }
    }
}
