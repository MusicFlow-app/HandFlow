use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::fs::{self};

/// Asynchronously cleans up old uploaded files from a specified directory.
///
/// This function:
///
/// 1. **Directory Check**: Converts the provided directory path to a `PathBuf` and checks if it exists.
/// 2. **File Iteration**: Asynchronously iterates over files in the directory.
/// 3. **Age Calculation**: Determines the age of each file by comparing the current time with the last modified time.
/// 4. **File Deletion**: Deletes files that exceed the specified maximum age (`max_age`).
///
/// # Parameters
/// - `dir`: The directory path as a `&str`.
/// - `max_age`: The maximum age for files as a `Duration`.
///
/// # Returns
/// - `Ok(())` if the cleanup is successful.
/// - An `std::io::Result` error if any I/O operations fail.
pub async fn clean_old_uploads(dir: &str, max_age: Duration) -> std::io::Result<()> {
    let upload_dir = PathBuf::from(dir);
    if upload_dir.exists() {
        let mut entries = fs::read_dir(upload_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let modified = metadata.modified()?;
            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap_or(Duration::from_secs(0));
            if age > max_age {
                fs::remove_file(entry.path()).await?;
            }
        }
    }
    Ok(())
}

/// Sanitizes a given file name by removing potentially dangerous or invalid characters.
///
/// This function removes instances of "..", "/", and "\\" from the file name to prevent directory traversal attacks.
///
/// # Parameters
/// - `file_name`: The file name to sanitize as a `&str`.
///
/// # Returns
/// - A `String` containing the sanitized file name.
pub fn sanitize_file_name(file_name: &str) -> String {
    file_name
        .replace("..", "")
        .replace("/", "")
        .replace("\\", "")
}

/// Asynchronously reads the content of an MSCX file into a string.
///
/// This function:
///
/// 1. **Buffer Setup**: Initializes a string buffer to hold the content of the file.
/// 2. **File Reading**: Uses a buffered reader to read the entire content of the file into the buffer.
///
/// # Parameters
/// - `reader`: A generic reader that implements the `Read` trait.
///
/// # Returns
/// - A `Result<String, io::Error>` containing the file content or an I/O error.
pub async fn read_mscx<R: Read>(reader: R) -> io::Result<String> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(reader);
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Validates a ZIP archive to ensure it meets specific size constraints.
///
/// This function:
///
/// 1. **Maximum File Size Check**: Defines a maximum file size (100 MB) for individual files within the ZIP archive.
/// 2. **File Iteration**: Iterates through each file in the ZIP archive and checks its size.
/// 3. **Total Size Check**: Tracks the total uncompressed size of all files in the ZIP archive.
///
/// # Parameters
/// - `zip`: A mutable reference to a `zip::ZipArchive` containing a file.
///
/// # Returns
/// - `true` if all files in the ZIP archive are within the allowed size limits.
/// - `false` if any file exceeds the maximum allowed size or if the total uncompressed size of all files exceeds the limit.
pub fn is_valid_zip(zip: &mut zip::ZipArchive<std::fs::File>) -> bool {
    let max_file_size = 100 * 1024 * 1024; // 100 MB
    let mut total_uncompressed_size = 0;

    for i in 0..zip.len() {
        let file = match zip.by_index(i) {
            Ok(file) => file,
            Err(_) => return false,
        };

        if file.size() > max_file_size {
            return false;
        }

        total_uncompressed_size += file.size();

        if total_uncompressed_size > max_file_size {
            return false;
        }
    }

    true
}
