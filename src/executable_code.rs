use std::ffi::OsStr;
use std::fmt;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use tempdir::TempDir;

#[derive(Clone)]
pub enum ExecutableCode {
    Bash(String),
    Python(String),
    Ruby(String),
    Perl(String),
    Rust(String),
}

impl fmt::Display for ExecutableCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutableCode::Bash(_) => write!(f, "bash"),
            ExecutableCode::Python(_) => write!(f, "python"),
            ExecutableCode::Ruby(_) => write!(f, "ruby"),
            ExecutableCode::Perl(_) => write!(f, "perl"),
            ExecutableCode::Rust(_) => write!(f, "rust"),
        }
    }
}

impl ExecutableCode {
    pub fn from(language: &String, code: &String) -> Option<Self> {
        match language.as_str() {
            "bash" | "sh" => Some(ExecutableCode::Bash(code.to_string())),
            "python" => Some(ExecutableCode::Python(code.to_string())),
            "ruby" => Some(ExecutableCode::Ruby(code.to_string())),
            "perl" => Some(ExecutableCode::Perl(code.to_string())),
            "rust" => Some(ExecutableCode::Rust(code.to_string())),
            _ => None,
        }
    }

    pub fn execute(&self) -> String {
        match self {
            ExecutableCode::Bash(code) => return self.execute_command("bash", ["-"], code),
            ExecutableCode::Python(code) => return self.execute_command("python3", ["-"], code),
            ExecutableCode::Ruby(code) => return self.execute_command("ruby", ["-"], code),
            ExecutableCode::Perl(code) => return self.execute_command("perl", ["-"], code),
            ExecutableCode::Rust(code) => {
                if let Ok(tmp_dir) = TempDir::new("rusty-slider") {
                    if let Some(file_path) = tmp_dir.path().join("rustc.out").to_str() {
                        self.execute_command("rustc", ["-v", "-o", file_path, "-"], code);
                        return self.run_command_capture_output(file_path);
                    }
                }
                "Error: could not create temp file".to_string()
            }
        }
    }

    fn run_command_capture_output(&self, command: &str) -> String {
        let process = match Command::new(command).stdout(Stdio::piped()).spawn() {
            Err(why) => return self.error(why),
            Ok(process) => process,
        };
        let mut output = String::new();
        match process.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => return self.error(why),
            Ok(_) => (),
        };
        return output;
    }

    fn execute_command<I, S>(&self, command: &str, arguments: I, code: &String) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let process = match Command::new(command)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => return self.error(why),
            Ok(process) => process,
        };
        match process.stdin.unwrap().write_all(code.as_bytes()) {
            Err(why) => return self.error(why),
            Ok(_) => (),
        };
        let mut output = String::new();
        match process.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => return self.error(why),
            Ok(_) => (),
        };
        return output;
    }

    fn error(&self, error: std::io::Error) -> String {
        format!("Error running {}:\n{}", self, error)
    }
}
