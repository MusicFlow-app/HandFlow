/**
 * Logs an error message along with a formatted error value using the `log` crate.
 *
 * This function:
 *
 * 1. **Generics**: Accepts a generic parameter `T` that implements the `std::fmt::Debug` trait, allowing it to log any value that can be formatted using `Debug`.
 *
 * 2. **Logging**: Utilizes the `log::error!` macro to log an error-level message.
 *    - The `message` parameter is a string slice that serves as the main error message.
 *    - The `err` parameter represents the error or additional information to be logged.
 *    - The error value is formatted using the `Debug` trait, allowing detailed output.
 *
 * 3. **Usage**: This function is useful for logging errors in a consistent and structured manner throughout the application.
 */
pub fn log_error<T: std::fmt::Debug>(message: &str, err: T) {
    log::error!("{}: {:?}", message, err);
}
