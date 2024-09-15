# AI Shell 🚀

![AI Shell Screenshot](https://github.com/Shubham0850/AIShell/blob/main/resources/AIShell-SS.png)

AI Shell is a simple GUI-based terminal application built with Rust 🦀 and the [iced](https://github.com/iced-rs/iced) ❄️ GUI library. It allows users to enter shell commands 💻 and view the output within a graphical interface 🖥️.

## Why Rust ?
Here's an updated table focusing on the **Size** aspect, incorporating typical application sizes:

| **Aspect**  | **Rust**                                               | **C++**                                              | **Go**                                               | **Python (PyInstaller)**                             | **Java (JAR files)**                                 | **Electron (JavaScript)**                            |
|-------------|--------------------------------------------------------|------------------------------------------------------|------------------------------------------------------|------------------------------------------------------|------------------------------------------------------|------------------------------------------------------|
| **Size**    | - **~8.3 MB** (as in your app)<br>- Produces small, standalone binaries without large runtimes | - ~10-15 MB<br>- Can produce small binaries but may require manual optimization | - ~10-20 MB<br>- Includes garbage collector, slight overhead | - ~20-30 MB<br>- Requires bundling interpreter and libraries | - ~50-100 MB<br>- Requires JVM, adds overhead | - ~50-150 MB<br>- Includes Chromium and Node.js runtime, leading to large sizes |

**Key Takeaways:**

- **Rust's Small Binary Size:** Your 8.3MB application showcases Rust's efficiency in producing compact binaries, which is especially beneficial for distribution, faster downloads, and reduced storage requirements.
  
- **Comparison with Other Frameworks:**
  - **Python:** Applications bundled with tools like PyInstaller often exceed 20 MB because they need to include the Python interpreter and all dependencies.
  - **Java:** Java applications require the Java Virtual Machine (JVM), adding significant overhead and increasing the overall size.
  - **Electron:** Apps built with Electron can easily exceed 50 MB because they bundle a Chromium browser and Node.js runtime.

- **No Need for Large Runtimes:** Rust compiles directly to machine code, eliminating the need for a virtual machine or interpreter at runtime, which contributes to smaller application sizes.

## Features ✨

- **Command Execution**: Enter shell commands 💻 and view their output in a scrollable window 📜.
- **Graphical User Interface**: Built using the `iced` ❄️ library for a modern and responsive GUI 🖥️.
- **Splash Screen**: Displays a splash screen 🎉 on startup.
- **Custom Application Icon**: Includes a custom icon 🖼️ when running on macOS 🍎.

## Technologies Used 🛠️

- **Rust** 🦀: A systems programming language focused on safety 🛡️ and performance 🚀.
  - Version: `1.56` or later (edition 2021)
- **[iced](https://github.com/iced-rs/iced)** ❄️: A cross-platform GUI library for Rust, inspired by Elm 🌳.
  - Version: `0.12.1`
  - Features Used:
    - `canvas` 🎨
    - `tokio` ⚡
- **[cargo-bundle](https://crates.io/crates/cargo-bundle)** 📦: A tool for bundling Rust applications into distributable formats.
  - Used to create a macOS 🍎 application bundle (`.app`) with a custom icon 🖼️.
- **[winit](https://crates.io/crates/winit)** 🪟: A window handling library in Rust.
  - Used indirectly through `iced` ❄️ for window management and setting the application icon 🖼️.

### Project Structure 🗂️

```
ai_shell/
├── Cargo.toml
├── resources/
│   ├── AppIcon.icns       # Application icon 🖼️ for macOS bundling
│   ├── icon.png           # Icon image 🖼️ used in the application
│   └── splash.png         # Splash screen image 🎉 (optional)
└── src/
    ├── executor.rs        # Module handling command execution 💻
    └── main.rs            # Main application code 🖥️
```

### Building and Running 🏗️

1. **Clone the Repository** 📥:
   ```bash
   git clone https://github.com/shubham0850/AIShell.git
   cd AIShell
   ```

2. **Build the Application** 🛠️:
   ```bash
   cargo build
   ```

3. **Run the Application** ▶️:
   ```bash
   cargo run
   ```

   For development with live reloading ♻️:
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

4. **Build the Application for Distribution** 📦:
   ```bash
   cargo install cargo-bundle  # Install cargo-bundle if not already installed
   cargo bundle --release      # Build and bundle the application
   ```

   This command will create a bundled application in the `target/release/bundle/` directory, including a macOS `.app` bundle with the custom icon 🖼️ specified in `Cargo.toml`.

## License 📄

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- Thanks to the [iced](https://github.com/iced-rs/iced) ❄️ community for providing an excellent GUI library for Rust 🦀.

## Contact 📬

- **Author**: Shubham Raj
- **Email**: rajshubham0850@gmail.com
- **GitHub**: [Shubham0850](https://github.com/Shubham0850)
- **LinkedIn**: [Shubham Raj](https://www.linkedin.com/in/shubham0850/)

---

Feel free to customize this README to better suit your project. Include any additional sections or information that you think would be helpful for users and contributors.