/// Logs an error message along with a formatted error value using the `log` crate.
///
/// This function:
///
/// 1. **Generic Parameter**: Accepts a generic parameter `T` that implements the `std::fmt::Debug` trait, allowing it to log any value that can be formatted using `Debug`.
/// 2. **Error Logging**: Uses the `log::error!` macro to log an error-level message.
///     - `message`: A string slice that serves as the main error message.
///     - `err`: The error or additional information to be logged, formatted using the `Debug` trait.
/// 3. **Usage**: Ensures consistent and structured error logging across the application.
///
/// # Parameters
/// - `message`: A `&str` representing the main error message.
/// - `err`: A generic parameter `T` that implements the `Debug` trait, representing the error or additional context to log.
pub fn log_error<T: std::fmt::Debug>(message: &str, err: T) {
    log::error!("{}: {:?}", message, err);
}
