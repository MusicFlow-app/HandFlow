use std::fs::File;
use std::io::{self, Read};

/// Loads the SVG content for a handpan scale based on the number of notes.
///
/// This function:
///
/// 1. **Generates the File Name**: Constructs the file name based on the scale length.
/// 2. **Opens the SVG File**: Opens the corresponding SVG file from the `static/img` directory.
/// 3. **Reads the Content**: Reads the content of the SVG file into a string.
///
/// # Parameters
/// - `scale_len`: The number of notes in the scale.
///
/// # Returns
/// An `io::Result<String>` containing the SVG content.
pub fn load_svg_for_scale(scale_len: usize) -> io::Result<String> {
    let file_name = format!("static/img/hand-{}.svg", scale_len);
    let mut file = File::open(file_name)?;
    let mut svg_content = String::new();
    file.read_to_string(&mut svg_content)?;
    Ok(svg_content)
}

/// Loads the SVG content for a rest symbol based on its duration.
///
/// This function:
///
/// 1. **Generates the File Name**: Constructs the file name based on the rest duration (e.g., "quarter", "half").
/// 2. **Opens the SVG File**: Opens the corresponding SVG file from the `static/img` directory.
/// 3. **Reads the Content**: Reads the content of the SVG file into a string.
///
/// # Parameters
/// - `duration`: The duration of the rest (e.g., "quarter", "half").
///
/// # Returns
/// An `io::Result<String>` containing the SVG content.
pub fn load_svg_for_rest(duration: &str) -> io::Result<String> {
    let file_name = format!("static/img/rest-{}.svg", duration);
    let mut file = File::open(file_name)?;
    let mut svg_content = String::new();
    file.read_to_string(&mut svg_content)?;
    Ok(svg_content)
}

/// Modifies the color of a note or rest in the SVG content.
///
/// This function:
///
/// 1. **Handles Special Cases**:
///    - For `note_idx` 999: Changes SVG classes to "base-out-svg" and "note-out-svg".
///    - For `note_idx` 420: Modifies the color of a rest symbol based on the duration.
/// 2. **Standard Case**: Modifies the note's color in the SVG based on the `note_idx` and duration.
/// 3. **Inserts Style**: Adds a `style` attribute to SVG elements to change the fill color and apply stroke styling.
///
/// # Parameters
/// - `svg_content`: The original SVG content as a string.
/// - `note_idx`: The index of the note in the SVG that should be modified.
/// - `duration`: The duration of the note or rest (e.g., "quarter", "half").
///
/// # Returns
/// A `String` containing the modified SVG content.
pub fn modify_svg_note_color(svg_content: &str, note_idx: usize, duration: &str) -> String {
    let mut modified_svg = String::from(svg_content);

    if note_idx == 999 {
        modified_svg = modified_svg.replace("base-svg", "base-out-svg");
        modified_svg = modified_svg.replace("note-svg", "note-out-svg");
    } else if note_idx == 420 {
        if let Some(color) = crate::templates::html::get_color_for_duration(duration) {
            let rest_id = r#"class="rest-svg""#;
            if let Some(pos) = modified_svg.find(rest_id) {
                let style_attr = format!(r#" style="fill:{}""#, color);
                let insert_pos = pos + rest_id.len();
                modified_svg.insert_str(insert_pos, &style_attr);
            }
        }
    } else {
        let note_id = format!(r#"id="note_{}""#, note_idx);
        if let Some(color) = crate::templates::html::get_color_for_duration(duration) {
            if let Some(pos) = modified_svg.find(&note_id) {
                let style_attr = format!(
                    r#" style="fill:{};stroke: black;stroke-width: 0.25em;""#,
                    color
                );
                let insert_pos = pos + note_id.len();
                modified_svg.insert_str(insert_pos, &style_attr);
            }
        }
    }

    modified_svg
}
