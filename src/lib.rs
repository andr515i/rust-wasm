use std::collections::HashSet;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Terminal {
    cwd: String,
    directories: HashSet<String>,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Terminal {
    /// Constructor for Terminal
    pub fn new() -> Terminal {
        let mut directories = HashSet::new();
        directories.insert("/".to_string());
        directories.insert("/games".to_string());
        directories.insert("/projects".to_string());
        directories.insert("/home".to_string());
        directories.insert("/test/test".to_string());

        Terminal {
            cwd: "/".to_string(),
            directories,
        }
    }
    /// takes in a string command and returns the output as a string
    pub fn handle_command(&mut self, command: &str) -> Result<String, JsValue> {
        match command.trim() {
            "help" => {
                Ok(
                "help: display this help message\n
                    about: prints information about the author of this website.\n
                    projects: prints information about my most recent projects.\n
                    contact: prints all the current ways you can contact me.\n"
                    .to_string(),
                )
            },
            "about" => {
                Ok(
                "I am a software developer, working primarily in C#. I have recently begun to learn Rust, which is also what I have used to make most of the logic on this site!\n"
                    .to_string(),
                )
            },
            "projects" => {
                Ok(
                "I have a few projects on my GitHub (https://www.github.com/andr515i), but most noteworthy are these projects:\n
                    1. test project at http://www.github.com/andr515i/test\n"
                    .to_string(),
                )
            },
            "contact" => {
                Ok(
                "You can contact me at\n
                    1. email: andreasmadsen64@gmail.com \n
                    2. linkedin: https://www.linkedin.com/in/andreas-madsen-35a856152/\n"
                    .to_string(),
                )
            },
            command if command.starts_with("cd ") => {
                let new_dir = command.strip_prefix("cd ").unwrap_or("/");
                if self.directories.contains(new_dir) {
                    self.cwd = new_dir.to_string();
                    Ok(format!("changed directory to {}", self.cwd))
                }
                    else {
                        Err(JsValue::from_str("directory not found"))
                    }
                }
            "test" => {
                Ok("test\n".to_string())
            }
            _ => {
                Ok("unknown command,\n".to_string())
            }
        }
    }
}
