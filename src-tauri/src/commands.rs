
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}



#[derive(Debug,   serde::Serialize)]
pub struct CharacterCount {
    total_chars: usize,
    without_newlines: usize,
    without_newlines_spaces: usize,
}

#[tauri::command]
pub fn count_characters(text: &str) -> CharacterCount {
    let total_chars = text.chars().count();
    let without_newlines = text.chars().filter(|&c| c != '\n').count();
    let without_newlines_spaces = text.chars().filter(|&c| c != '\n' && c != ' ').count();
    CharacterCount {
        total_chars,
        without_newlines,
        without_newlines_spaces,
    }
}