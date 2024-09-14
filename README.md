# AI Terminal

AI Terminal is a simple GUI-based terminal application built with Rust and the [iced](https://github.com/iced-rs/iced) GUI library. It allows users to enter shell commands and view the output within a graphical interface.

This is my first Rust project (though I have experience building smart contracts on Solana in the past). If you're interested, feel free to contribute to this project!

## Features

- **Command Execution**: Enter shell commands and view their output in a scrollable window.
- **Graphical User Interface**: Built using the `iced` library for a modern and responsive GUI.
- **Splash Screen**: Displays a splash screen on startup.
- **Custom Application Icon**: Includes a custom icon when running on macOS.

## Technologies Used

- **Rust**: A systems programming language focused on safety and performance.
  - Version: `1.56` or later (edition 2021)
- **[iced](https://github.com/iced-rs/iced)**: A cross-platform GUI library for Rust, inspired by Elm.
  - Version: `0.12.1`
  - Features Used:
    - `canvas`
    - `tokio`
- **[image](https://crates.io/crates/image)**: An image processing library for Rust.
  - Version: `0.25.2`
  - Used for loading and decoding images for the application icon and splash screen.
- **[cargo-bundle](https://crates.io/crates/cargo-bundle)**: A tool for bundling Rust applications into distributable formats.
  - Used to create a macOS application bundle (`.app`) with a custom icon.
- **[winit](https://crates.io/crates/winit)**: A window handling library in Rust.
  - Used indirectly through `iced` for window management and setting the application icon.

## Getting Started

### Prerequisites

- **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
- **cargo-bundle** (optional, for bundling on macOS): Install via Cargo:
  ```bash
  cargo install cargo-bundle
  ```

### Project Structure

```
ai_terminal/
├── Cargo.toml
├── resources/
│   ├── AppIcon.icns       # Application icon for macOS bundling
│   ├── icon.png           # Icon image used in the application
│   └── splash.png         # Splash screen image (optional)
└── src/
    ├── executor.rs        # Module handling command execution
    └── main.rs            # Main application code
```

### Building and Running

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/shubham0850/ai_terminal.git
   cd ai_terminal
   ```

2. **Build the Application**:
   ```bash
   cargo build
   ```

3. **Run the Application**:
   ```bash
   cargo run
   ```

### Bundling for macOS (Optional)

To create a macOS application bundle with a custom icon:

1. **Ensure `AppIcon.icns` is in the `resources` directory**.

2. **Add Bundle Metadata to `Cargo.toml`**:

   ```toml
   [package.metadata.bundle]
   name = "AIShell"
   identifier = "com.example.ai_terminal"
   icon = ["resources/AppIcon.icns"]
   version = "0.1.0"
   ```

3. **Bundle the Application**:
   ```bash
   cargo bundle --release
   ```

4. **Run the Bundled Application**:
   - Navigate to `target/release/bundle/osx/AIShell.app` and run it.

## Usage

- **Enter Commands**: Type shell commands into the input field at the bottom of the window.
- **Execute Commands**: Press the "Run" button or hit `Enter` to execute the command.
- **View Output**: The output will be displayed in the scrollable area above the input field.
- **Clear Output**: (Optional feature if implemented) Click the "Clear" button to clear the output log.

## Screenshots

*(Include screenshots of your application here if available)*

## Contributing

Contributions are welcome! If you're interested in contributing to this project:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with a description of your changes.

Feel free to open issues for suggestions and improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the [iced](https://github.com/iced-rs/iced) community for providing an excellent GUI library for Rust.
- Inspired by learning experiences from building smart contracts on Solana.

## Contact

- **Author**: Shubham Raj
- **Email**: rajshubham0850@gmail.com
- **GitHub**: [Shubham0850](https://github.com/Shubham0850)

---

Feel free to customize this README to better suit your project. Include any additional sections or information that you think would be helpful for users and contributors.