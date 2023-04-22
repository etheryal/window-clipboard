use std::error::Error;

use clipboard_win::{get_clipboard_string, set_clipboard_string};
use raw_window_handle::HasRawDisplayHandle;

use crate::ClipboardProvider;

pub fn connect<W: HasRawDisplayHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider + Send + Sync>, Box<dyn Error>> {
    Ok(Box::new(Clipboard))
}

pub struct Clipboard;

impl ClipboardProvider for Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(get_clipboard_string()?)
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        Ok(set_clipboard_string(&contents)?)
    }
}
