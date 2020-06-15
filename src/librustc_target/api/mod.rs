pub mod windows;

pub fn get_enabled_target_api_features(
    target_kind: &str,
    requested_apis: &[String],
) -> Vec<&'static str> {
    match target_kind {
        "windows" => windows::get_enabled_target_api_features(requested_apis),
        _ => vec![],
    }
}
