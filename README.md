## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## Linux Terminal Notify

**linux_terminal_notify** is a lightweight Rust library for sending terminal notifications on Linux systems. It allows developers to easily send customizable notifications directly to the terminal, enhancing the user experience with real-time alerts.

## Features
- **Simple API**: Easy-to-use functions to send notifications.
- **Customizable Messages**: Specify different types of notifications (info, warning, error).
- **Terminal Colors**: Color-coded messages for better visibility.
- **Optional Sound Alerts**: Support for sound notifications if the terminal supports it.

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
linux_terminal_notify = "0.1.0"
```
## Usage
Here’s a quick example to get you started:
```rust
extern crate linux_terminal_notify;

use linux_terminal_notify::{notify, NotificationType};

fn main() {
    notify("Build completed.", NotificationType::Info);
    notify("Disk space low.", NotificationType::Warning);
    notify("Build failed.", NotificationType::Error);
}
```
## Examples
```rust
extern crate linux_terminal_notify;

use linux_terminal_notify::{notify, NotificationType};

fn main() {
    // Info notification
    notify("Deployment successful!", NotificationType::Info);
    
    // Warning notification
    notify("Memory usage is high.", NotificationType::Warning);

    // Error notification
    notify("Unable to connect to database.", NotificationType::Error);
}
```
## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Author
Ben Santora 
