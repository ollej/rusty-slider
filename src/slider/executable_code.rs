use std::fmt;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(Clone)]
pub enum ExecutableCode {
    Bash(String),
    Python(String),
    Ruby(String),
    Perl(String),
}

impl fmt::Display for ExecutableCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutableCode::Bash(_) => write!(f, "bash"),
            ExecutableCode::Python(_) => write!(f, "python"),
            ExecutableCode::Ruby(_) => write!(f, "ruby"),
            ExecutableCode::Perl(_) => write!(f, "perl"),
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
            _ => None,
        }
    }

    pub fn execute(&self) -> String {
        match self {
            ExecutableCode::Bash(code) => return self.execute_command("bash", "-", code),
            ExecutableCode::Python(code) => return self.execute_command("python3", "-", code),
            ExecutableCode::Ruby(code) => return self.execute_command("ruby", "-", code),
            ExecutableCode::Perl(code) => return self.execute_command("perl", "-", code),
        }
    }

    fn execute_command(&self, command: &str, argument: &str, code: &String) -> String {
        let process = match Command::new(command)
            .arg(argument)
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
