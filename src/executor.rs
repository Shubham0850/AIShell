use std::process::Command;

pub fn execute_command(command_line: &str) -> String {
    // Split the command_line into command and arguments
    let mut parts = command_line.trim().split_whitespace();
    if let Some(command) = parts.next() {
        let args: Vec<&str> = parts.collect();

        // Execute the command
        match Command::new(command).args(&args).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    format!("{}\n{}", stdout, stderr)
                } else {
                    stdout.to_string()
                }
            }
            Err(e) => format!("Failed to execute command: {}", e),
        }
    } else {
        String::from("No command entered.")
    }
}
