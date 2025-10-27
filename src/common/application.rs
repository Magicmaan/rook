use color_eyre::Result;
use color_eyre::eyre::eyre;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Application {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    // pub icon_ascii: Option<String>,
    pub comment: Option<String>,
    pub categories: Vec<String>,
    pub terminal: bool,
    pub mime_types: Vec<String>,
    pub desktop_file_path: PathBuf,
}
impl Application {
    pub fn launch(&self) -> Result<()> {
        // run the application using std::process::Command
        let exec_parts: Vec<&str> = self.exec.split_whitespace().collect();
        if exec_parts.is_empty() {
            return Err(eyre!("No executable found for application: {}", self.name));
        }
        let exec_str = self.exec.clone();
        let binding = PathBuf::from(&exec_str);
        let executable = binding.file_name().unwrap();

        let mut cmd: Vec<&str> = vec![];
        if self.terminal {
            // launch in terminal
            // try to get preferred terminal from env
            let terminal = "kitty"; // TODO: make configurable
            cmd.push(&terminal);
            cmd.push("-e");
            cmd.push(self.exec.as_str());
        } else {
            // launch directly
            cmd.push("gtk-launch"); // use gtk-launch to launch the application properly
            cmd.push(
                self.desktop_file_path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );
        }

        let mut exec = std::process::Command::new(&cmd[0]);
        log::info!(
            "Launching application: {} with command: {:?}",
            self.name,
            cmd
        );
        if cmd.len() > 1 {
            exec.args(&cmd[1..]);
        }
        exec.stderr(std::process::Stdio::null());
        exec.stdout(std::process::Stdio::null());
        exec.stdin(std::process::Stdio::null());
        unsafe {
            exec.pre_exec(|| {
                // Become independent of the parent process
                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }

                Ok(())
            });
        }

        exec.spawn().is_err().then(|| {
            Some(Err::<(), color_eyre::Report>(eyre!(
                "Failed to launch application: {} \nExecutable Path: {}",
                self.name,
                self.exec
            )))
        });
        sleep(Duration::from_millis(100)); // give some time for the application to launch

        Ok(())
    }
}
