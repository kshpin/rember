use clipboard_rs::{Clipboard, ClipboardContext};
use std::sync::{Arc, LazyLock, Mutex};

// there's only one clipboard on the system, so a global instance is fine
static CLIPBOARD: LazyLock<Arc<Mutex<ClipboardContext>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(
        ClipboardContext::new().expect("clipboard not supported on this platform"),
    ))
});

pub fn get_text() -> String {
    let clipboard = CLIPBOARD.lock().expect("failed to lock clipboard");
    clipboard.get_text().expect("failed to get clipboard text")
}

pub fn set_text(text: String) {
    let clipboard = CLIPBOARD.lock().expect("failed to lock clipboard");
    clipboard
        .set_text(text)
        .expect("failed to set clipboard text");
}
