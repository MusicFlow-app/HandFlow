use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tokio::fs;

static HEADER_CONTENT: OnceCell<String> = OnceCell::new();

/**
 * Loads and caches the header content for the HTML pages.
 *
 * This function:
 *
 * 1. **Checks Cache**: If the header content is already cached in `HEADER_CONTENT`, it returns the cached content.
 * 2. **Reads Header File**: If not cached, it reads the header content from the `html_tmpl.html` file asynchronously.
 * 3. **Handles Errors**: Logs an error and returns an empty string if the file cannot be read.
 * 4. **Caches Content**: Stores the content in `HEADER_CONTENT` for future use.
 * 5. **Returns**: The header content as a `String`.
 *
 * @return A `String` containing the header content.
 */
pub async fn load_header_content() -> String {
    if let Some(content) = HEADER_CONTENT.get() {
        content.clone()
    } else {
        let header_path = PathBuf::from("src/html/html_tmpl.html");
        let content = match fs::read_to_string(header_path).await {
            Ok(content) => content,
            Err(_) => {
                log::error!("Failed to read html_tmpl.html");
                String::new()
            }
        };
        HEADER_CONTENT.set(content.clone()).unwrap();
        content
    }
}

/**
 * Sanitizes a given HTML input string to escape potentially dangerous characters.
 *
 * This function:
 *
 * 1. **Escapes HTML Characters**: Uses the `htmlescape` crate to encode minimal HTML entities, preventing injection attacks.
 * 2. **Returns**: The sanitized HTML as a `String`.
 *
 * @param input The input string that may contain unsafe characters.
 * @return A `String` containing the sanitized HTML.
 */
pub fn sanitize_html(input: &str) -> String {
    htmlescape::encode_minimal(input)
}

/**
 * Retrieves the color associated with a given musical note duration.
 *
 * This function:
 *
 * 1. **Maps Duration to Color**: Matches the duration string (e.g., "quarter", "half") to a specific color hex code.
 * 2. **Returns**: The associated color as an `Option<&'static str>`. Returns `None` if the duration is not recognized.
 *
 * @param duration The duration of the musical note (e.g., "quarter", "half").
 * @return An `Option<&'static str>` containing the color hex code or `None`.
 */
pub fn get_color_for_duration(duration: &str) -> Option<&'static str> {
    match duration {
        "64th" => Some("#B13B8E"),
        "32nd" => Some("#4B348B"),
        "16th" => Some("#4563AC"),
        "eighth" => Some("#32CD32"),
        "quarter" => Some("#DAA520"),
        "half" => Some("#FF4500"),
        "whole" => Some("#8B0000"),
        _ => None,
    }
}

/**
 * Generates an HTML legend for musical note and rest durations, displaying their corresponding colors.
 *
 * This function:
 *
 * 1. **Defines Durations**: Creates a list of musical note durations (e.g., "64th", "32nd", "16th").
 * 2. **Creates Legend Structure**: Builds the HTML structure for the legend, including color boxes and labels.
 * 3. **Loads SVGs**: For each duration, loads the corresponding SVG for the rest symbol and incorporates it into the legend.
 * 4. **Returns**: The complete HTML string for the legend.
 *
 * @return A `String` containing the HTML structure for the note & rest duration legend.
 */
pub fn generate_html_css_legend() -> String {
    let durations = vec!["64th", "32nd", "16th", "eighth", "quarter", "half", "whole"];
    let mut legend_html = String::from(
        r#"
    <div id="legends" class="information-container">
        <h3 class="info-title">Note & Rest Duration Legend</h3>
        <div class="legend-items">
    "#,
    );

    for duration in durations {
        if let Some(color) = get_color_for_duration(duration) {
            let rest = if duration == "whole" {
                crate::utils::svg::load_svg_for_rest("measure").unwrap_or_default()
            } else {
                crate::utils::svg::load_svg_for_rest(duration).unwrap_or_default()
            };
            legend_html.push_str(&format!(
                r#"
                <div class="legend-item">
                    <div class="color-box" style="background-color:{};"></div>
                    <span class="duration-label">{}</span>
                    <div class="rest-box">{}</div>
                </div>
                "#,
                color, duration, rest
            ));
        }
    }

    legend_html.push_str("</div></div>\n");
    legend_html
}
