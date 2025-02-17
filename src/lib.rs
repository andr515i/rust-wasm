use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("hello, {}", name));
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub fn handle_command(command: &str) -> String {
    match command {
        "help" => "help: display this help message\n".to_string(),
        "about" => "I am a software developer, working primarily in C#. I have recently begun to learn Rust, which is also what I have used to make most of the logic on this site!\n".to_string(),
        _ => "unknown command".to_string(),
    }
}
