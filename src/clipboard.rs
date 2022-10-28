use macroquad::window::get_internal_gl;

pub fn set_clipboard(data: &str) {
    let gl = unsafe { get_internal_gl() };
    let ctx = gl.quad_context;
    ctx.clipboard_set(data);
}
