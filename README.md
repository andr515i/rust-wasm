# Ubuntu Terminal Simulator

A web-based terminal simulator built with Angular and Rust (compiled to WebAssembly). This project demonstrates how to integrate Rust logic into an Angular application to create a dynamic, stateful terminal interface with fun, interactive commands and simulated directory navigation.

## Table of Contents

- [Features](#features)
- [Technologies](#technologies)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Terminal Simulation:** Emulates a terminal prompt with a history of commands.
- **Command Handling:** Processes commands such as `help`, `about`, `projects`, `contact`, and directory navigation (`cd`).
- **Rust-Powered Logic:** Most of the business logic and state management (e.g., directory navigation, command parsing) is handled in Rust and compiled to WebAssembly for performance.
- **Extensible Design:** Easily add new commands, subdirectories, and mini-games for a richer user experience.

## Technologies

- **Angular:** Frontend framework for building a modern web interface.
- **Rust & WebAssembly:** High-performance language for handling core logic, compiled to WebAssembly using [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).
- **wasm-pack:** Tool for building and packaging Rust-generated WebAssembly.

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) (v23 or later)
- [Angular CLI](https://cli.angular.io/) (v19 or later)
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (install via `cargo install wasm-pack`)

### Clone and Build

1. **Clone the repository:**

   ```bash
   git clone https://github.com/andr515i/rust-wasm.git
   cd rust-wasm
   ```

2. **Build the Rust side:**

   Navigate to the Root project directory and run:

   ```bash
   wasm-pack build --target web
   ```
    Alternatively you can run this line, to clear you assets folder for all webassembly files, build new files and move them to the correct directory:
    ```bash
    rm -rf ./wasm/src/assets/pkg/* &&  wasm-pack build --target web && mv ./pkg/* ./wasm/src/assets/pkg/
    ```

   This command compiles your Rust code into WebAssembly and generates the necessary JavaScript glue code. Make sure the output (the `pkg` folder) is copied to your Angular `src/assets` folder (or update the Angular assets configuration accordingly).

3. **Install Angular dependencies and run the app:**

   ```bash
   npm install
   ng serve
   ```

   Open [http://localhost:4200](http://localhost:4200) in your browser.

## Usage

Once the application is running:

- The terminal prompt appears on the page.
- Type a command (e.g., `help`, `about`, `cd /projects`) and press Enter.
- The terminal will process the command using the WebAssembly logic and display the result.
- Use the up and down arrow keys to navigate through your command history.

## Project Structure

rust-wasm/

├── angular.json                # Angular CLI configuration, includes assets configuration

├── package.json                # Node dependencies and scripts

├── src/

│   ├── app/

│   │   ├── app.component.html  # Terminal UI

│   │   ├── app.component.ts    # Angular component logic

│   │   └── app.component.css   # Terminal styles

│   └── assets/

│       └── pkg/                # Generated WebAssembly files (from Rust)

├── Cargo.toml                  # Rust project configuration

└── src/

    └── lib.rs                  # Rust source code for terminal logic
    

## Development

### Angular Side

- **Component Development:**  
  The `AppComponent` in `app.component.ts` handles user input, command history, and interaction with the WebAssembly module.
- **Styles and UI:**  
  The terminal UI is defined in `app.component.html` and styled in `app.component.css`.

### Rust Side

- **State Management:**  
  The `Terminal` struct in `lib.rs` manages the current working directory (`cwd`) and available directories (stored in a `HashSet`).
- **Command Processing:**  
  The `handle_command` method processes commands, resolves directory paths, and returns results (or errors) using `Result<String, JsValue>`.
- **Path Resolution Helpers:**  
  Helper functions like `resolve_path` and `go_back_one_dir` provide path resolution logic.

## Contributing

Contributions are welcome! Please fork this repository and submit a pull request with your changes. Before submitting, make sure your code adheres to the project's coding style

## License

This project is licensed under the [MIT License](LICENSE).

