/// The prefix prepended to every log message emitted by this extension.
pub const LOG_PREFIX: &str = "[angular-language-server]";

/// Emits an informational log message prefixed with the extension name.
///
/// Accepts the same format string syntax as [`println!`].
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{} {}", $crate::logging::LOG_PREFIX, format!($($arg)*))
    };
}

/// Emits a warning log message prefixed with the extension name.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("{} WARN {}", $crate::logging::LOG_PREFIX, format!($($arg)*))
    };
}

/// Emits an error log message prefixed with the extension name.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("{} ERROR {}", $crate::logging::LOG_PREFIX, format!($($arg)*))
    };
}
