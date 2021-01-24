use std::io;

/// Keyboard input
pub fn input() -> String {
    let mut input = String::new();
    return match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => "Input Error...".to_string(),
    };
}
