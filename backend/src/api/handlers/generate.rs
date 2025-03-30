use crate::db::Database;
use crate::error::AppError;
use crate::utils::svg;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GenerateParams {
    play_only_inscale: Option<bool>,
}

pub async fn generate_tab_html(
    path: web::Path<(Uuid, i32)>,
    query: web::Query<GenerateParams>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppError> {
    let (tab_id, part_id) = path.into_inner();
    let play_only_inscale = query.play_only_inscale.unwrap_or(false);

    // Get tab from database
    let tab = db.get_tab(tab_id).await.map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Tab not found".to_string()))?;

    // Parse score data and get the part
    let score_data: crate::models::ScoreData = serde_json::from_value(tab.score_data)
        .map_err(|e| AppError::Parse(format!("Invalid score data: {}", e)))?;

    // Find the requested part
    let part = score_data.parts.into_iter()
        .find(|p| p.id == part_id as u32)
        .ok_or_else(|| AppError::NotFound("Part not found".to_string()))?;

    // Convert measures to the format expected by generate_measures_html
    let formatted_measures: Vec<(u32, String, Vec<Vec<(u32, String, String, i32, Option<usize>)>>)> = 
        part.measures.iter()
        .map(|measure| {
            let measure_number = measure.id;
            let time_signature = measure.time_signature
                .map(|(num, denom)| format!("{num}|{denom}"))
                .unwrap_or_else(|| "4|4".to_string());
            
            let chords = measure.chords.iter()
                .map(|notes| {
                    notes.iter()
                        .map(|note| {
                            (
                                note.pitch.as_int() as u32,
                                format!("note_{}", note.pitch), // Generate note name from pitch
                                note.duration.to_string(),
                                0, // No delta needed
                                None // No index needed
                            )
                        })
                        .collect()
                })
                .collect();

            (measure_number, time_signature, chords)
        })
        .collect();

    // Load SVG template
    let buffer_svg = svg::load_default_note_svg()
        .map_err(|e| AppError::Upload(format!("Failed to load SVG template: {}", e)))?;

    // Generate HTML
    let html = generate_measures_html(formatted_measures, &buffer_svg, play_only_inscale);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

/// Generates HTML for musical measures based on parsed score data and SVG templates.
fn generate_measures_html(
    measures: Vec<(
        u32,
        String,
        Vec<Vec<(u32, String, String, i32, Option<usize>)>>,
    )>,
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
        measures_html.push_str(&format!(
            "<div class='measure-header'>Measure: {}</div>\n",
            measure_num
        ));

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
                            if let Ok(svg_content) = svg::load_svg_for_rest(duration) {
                                svg_image = svg::modify_svg_note_color(
                                    &svg_content,
                                    420,
                                    &current_duration,
                                );
                            }
                        } else {
                            class_type = "handpansvg".to_string();
                            let (note_style, delta_display) = if *delta == 0 {
                                ("inscale", "".to_string())
                            } else if *delta > 0 {
                                ("outscale", format!("<span class='delta'>(<span class='delta_green'>{}</span>)</span>", delta))
                            } else {
                                ("outscale", format!("<span class='delta'>(<span class='delta_red'>{}</span>)</span>", delta))
                            };
                            note_formated.push_str(&format!(
                                "<span class='noteformated {}'>{}{}</span>",
                                note_style, note, delta_display
                            ));

                            let should_push_pitch = (!play_only_inscale && *delta != 0) || *delta == 0;
                            if should_push_pitch {
                                pitches.push(pitch);
                            }

                            let contains_zero_delta = notes.iter().any(|(_, _, _, delta, _)| *delta == 0);
                            if let Some(index) = note_index {
                                svg_image = svg::modify_svg_note_color(&svg_image, *index, duration);
                            } else if !contains_zero_delta {
                                svg_image = svg::modify_svg_note_color(&svg_image, 999, duration);
                            }
                        }
                    }

                    let pitches_data = pitches
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(";");
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
