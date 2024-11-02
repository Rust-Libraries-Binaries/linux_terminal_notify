pub enum NotificationType {
    Info,
    Warning,
    Error,
}

pub fn notify(msg: &str, ntype: NotificationType) {
    match ntype {
        NotificationType::Info => println!("\x1b[32m[INFO]\x1b[0m {}", msg),
        NotificationType::Warning => println!("\x1b[33m[WARNING]\x1b[0m {}", msg),
        NotificationType::Error => println!("\x1b[31m[ERROR]\x1b[0m {}", msg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_notification() {
        notify("This is an info message.", NotificationType::Info);
    }

    #[test]
    fn test_warning_notification() {
        notify("This is a warning message.", NotificationType::Warning);
    }

    #[test]
    fn test_error_notification() {
        notify("This is an error message.", NotificationType::Error);
    }
}
