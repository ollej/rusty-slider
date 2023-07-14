pub fn set_clipboard(data: &str) {
    macroquad::miniquad::window::clipboard_set(data);
}
