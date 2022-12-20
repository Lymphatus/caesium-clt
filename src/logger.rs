pub enum ErrorLevel {
    Log,
    Warning,
    Error,
}

pub fn log(message: &str, code: i32, level: ErrorLevel, verbose: u8) {
    if verbose == 0 {
        return;
    }
    match level {
        ErrorLevel::Error => panic!("[ERROR] {} (Code: {})", message, code),
        ErrorLevel::Warning => eprintln!("[WARNING] {} (Code: {})", message, code),
        _ => println!("{}", message)
    };
}