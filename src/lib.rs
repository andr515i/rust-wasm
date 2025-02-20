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
        directories.insert("/projects/rust".to_string());
        directories.insert("/projects/angular".to_string());
        directories.insert("/home".to_string());
        directories.insert("/home/user".to_string());
        directories.insert("/home/user/documents".to_string());

        Terminal {
            cwd: "/".to_string(),
            directories,
        }
    }

    /// returns all available commands, in a list
    pub fn get_commands(&self) -> Vec<String> {
        vec![
            "help".to_string(),
            "about".to_string(),
            "projects".to_string(),
            "contact".to_string(),
            "cd".to_string(),
            "test".to_string(),
        ]
    }

    /// takes in the currently typed command and returns the most likely command via a kind of
    /// fuzzy search that the user is trying to type.
    pub fn tab_complete(&self, command: &str) -> Vec<String> {
        let mut commands = self.get_commands();
        commands.sort();
        commands
            .into_iter()
            .filter(|c| c.starts_with(command))
            .collect()
    }

    /// takes in a string command and returns the output as a string
    pub fn handle_command(&mut self, command: &str) -> Result<String, JsValue> {
        if command.is_empty() {
            return Ok("".to_string());
        }
        match command.trim() {
            "help" => {
                Ok(
                "help: display this help message\n
                    about: prints information about the author of this website.\n
                    projects: prints information about my most recent projects.\n
                    contact: prints all the current ways you can contact me.\n
                    cd: change directory\n
                    "
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
                    let input_dir = command.strip_prefix("cd ").unwrap_or("");
                    let target_path = resolve_path(&self.cwd, input_dir);
                    if self.directories.contains(&target_path) {
                        self.cwd = target_path.clone();
                        Ok(format!("changed directory to {}", self.cwd))
                    }
                    else if 
                        command == "cd .." {
                        self.cwd = go_back_one_dir(&self.cwd);    
                        if self.cwd.is_empty() {
                            self.cwd = "/".to_string();
                        }
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
                Ok("unknown command\n".to_string())
            }
        }
    }
}

/// helper funcion to reslove a new path on the current working directory
/// if new path starts with '/' it will be resolved from the root directory
/// otherwise it will be resolved from the current working directory
fn resolve_path(cwd: &str, new_dir: &str) -> String {
    if new_dir.starts_with('/') {
        new_dir.to_string()
    } else {
        let mut path = cwd.to_string();
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(new_dir);
        path
    }
}

/// helper function to go back one directory
/// if the current working directory is the root directory, it will return the root directory
fn go_back_one_dir(cwd: &str) -> String {
    let path = cwd.to_string();
    if path == "/" {
        path
    } else {
        let mut split_path = path.split('/').collect::<Vec<&str>>();
        split_path.pop();
        split_path.join("/")
    }
}


