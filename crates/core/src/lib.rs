pub fn hello_message() -> String {
    "Hello from kinematics-core (Rust)!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_message_is_non_empty() {
        assert!(!hello_message().is_empty());
    }
}
