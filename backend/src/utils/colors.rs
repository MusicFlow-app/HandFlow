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
