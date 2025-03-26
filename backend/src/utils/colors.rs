pub fn get_color_for_duration(duration: &str) -> Option<&'static str> {
    match duration {
        "whole" => Some("#FF0000"),     // Rouge
        "half" => Some("#00FF00"),      // Vert
        "quarter" => Some("#0000FF"),   // Bleu
        "eighth" => Some("#FFA500"),    // Orange
        "sixteenth" => Some("#800080"), // Violet
        _ => None,
    }
}
