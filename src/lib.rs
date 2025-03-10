use rust_fuzzy_search::fuzzy_search_sorted;
use std::{collections::HashSet, vec};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Terminal {
    cwd: String,
    directories: HashSet<String>,
    history_index: usize,
    history: Vec<String>,
}

/// import console.log for debugging purposes
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Terminal {
    /// Constructor for Terminal
    #[wasm_bindgen(constructor)]
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
            history_index: 0,
            history: Vec::new(),
        }
    }

    /// takes in the currently typed command and returns the most likely command via a kind of
    /// fuzzy search that the user is trying to type.
    pub fn tab_complete(&mut self, command: &str) -> String {
        //self.history_index += 1;
        if command.is_empty() {
            return "".to_string();
        }
        let commands: Vec<&str> = vec!["help", "about", "projects", "contact", "cd", "test"];

        let result: Vec<(&str, f32)> = fuzzy_search_sorted(command, &commands);

        result[0].0.to_string();

        if result.is_empty() {
            command.to_string()
        } else {
            result[0].0.to_string()
        }
    }

    /// returns the next or previous command in the command history based on the dircection given.
    /// only accepts "up" or "down" as direction"
    pub fn cycle_command_history(&mut self, direction: String) -> String {
        if self.history.is_empty() {
            return "".to_string();
        }
        if direction == "up" && self.history_index > 0 {
            self.history_index -= 1;
        } else if direction == "down" && self.history_index < self.history.len() - 1 {
            self.history_index += 1;
        }
        self.history[self.history_index].clone()
    }

    /// takes in a string command and returns the output as a string
    pub fn handle_command(&mut self, command: &str) -> Result<String, JsValue> {
        if command.is_empty() {
            return Ok("".to_string());
        }
        self.history_index += 1;
        let command: Result<String, JsValue> = match command.trim() {
            "help" => {
                self.history.push("help".to_string());
                Ok("help: display this help message\nabout: prints information about the author of this website.\nprojects: prints information about my most recent projects.\ncontact: prints all the current ways you can contact me.\ncd: change directory\n"
                .to_string())
            }
            "about" => {
                self.history.push("about".to_string());
                Ok(
                    "I am a software developer, working primarily in C#. I have recently begun to learn Rust, which is also what I have used to make most of the logic on this site!\nI am currently studying under an education we roughly translate to \"data and communication\" in Denmark, where I am from.\nI started programming when i was around 16, with some simple python scripts, and have since then moved on to more complex projects.\nWhen I started really getting into programming, I found something called Neovim, which is a text editor that is highly customizable, and I haven't been able to look away since. I love how i am able to do anything with just my keyboard, I especially love the feeling of owning everything that happens.\n"
                    .to_string(),
                )
            }
            "projects" => {
                self.history.push("projects".to_string());
                Ok(
                    "I have a few projects on my GitHub (https://www.github.com/andr515i), but most noteworthy are these projects:\n
                    1. test project at http://www.github.com/andr515i/test\n"
                    .to_string(),
                )
            }
            "contact" => {
                self.history.push("contact".to_string());
                Ok("You can contact me at\n
                    1. email: andreasmadsen64@gmail.com \n
                    2. linkedin: https://www.linkedin.com/in/andreas-madsen-35a856152/\n"
                    .to_string())
            }
            command if command.starts_with("cd ") => {
                self.history.push(command.to_string());
                let input_dir = command.strip_prefix("cd ").unwrap_or("");
                let target_path = resolve_path(&self.cwd, input_dir);
                if self.directories.contains(&target_path) {
                    self.cwd = target_path.clone();
                    Ok(format!("changed directory to {}", self.cwd))
                } else if command == "cd .." {
                    self.cwd = go_back_one_dir(&self.cwd);
                    if self.cwd.is_empty() {
                        self.cwd = "/".to_string();
                    }
                    Ok(format!("changed directory to {}", self.cwd))
                } else {
                    Err(JsValue::from_str("directory not found"))
                }
            }
            "test" => {
                self.history.push("test".to_string());
                Ok("test\n".to_string())
            }
            _ => Ok("unknown command\n".to_string()),
        };
        command
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
