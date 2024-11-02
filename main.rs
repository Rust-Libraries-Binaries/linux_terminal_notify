extern crate linux_terminal_notify;

use linux_terminal_notify::{notify, NotificationType};

fn main() {
    notify("Build completed.", NotificationType::Info);
    notify("Disk space low.", NotificationType::Warning);
    notify("Build failed.", NotificationType::Error);
}
