use crate::utils::logging::log_error;
use crate::utils::{
    scales::find_best_transposition_with_harmonic_context,
    scales::midi_to_note_and_octave_with_tpc, scales::transpose_pitch_and_tpc,
};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

/**
 * Extracts text content from the current position in the XML reader.
 *
 * This function:
 *
 * 1. **Loops Through Events**: Continuously reads events from the XML reader.
 * 2. **Handles Text Events**: When encountering a `Text` event, it extracts and returns the text content.
 * 3. **Handles End of Element**: Returns `None` when an `End` event is encountered.
 * 4. **Handles EOF**: Exits the loop if the end of the file (EOF) is reached.
 * 5. **Handles Errors**: If an error occurs during reading, it is returned as a boxed error.
 * 6. **Returns**: The extracted text as an `Option<String>`, or `None` if no text is found.
 *
 * @param reader A mutable reference to an XML `Reader` object.
 * @return A `Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>` containing the extracted text or an error.
 */
pub fn extract_text<R: std::io::BufRead>(
    reader: &mut Reader<R>,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => return Ok(Some(e.unescape()?.to_string())),
            Ok(Event::End(_)) => return Ok(None),
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(None)
}

/**
 * Parses metadata from an MSCX file, extracting the work title, composer, and arranger.
 *
 * This function:
 *
 * 1. **Initializes Default Values**: Sets the default values for composer, arranger, and work title as "Unknown".
 * 2. **Loops Through XML Events**: Reads the XML content event by event.
 * 3. **Identifies Meta Tags**: Looks for `metaTag` elements and extracts the `name` and associated value.
 * 4. **Assigns Values**: Updates the work title, composer, or arranger based on the extracted data.
 * 5. **Handles EOF**: Breaks the loop when the end of the file (EOF) is reached.
 * 6. **Logs Errors**: Logs any errors encountered during the parsing process.
 * 7. **Returns**: A tuple containing the work title, composer, and arranger as `String`s.
 *
 * @param xml_content The XML content of the MSCX file as a `&str`.
 * @return A tuple `(String, String, String)` containing the work title, composer, and arranger.
 */
pub fn parse_mscx_metadata(xml_content: &str) -> (String, String, String) {
    let mut reader = Reader::from_str(xml_content);
    let mut buf = Vec::new();

    let mut composer = String::from("Unknown");
    let mut arranger = String::from("Unknown");
    let mut work_title = String::from("Unknown");

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == QName(b"metaTag") => {
                let mut name = None;
                let mut value = None;
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key {
                        QName(b"name") => {
                            name = Some(attr.unescape_value().unwrap_or_default().to_string());
                        }
                        _ => {}
                    }
                }
                if let Some(name) = name {
                    match name.as_str() {
                        "composer" => value = Some(&mut composer),
                        "arranger" => value = Some(&mut arranger),
                        "workTitle" => value = Some(&mut work_title),
                        _ => {}
                    }
                }
                if let Some(value) = value {
                    if let Ok(Event::Text(e)) = reader.read_event_into(&mut buf) {
                        *value = e.unescape().unwrap_or_else(|_| "Unknown".into()).into_owned();
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                log_error("Error while parsing XML: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    (work_title, composer, arranger)
}

/**
 * Parses the part names and their corresponding staff IDs from an MSCX file.
 *
 * This function:
 *
 * 1. **Loops Through XML Events**: Reads the XML content event by event.
 * 2. **Identifies Parts**: Detects `Part` elements and extracts the `trackName` and `Staff` IDs.
 * 3. **Handles Multi-Staff Parts**: Differentiates between Treble and Bass for parts with two staffs.
 * 4. **Handles EOF**: Breaks the loop when the end of the file (EOF) is reached.
 * 5. **Handles Errors**: Returns any errors encountered during parsing.
 * 6. **Returns**: A `Result` containing a vector of tuples with the staff ID and part name, or an error.
 *
 * @param xml_content The XML content of the MSCX file as a `&str`.
 * @return A `Result<Vec<(u32, String)>, Box<dyn std::error::Error + Send + Sync>>` containing the parsed parts or an error.
 */
pub fn parse_mscx_parts(
    xml_content: &str,
) -> Result<Vec<(u32, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut reader = Reader::from_str(xml_content);
    let mut buf = Vec::new();
    let mut parts = Vec::new();
    let mut current_part_name: Option<String> = None;
    let mut current_staff_ids: Vec<u32> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            // Detect the start of a <Part> block
            Ok(Event::Start(ref e)) if e.name() == QName(b"Part") => {
                // Reset for the new part
                current_part_name = None;
                current_staff_ids.clear();
            }
            // Detect the end of a <Part> block
            Ok(Event::End(ref e)) if e.name() == QName(b"Part") => {
                // Associate all collected staff IDs with the current part name
                if let Some(ref part_name) = current_part_name {
                    if current_staff_ids.len() == 2 {
                        // If two staff IDs, label them as Treble and Bass
                        parts.push((current_staff_ids[0], format!("{} (Treble)", part_name)));
                        parts.push((current_staff_ids[1], format!("{} (Bass)", part_name)));
                    } else {
                        // Otherwise, just use the part name
                        for &staff_id in &current_staff_ids {
                            parts.push((staff_id, part_name.clone()));
                        }
                    }
                }
                current_part_name = None;
                current_staff_ids.clear();
            }
            // Detect the <trackName> inside a <Part> block
            Ok(Event::Start(ref e)) if e.name() == QName(b"trackName") => {
                // Extract the text content of <trackName>
                if let Ok(Some(name)) = extract_text(&mut reader) {
                    current_part_name = Some(name);
                }
            }
            // Detect the <Staff> element
            Ok(Event::Start(ref e)) if e.name() == QName(b"Staff") => {
                let mut staff_id = None;

                // Extract the "id" attribute from <Staff>
                for attr in e.attributes().filter_map(Result::ok) {
                    if attr.key == QName(b"id") {
                        staff_id = match attr.unescape_value()?.parse::<u32>() {
                            Ok(id) => Some(id),
                            Err(err) => return Err(Box::new(err)),
                        };
                    }
                }

                // Collect the staff ID to associate later
                if let Some(id) = staff_id {
                    current_staff_ids.push(id);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(parts)
}

/**
 * Parses the musical score from an MSCX file, handling transposition and scale matching.
 *
 * This function:
 *
 * 1. **Initializes Variables**: Sets up necessary variables to track measures, notes, and other score data.
 * 2. **Loops Through XML Events**: Reads the XML content event by event.
 * 3. **Handles Staff Identification**: Identifies the relevant staff based on the provided `part_id`.
 * 4. **Processes Measures and Chords**: Extracts and processes measure and chord information, including transposition.
 * 5. **Handles Transposition**: Automatically transposes notes if required, and finds the best matching notes in the handpan scale.
 * 6. **Handles EOF**: Breaks the loop when the end of the file (EOF) is reached.
 * 7. **Returns**: A `Result` containing the parsed measures and final transposed value, or an error.
 *
 * @param xml_content The XML content of the MSCX file as a `&str`.
 * @param part_id The ID of the part to be parsed.
 * @param scale_notes A slice of bytes representing the notes in the handpan scale.
 * @param auto_transpose A boolean indicating whether to auto-transpose notes.
 * @param transpose_value The value by which to transpose the notes.
 * @return A `Result` containing a vector of measures and the final transposed value, or an error.
 */
pub fn parse_mscx_score(
    xml_content: &str,
    part_id: u32,
    scale_notes: &[u8],
    auto_transpose: bool,
    transpose_value: i32,
) -> Result<
    (Vec<(u32, String, Vec<Vec<(u32, String, String, i32, Option<usize>)>>)>, i32),
    Box<dyn std::error::Error + Send + Sync>,
> {
    let mut reader = Reader::from_str(xml_content);
    let mut buf = Vec::new();
    let mut measures = Vec::new();
    let mut in_correct_staff = false;
    let mut current_duration: Option<String> = None;
    let mut current_time_signature = String::new();
    let mut all_notes = Vec::new();
    let mut measure_chords = Vec::new();
    let mut current_chord_notes = Vec::new();
    let mut final_transposed_value = transpose_value;
    let mut mesure_id = 0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) if e.name() == QName(b"Staff") => {
                if let Some(id) = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .find(|a| a.key == QName(b"id"))
                    .and_then(|a| a.unescape_value().ok())
                    .and_then(|id_str| id_str.parse::<u32>().ok())
                {
                    if id == part_id {
                        in_correct_staff = true;
                    }
                }
            }
            Event::End(ref e) if e.name() == QName(b"Staff") => {
                if in_correct_staff {
                    in_correct_staff = false;
                }
            }
            Event::Start(ref e) if e.name() == QName(b"Measure") && in_correct_staff => {
                mesure_id += 1;
                measures.push((mesure_id, String::new(), Vec::new()));
                current_time_signature.clear(); // Reset the time signature for the new measure
                measure_chords.clear(); // Reset chords for the new measure
            }
            Event::End(ref e) if e.name() == QName(b"Measure") && in_correct_staff => {
                if let Some((_, time_sig, chords)) = measures.last_mut() {
                    *time_sig = current_time_signature.clone();
                    *chords = measure_chords.clone(); // Add the collected chords to the measure
                }
            }
            Event::Start(ref e) if e.name() == QName(b"TimeSig") && in_correct_staff => {
                let mut sig_n = String::new();
                let mut sig_d = String::new();

                // Extract time signature numbers
                loop {
                    match reader.read_event_into(&mut buf)? {
                        Event::Start(ref e) if e.name() == QName(b"sigN") => {
                            if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                                sig_n = text.unescape()?.trim().to_string();
                            }
                        }
                        Event::Start(ref e) if e.name() == QName(b"sigD") => {
                            if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                                sig_d = text.unescape()?.trim().to_string();
                            }
                        }
                        Event::End(ref e) if e.name() == QName(b"TimeSig") => {
                            break;
                        }
                        _ => {}
                    }
                }

                // Format and store the time signature
                current_time_signature = format!("{}|{}", sig_n, sig_d);
            }
            Event::Start(ref e) if e.name() == QName(b"Chord") && in_correct_staff => {
                // Extract the duration when inside a Chord
                current_duration = None; // Reset the duration at the start of each Chord
                current_chord_notes.clear(); // Reset notes for the current chord
            }
            Event::End(ref e) if e.name() == QName(b"Chord") && in_correct_staff => {
                // Add the collected notes to the chord list
                if !current_chord_notes.is_empty() {
                    measure_chords.push(current_chord_notes.clone());
                }
            }
            Event::Start(ref e) if e.name() == QName(b"Rest") && in_correct_staff => {
                // Extract the duration when inside a Rest
                current_duration = None; // Reset the duration at the start of each Rest
                current_chord_notes.clear(); // Reset notes for the current Rest
            }
            Event::End(ref e) if e.name() == QName(b"Rest") && in_correct_staff => {
                // Add the collected notes to the Rest list
                if let Some(ref duration) = current_duration {
                    let note_info = (0, "Rest".to_string(), duration.clone(), 0, None);
                    current_chord_notes.push(note_info);
                    measure_chords.push(current_chord_notes.clone());
                }
            }
            Event::Start(ref e) if e.name() == QName(b"durationType") && in_correct_staff => {
                // Read the durationType value inside a Chord
                if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                    current_duration = Some(text.unescape()?.trim().to_string());
                }
            }
            Event::Start(ref e) if e.name() == QName(b"Note") && in_correct_staff => {
                let mut pitch: Option<u8> = None;
                let mut tpc: Option<i8> = None;

                // Extract pitch inside the Note element
                loop {
                    match reader.read_event_into(&mut buf)? {
                        Event::Start(ref e) if e.name() == QName(b"pitch") => {
                            if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                                pitch = text.unescape()?.trim().parse::<u8>().ok();
                                if let Some(p) = pitch {
                                    all_notes.push(p);
                                }
                            }
                        }
                        Event::Start(ref e) if e.name() == QName(b"tpc") => {
                            if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                                tpc = text.unescape()?.trim().parse::<i8>().ok();
                            }
                        }
                        Event::End(ref e) if e.name() == QName(b"Note") => {
                            break;
                        }
                        _ => {}
                    }
                }

                if let Some(pitch) = pitch {
                    let (transposed_pitch, transposed_tpc) = if auto_transpose {
                        let best_transpose_value =
                            find_best_transposition_with_harmonic_context(&all_notes, scale_notes);
                        final_transposed_value = best_transpose_value;
                        // Call the function to transpose the pitch and TPC
                        transpose_pitch_and_tpc(pitch, tpc, best_transpose_value).unwrap()
                    } else {
                        final_transposed_value = transpose_value;
                        transpose_pitch_and_tpc(pitch, tpc, transpose_value).unwrap()
                    };

                    let (note, octave) =
                        midi_to_note_and_octave_with_tpc(transposed_pitch, transposed_tpc);
                    let note_with_octave = format!("{}{}", note, octave);

                    // Calculate the delta and find the index of the closest note in the handpan scale
                    let mut closest_index = None;
                    let mut min_delta = i32::MAX;
                    for (i, &s_note) in scale_notes.iter().enumerate() {
                        let current_delta = transposed_pitch as i32 - s_note as i32;
                        if current_delta.abs() < min_delta.abs() {
                            // Compare absolute values to find the smallest difference
                            min_delta = current_delta; // Keep the actual signed delta
                            closest_index = Some(i);
                        }
                    }

                    let delta = min_delta;

                    if let Some(ref duration) = current_duration {
                        let note_info = if delta == 0 {
                            (
                                transposed_pitch.clone() as u32,
                                note_with_octave,
                                duration.clone(),
                                delta,
                                Some(closest_index.unwrap()),
                            )
                        } else {
                            (
                                transposed_pitch.clone() as u32,
                                note_with_octave,
                                duration.clone(),
                                delta,
                                None,
                            )
                        };
                        current_chord_notes.push(note_info);
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear(); // Clear the buffer at the end of the loop iteration
    }
    Ok((measures, final_transposed_value))
}

/**
 * Generates HTML for musical measures based on parsed score data and SVG templates.
 *
 * This function:
 *
 * 1. **Initializes HTML Structure**: Sets up the initial HTML structure for measures.
 * 2. **Loops Through Measures**: Iterates over the provided measures, processing time signatures and chords.
 * 3. **Handles Note Formatting**: Formats each note, applying transpositions and determining color codes.
 * 4. **Modifies SVGs**: Adjusts SVG images for notes and rests based on their pitch and duration.
 * 5. **Generates HTML Output**: Builds the HTML string for each measure, incorporating formatted notes and time signatures.
 * 6. **Returns**: The complete HTML for all the measures.
 *
 * @param measures A vector of measures containing the parsed score data.
 * @param buffer_svg A reference to the SVG template to be used for notes.
 * @param play_only_inscale A boolean indicating whether to display only in-scale notes.
 * @return A `String` containing the generated HTML for the measures.
 */
pub fn generate_measures_html(
    measures: Vec<(u32, String, Vec<Vec<(u32, String, String, i32, Option<usize>)>>)>,
    buffer_svg: &str,
    play_only_inscale: bool,
) -> String {
    let mut measures_html = String::new();
    let mut current_sign = String::new();
    let mut current_sigb = String::new();

    for (measure_num, time_signature, chords) in measures {
        if !time_signature.is_empty() {
            let sig: Vec<&str> = time_signature.split('|').collect();
            current_sign = sig.get(0).unwrap_or(&"default").to_string();
            current_sigb = sig.get(1).unwrap_or(&"default").to_string();

            measures_html.push_str("<div class='measure'>\n");
            measures_html.push_str("<div class='signature'>\n");
            measures_html.push_str(&format!("<div class='sigN'>{}</div>\n", current_sign));
            measures_html.push_str(&format!("<div class='sigD'>{}</div>\n", current_sigb));
            measures_html.push_str("</div>\n");
            measures_html.push_str("</div>\n");
        }

        measures_html.push_str("<div class='measure'>\n");
        measures_html
            .push_str(&format!("<div class='measure-header'>Measure: {}</div>\n", measure_num));

        if !chords.is_empty() {
            measures_html.push_str("<div class='notes'>\n");

            for notes in chords.iter() {
                if !notes.is_empty() {
                    let mut svg_image = buffer_svg.to_string();
                    let mut note_formated = String::new();
                    let mut class_type = String::new();
                    let mut current_duration = String::new();
                    let mut pitches: Vec<&u32> = Vec::new();

                    for (pitch, note, duration, delta, note_index) in notes {
                        if duration == "measure" {
                            current_duration = "whole".to_string();
                        } else {
                            current_duration = duration.to_string();
                        }

                        if note == "Rest" {
                            pitches.push(pitch);
                            class_type = "restsvg".to_string();
                            note_formated = String::new();
                            match crate::utils::svg::load_svg_for_rest(duration) {
                                Ok(svg_content) => {
                                    svg_image = crate::utils::svg::modify_svg_note_color(
                                        &svg_content,
                                        420,
                                        &current_duration,
                                    );
                                }
                                Err(e) => {
                                    log::error!("Failed to load SVG: {:?}", e);
                                }
                            }
                        } else {
                            class_type = "handpansvg".to_string();
                            let (note_style, delta_display) = if *delta == 0 {
                                ("inscale", "".to_string()) // String
                            } else if *delta > 0 {
                                ("outscale", format!("<span class='delta'>(<span class='delta_green'>{}</span>)</span>", delta))
                            // String
                            } else {
                                ("outscale", format!("<span class='delta'>(<span class='delta_red'>{}</span>)</span>", delta))
                                // String
                            };
                            note_formated.push_str(&format!(
                                "<span class='noteformated {}'>{}{}</span>",
                                note_style, note, delta_display
                            ));

                            let should_push_pitch =
                                (!play_only_inscale && *delta != 0) || *delta == 0;
                            if should_push_pitch {
                                pitches.push(pitch);
                            }

                            let contains_zero_delta =
                                notes.iter().any(|(_, _, _, delta, _)| *delta == 0);
                            if let Some(index) = note_index {
                                svg_image = crate::utils::svg::modify_svg_note_color(
                                    &svg_image, *index, &duration,
                                );
                            } else if !contains_zero_delta {
                                svg_image = crate::utils::svg::modify_svg_note_color(
                                    &svg_image, 999, &duration,
                                );
                            }
                        }
                    }

                    let pitches_data =
                        pitches.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(";");
                    measures_html.push_str(&format!(
                        "<div class='note' sigN='{}' sigD='{}' pitches='{}' duration='{}'><div class='svg_container {}'>{}</div><div class='note-label'>{}</div></div>\n",
                        current_sign, current_sigb, pitches_data, current_duration, class_type, svg_image, note_formated
                    ));
                }
            }
            measures_html.push_str("</div>\n");
        }
        measures_html.push_str("</div>\n");
    }

    measures_html
}
