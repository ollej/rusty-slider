use std::{
    ffi::OsStr,
    fmt,
    io::prelude::*,
    process::{Command, Stdio},
};

use tempfile::NamedTempFile;

#[derive(Debug)]
pub enum ExecutionError {
    Execute(std::io::Error),
    InputOutput,
    CreateTempFile(String),
    Compile(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::Execute(error) => write!(f, "Execution error: {:?}", error.to_string()),
            ExecutionError::InputOutput => write!(f, "Coulnt't read Std I/O"),
            ExecutionError::CreateTempFile(message) => {
                write!(f, "Creating build file: {}", message)
            }
            ExecutionError::Compile(message) => write!(f, "Compile error: {}", message),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl From<std::io::Error> for ExecutionError {
    fn from(e: std::io::Error) -> Self {
        ExecutionError::Execute(e)
    }
}

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
    pub fn from(language: &str, code: &String) -> Option<Self> {
        match language {
            "bash" | "sh" => Some(ExecutableCode::Bash(code.to_string())),
            "python" => Some(ExecutableCode::Python(code.to_string())),
            "ruby" => Some(ExecutableCode::Ruby(code.to_string())),
            "perl" => Some(ExecutableCode::Perl(code.to_string())),
            "rust" => Some(ExecutableCode::Rust(code.to_string())),
            _ => None,
        }
    }

    pub fn execute(&self) -> Result<String, ExecutionError> {
        match self {
            ExecutableCode::Bash(code) => self.execute_command("bash", ["-"], code),
            ExecutableCode::Python(code) => self.execute_command("python3", ["-"], code),
            ExecutableCode::Ruby(code) => self.execute_command("ruby", ["-"], code),
            ExecutableCode::Perl(code) => self.execute_command("perl", ["-"], code),
            ExecutableCode::Rust(code) => {
                let temp_file = NamedTempFile::new()?;
                let file_path = temp_file.path().to_string_lossy();
                self.execute_command("rustc", ["-v", "-o", &file_path, "-"], code)
                    .map_err(|e| ExecutionError::Compile(e.to_string()))?;
                self.run_command_capture_output(&file_path)
            }
        }
    }

    fn run_command_capture_output(&self, command: &str) -> Result<String, ExecutionError> {
        let process = Command::new(command).stdout(Stdio::piped()).spawn()?;
        let mut output = String::new();
        process.stdout.unwrap().read_to_string(&mut output)?;
        Ok(output)
    }

    fn execute_command<I, S>(
        &self,
        command: &str,
        arguments: I,
        code: &String,
    ) -> Result<String, ExecutionError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let process = Command::new(command)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        process
            .stdin
            .ok_or(ExecutionError::InputOutput)?
            .write_all(code.as_bytes())?;
        let mut output = String::new();
        process
            .stdout
            .ok_or(ExecutionError::InputOutput)?
            .read_to_string(&mut output)?;
        Ok(output)
    }
}
