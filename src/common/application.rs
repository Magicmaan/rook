use color_eyre::Result;
use color_eyre::eyre::eyre;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use xdgkit::desktop_entry::DesktopEntry;

use crate::app::App;

// Exec field codes as per the Desktop Entry Specification
// See: https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html
const EXEC_CODES: &[&str] = &[
    "%f", // single file
    "%F", // multiple files
    "%u", // single URL
    "%U", // multiple URLs
    "%i", // icon
    "%c", // translated name
    "%k", // location of the desktop file
    "%d", // deprecated
    "%D", // deprecated
    "%n", // deprecated
    "%N", // deprecated
    "%v", // deprecated
    "%m", // reserved
];

#[derive(Clone, Debug)]
pub struct TerminalCommand {
    pub exec: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Application {
    DesktopFile(DesktopEntry, String), // DesktopEntry and its path
    TerminalCommand(TerminalCommand, String),
}
impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Application::DesktopFile(a, x), Application::DesktopFile(b, y)) => x == y,
            (_, _) => false,
        }
    }
}
impl Eq for Application {}
impl Application {
    pub fn launch(&self) -> bool {
        let exec_string = match self.exec_string() {
            Some(s) => s,
            None => {
                log::error!("No executable found for application: {}", self.name());
                return false;
            }
        };
        let name = self.name();

        // run the application using std::process::Command
        let exec_parts: Vec<&str> = exec_string
            .split_whitespace()
            .into_iter()
            .filter(|e| !EXEC_CODES.contains(e))
            .collect();
        if exec_parts.is_empty() {
            log::error!("No executable found for application: {}", name);
            return false;
        }

        let mut cmd: Vec<&str> = vec![];
        if self.is_terminal() {
            // launch in terminal
            // try to get preferred terminal from env
            let terminal = "kitty"; // TODO: make configurable
            cmd.push(&terminal);
            cmd.push("-e");
        }
        cmd.extend(exec_parts.iter());

        let mut exec_command = std::process::Command::new(&cmd.iter().next().unwrap());
        log::info!("Launching application: {} with command: {:?}", name, cmd);
        exec_command.stderr(std::process::Stdio::null());
        exec_command.stdout(std::process::Stdio::null());
        exec_command.stdin(std::process::Stdio::null());
        unsafe {
            exec_command.pre_exec(|| {
                // Become independent of the parent process
                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }

                Ok(())
            });
        }

        exec_command.spawn().is_err().then(|| {
            return false;
        });
        sleep(Duration::from_millis(100)); // give some time for the application to launch

        true
    }
    pub fn name(&self) -> String {
        match self {
            Application::DesktopFile(desktop_entry, _) => desktop_entry
                .name
                .as_ref()
                .unwrap_or(&"UNKNOWN APPLICATION".to_string())
                .to_string(),
            Application::TerminalCommand(cmd, _) => {
                cmd.name.clone().unwrap_or("UNKNOWN COMMAND".to_string())
            }
        }
    }

    pub fn path(&self) -> Option<String> {
        match self {
            Application::DesktopFile(desktop_entry, path) => Some(path.clone()),
            Application::TerminalCommand(_, path) => Some(path.clone()),
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            Application::DesktopFile(desktop_entry, _) => desktop_entry.terminal.unwrap_or(false),
            Application::TerminalCommand(_, _) => true,
        }
    }
    pub fn exec_string(&self) -> Option<String> {
        match self {
            Application::DesktopFile(desktop_entry, _) => desktop_entry.exec.clone(),
            Application::TerminalCommand(cmd, _) => cmd.exec.clone(),
        }
    }
}
