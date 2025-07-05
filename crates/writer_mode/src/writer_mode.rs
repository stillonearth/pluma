use gpui::{App, actions};

pub mod editor_writer_mode;
pub mod writer_mode_status_button;

pub fn init(cx: &mut App) {
    editor_writer_mode::init(cx);
}

actions!(writer_mode, [ToggleWriterMode,]);
