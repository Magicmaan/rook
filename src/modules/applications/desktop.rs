use std::fs;
use std::path::PathBuf;
use std::thread::sleep;
use std::{collections::HashMap, os::unix::process::CommandExt};
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
    pub desktop_file_path: Option<PathBuf>,
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
        let command = "gtk-launch";
        let executable = self
            .desktop_file_path
            .as_ref()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let args: &[&str; 1] = &[executable];

        let mut cmd = std::process::Command::new(command);
        cmd.args(args);

        match cmd.spawn() {
            Ok(mut child) => {
                let res = child.try_wait();
                if let Ok(Some(status)) = res {
                    log::info!(
                        "Launched application '{}' with exit code: {}",
                        self.name,
                        status.code().unwrap_or_default()
                    );
                } else {
                    log::info!("Launched application '{}'", self.name);
                }
                log::info!("Launched application command: {}", args.join(" "));
                sleep(std::time::Duration::from_millis(500)); // give some time for the process to start
                Ok(())
            }
            Err(e) => {
                Err(std::io::Error::other(
                    format!(
                        "Failed to launch application: {} \nExecutable Path: {}",
                        e, self.exec
                    ),
                ))
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
        if apps_dir.is_dir()
            && let Ok(entries) = fs::read_dir(apps_dir) {
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
        desktop_file_path: Some(path.clone()),
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

use nucleo::{Config, Matcher};

pub fn resolve_same_score(app_1: &Application, app_2: &Application, query: &str) -> i32 {
    let app_1_name = app_1.name.to_lowercase();
    let app_2_name = app_2.name.to_lowercase();

    let split_1 = app_1_name.split_whitespace().collect::<Vec<&str>>();
    let split_2 = app_2_name.split_whitespace().collect::<Vec<&str>>();
    let query_lower = query.to_lowercase();

    let app_1_exact = split_1.iter().any(|&s| s == query_lower);
    let app_2_exact = split_2.iter().any(|&s| s == query_lower);

    if app_1_exact && !app_2_exact {
        1
    } else if app_2_exact && !app_1_exact {
        -1
    } else {
        // neither or both are exact matches, prioritise shorter name
        if app_1_name.len() < app_2_name.len() {
            1
        } else if app_2_name.len() < app_1_name.len() {
            -1
        } else {
            0
        }
    }
}

pub fn sort_applications(apps: &mut Vec<Application>, query: &str) -> Vec<(u16, usize)> {
    // TODO: improve sorting algorithm
    // TODO: fuzzy search the application type, and mime types too
    //

    let mut matcher = Matcher::new(Config::DEFAULT);

    // Use a map score -> list of indices so we preserve all results
    let mut results: HashMap<u16, Vec<usize>> = HashMap::new();
    for (index, app) in apps.iter().enumerate() {
        // get score from fuzzy match
        if let Some(score) = matcher.fuzzy_match(
            nucleo::Utf32Str::new(&app.name.to_lowercase(), &mut Vec::new()),
            nucleo::Utf32Str::new(query, &mut Vec::new()),
        ) {
            if let std::collections::hash_map::Entry::Vacant(e) = results.entry(score) {
                // no collision, insert normally
                e.insert(vec![index]);
            } else {
                // Compare current app against existing entries in this score bucket.
                // If current clearly beats any existing entry, promote current to score+1.
                // If an existing clearly beats current, promote that existing to score+1.
                // If all comparisons are ties, keep both at the same score.

                // example
                // query = "zen"
                // results = { [88, zen browser], [88, zenity]}
                // in this case, zen browser should beat zenity, as it has a closer substring match
                // so we promote zen browser to 89, and keep zenity at 88
                //
                // users don't want a "maybe" from multiple results, they want the best match at the top
                // considering language, if i type zen, i want something with exactly "zen" in the name to be at the top
                // even if zenity has the same fuzzy score, it's not as good a match

                // this method still ensures normal matching
                // i.e. if type "browser", multiple browsers with same score will be kept at same score as they are all equally relevant

                let mut existing_beats_current = None;
                let mut current_beats_existing = false;

                // get colliding scores
                let bucket = results.get(&score).unwrap().clone();
                for &existing_index in bucket.iter() {
                    let res = resolve_same_score(&apps[existing_index], app, query);
                    if res > 0 {
                        // existing is better than current
                        existing_beats_current = Some(existing_index);
                        break;
                    } else if res < 0 {
                        // current is better than at least one existing
                        current_beats_existing = true;
                    }
                }

                if current_beats_existing {
                    // promote current to score + 1
                    results.entry(score + 1).or_default().push(index);
                } else if let Some(best_existing) = existing_beats_current {
                    // promote the existing winner to score + 1, keep current at this score
                    // remove best_existing from this bucket
                    results
                        .get_mut(&score)
                        .unwrap()
                        .retain(|&i| i != best_existing);
                    results.entry(score + 1).or_default().push(best_existing);
                    results.get_mut(&score).unwrap().push(index);
                } else {
                    // all ties -> keep both at same score
                    results.get_mut(&score).unwrap().push(index);
                }
            }
        }
    }

    // flatten into Vec<(score, index)>
    let mut output: Vec<(u16, usize)> = Vec::new();
    for (score, idxs) in results {
        for idx in idxs {
            output.push((score, idx));
        }
    }

    output.sort_by(|a, b| b.0.cmp(&a.0));

    output
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

    #[test]
    fn test_sort_applications() {
        let now = std::time::Instant::now();
        let query = "zen";
        let apps = find_desktop_files();

        let mut apps_clone = apps.clone();
        let sorted = sort_applications(&mut apps_clone, query);
        assert!(!sorted.is_empty());
        println!("Sorted {} applications in {:?}", apps.len(), now.elapsed());

        let mut i = 1;
        for (score, idx) in sorted {
            println!("Score: {}, App: {:?}", score, apps[idx].name);
            if i >= 10 {
                break;
            }
            i += 1;
        }
    }
}
