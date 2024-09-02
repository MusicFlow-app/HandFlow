use crate::utils::{file::read_mscx, scales::get_handpan_scale};
use actix_web::{web::Form, Error, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A static atomic counter used to track the number of active generation requests.
/// This helps enforce rate limiting by ensuring that no more than a specified
/// number of generate requests are processed concurrently.
static GENERATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// The maximum number of concurrent generation requests allowed. If the number of
/// active requests exceeds this value, additional requests will be rejected with a
/// "Too Many Requests" response.
const MAX_GENERATES: usize = 100;

/// A data structure representing the form data submitted with a generate request.
///
/// Fields:
/// - `mscx_path`: The file path to the MSCX file to be processed.
/// - `part_name`: The name of the musical part being processed.
/// - `part_id`: The ID of the specific part within the MSCX file to be processed.
/// - `scale`: The index of the scale to be used in the generation process.
/// - `auto_transpose`: An optional flag indicating whether auto-transposition should be applied.
/// - `play_only_inscale`: An optional flag indicating whether only in-scale notes should be played.
/// - `transpose`: An optional value specifying the number of semitones by which the notes should be transposed.
#[derive(Deserialize)]
pub struct GenerateForm {
    mscx_path: String,
    part_name: String,
    part_id: u32,
    scale: usize,
    auto_transpose: Option<String>,
    play_only_inscale: Option<String>,
    transpose: Option<String>,
}

/// Handles the generation of musical content based on an uploaded MSCX file and user-provided parameters.
///
/// This function performs the following tasks:
///
/// 1. **Rate Limiting**: Checks the current number of active generate requests against a maximum limit. If the limit is exceeded, returns a "Too Many Requests" response.
/// 2. **Form Processing**: Extracts and processes parameters from the form, including the path to the MSCX file, part name, part ID, scale, and various options for transposition and note filtering.
/// 3. **File Handling**: Attempts to open and read the MSCX file specified in the form. If the file cannot be opened or read, an error response is returned.
/// 4. **Scale Selection**: Retrieves the handpan scale based on the provided scale index. If the scale is invalid, an error response is returned.
/// 5. **Template Loading**: Loads the HTML template used for generating the response. If the template cannot be opened or read, an error response is returned.
/// 6. **MSCX Parsing**: Parses the MSCX content to extract musical measures, applying any necessary transpositions and scale constraints.
/// 7. **SVG Handling**: Loads an SVG representation of the scale. If the SVG cannot be loaded, an error response is returned.
/// 8. **HTML Generation**: Generates HTML content representing the musical measures and integrates it with the loaded template.
/// 9. **Response Construction**: Replaces placeholders in the template with the generated content and returns the final HTML response to the client.
///
/// # Parameters
/// - `_req`: The incoming `HttpRequest`.
/// - `form`: The form data submitted by the client, wrapped in `Form<GenerateForm>`.
///
/// # Returns
/// - `Result<HttpResponse, Error>`: The final HTML response or an error if any step fails.
pub async fn handle_generate(
    _req: HttpRequest,
    form: Form<GenerateForm>,
) -> Result<HttpResponse, Error> {
    // Increment the generate counter and check if the maximum number of concurrent requests is exceeded
    let current_generates = GENERATE_COUNTER.fetch_add(1, Ordering::SeqCst);

    if current_generates >= MAX_GENERATES {
        GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
        return Ok(HttpResponse::TooManyRequests().body("Too many requests in progress"));
    }

    // Extract form data into individual variables
    let GenerateForm {
        mscx_path,
        part_name,
        part_id,
        scale,
        auto_transpose,
        play_only_inscale,
        transpose,
    } = form.into_inner();

    // Convert optional form fields into concrete values
    let auto_transpose = auto_transpose.is_some();
    let play_only_inscale: bool = play_only_inscale.map(|v| v == "1").unwrap_or(false);
    let transpose_value: i32 = transpose
        .unwrap_or_else(|| "0".to_string())
        .parse()
        .unwrap_or(0);

    // Attempt to open the MSCX file and handle any errors
    let file = match File::open(&mscx_path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open MSCX file: {:?}", e);
            GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return Ok(HttpResponse::InternalServerError().body("Failed to open MSCX file"));
        }
    };

    // Read the content of the MSCX file into a string
    let reader = BufReader::new(file);
    let mscx_content = match read_mscx(reader).await {
        Ok(content) => content,
        Err(e) => {
            log::error!("Failed to read MSCX content: {:?}", e);
            GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return Ok(HttpResponse::InternalServerError().body("Failed to read MSCX content"));
        }
    };

    // Retrieve the handpan scale based on the provided index, or return an error if the scale is invalid
    let (scale_name, scale_notes, scale_tpc) = match get_handpan_scale(scale) {
        Some(scale_data) => scale_data,
        None => {
            GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return Ok(HttpResponse::BadRequest().body("Invalid scale index"));
        }
    };

    // Prepare the scale name and notes for inclusion in the response
    let scale_name_with_count = format!("{} ({} Notes)", scale_name, scale_notes.len());
    let scale_notes_str = scale_notes
        .iter()
        .zip(scale_tpc.iter())
        .map(|(&midi_note, &tpc_note)| {
            let (note, octave) =
                crate::utils::scales::midi_to_note_and_octave_with_tpc(midi_note, tpc_note);
            format!("{}{}", note, octave)
        })
        .collect::<Vec<String>>()
        .join(", ");

    let scale_notes_slice: &[u8] = &scale_notes;

    // Load the HTML template for generating the response
    let template_path = "src/html/generate_tmpl.html";
    let mut template_file = match File::open(template_path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open template file: {:?}", e);
            GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return Ok(HttpResponse::InternalServerError().body("Failed to open template file"));
        }
    };

    // Read the content of the template file into a string
    let mut template_content = String::new();
    if let Err(e) = template_file.read_to_string(&mut template_content) {
        log::error!("Failed to read template file: {:?}", e);
        GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
        return Ok(HttpResponse::InternalServerError().body("Failed to read template file"));
    }

    // Parse the MSCX content to extract measures and apply transpositions and scale constraints
    let (measures, final_transposed_value) = match crate::templates::parser::parse_mscx_score(
        &mscx_content,
        part_id,
        scale_notes_slice,
        auto_transpose,
        transpose_value,
    ) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Failed to parse MSCX: {:?}", e);
            GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return Ok(HttpResponse::InternalServerError().body("Failed to parse MSCX"));
        }
    };

    // Load the SVG representation of the scale
    let buffer_svg = match crate::utils::svg::load_svg_for_scale(scale_notes.len()) {
        Ok(svg_content) => svg_content,
        Err(e) => {
            log::error!("Failed to load SVG: {:?}", e);
            GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return Ok(HttpResponse::InternalServerError().body("Failed to load SVG"));
        }
    };

    // Generate HTML content for the measures
    let measures_html =
        crate::templates::parser::generate_measures_html(measures, &buffer_svg, play_only_inscale);

    // Replace placeholders in the template with generated content and prepare the final response
    let response = template_content
        .replace("{{part_name}}", &part_name)
        .replace("{{scale_name}}", &scale_name_with_count)
        .replace("{{scale_notes}}", &scale_notes_str)
        .replace("{{measures}}", &measures_html)
        .replace("{{transposed_value}}", &final_transposed_value.to_string());

    // Decrement the generate counter and return the final HTML response
    GENERATE_COUNTER.fetch_sub(1, Ordering::SeqCst);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response))
}
