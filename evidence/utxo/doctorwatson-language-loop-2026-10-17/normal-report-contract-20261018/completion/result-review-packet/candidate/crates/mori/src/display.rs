/// Escape data before a terminal renderer adds its trusted styles and layout.
pub fn text(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for character in value.chars() {
        if character.is_ascii() && !character.is_control() {
            result.push(character);
        } else {
            result.extend(character.escape_default());
        }
    }
    result
}
