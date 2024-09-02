use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::fs::{self};

/**
 * Asynchronously cleans up old uploaded files from a specified directory.
 *
 * This function performs the following steps:
 *
 * 1. **Directory Check**: Converts the provided directory path to a `PathBuf` and checks if it exists.
 *
 * 2. **Iterate Through Files**: Reads the directory entries asynchronously and iterates over them.
 *
 * 3. **File Age Calculation**: For each file, it retrieves the file's metadata to determine the last modified time.
 *    It calculates the file's age by comparing the current time with the last modified time.
 *
 * 4. **File Deletion**: If a file's age exceeds the specified maximum age (`max_age`), the file is deleted asynchronously.
 *
 * 5. **Return Value**: The function returns `Ok(())` if successful, or an `std::io::Result` error if any I/O operations fail.
 */
pub async fn clean_old_uploads(dir: &str, max_age: Duration) -> std::io::Result<()> {
    let upload_dir = PathBuf::from(dir);
    if upload_dir.exists() {
        let mut entries = fs::read_dir(upload_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let modified = metadata.modified()?;
            let age = SystemTime::now().duration_since(modified).unwrap_or(Duration::from_secs(0));
            if age > max_age {
                fs::remove_file(entry.path()).await?;
            }
        }
    }
    Ok(())
}

/**
 * Sanitizes a given file name by removing potentially dangerous or invalid characters.
 *
 * This function:
 *
 * 1. **Replace Dangerous Characters**: Removes instances of "..", "/" and "\\" from the file name to prevent directory traversal attacks.
 *
 * 2. **Return Value**: Returns the sanitized file name as a `String`.
 */
pub fn sanitize_file_name(file_name: &str) -> String {
    file_name.replace("..", "").replace("/", "").replace("\\", "")
}

/**
 * Asynchronously reads the content of an MSCX file into a string.
 *
 * This function:
 *
 * 1. **Buffer Setup**: Initializes a string buffer to hold the content of the file.
 *
 * 2. **File Reading**: Uses a buffered reader to read the entire content of the file into the buffer.
 *
 * 3. **Return Value**: Returns the file content as a `Result<String, io::Error>`.
 */
pub async fn read_mscx<R: Read>(reader: R) -> io::Result<String> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(reader);
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/**
 * Validates a ZIP archive to ensure it meets specific size constraints.
 *
 * This function:
 *
 * 1. **Maximum File Size Check**: Defines a maximum file size (100 MB) for individual files within the ZIP archive.
 *
 * 2. **Iterate Through ZIP Contents**: Iterates through each file in the ZIP archive and checks its size.
 *
 * 3. **Total Size Check**: Keeps a running total of the uncompressed sizes of all files in the ZIP archive.
 *
 * 4. **Validation**: If any file exceeds the maximum allowed size, or if the total uncompressed size of all files exceeds the limit, the function returns `false`.
 *
 * 5. **Return Value**: Returns `true` if all files in the ZIP archive are within the allowed size limits, otherwise returns `false`.
 */
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
