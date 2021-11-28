use std::sync::atomic::{AtomicUsize, Ordering};

pub static DEBUG: bool = true;

pub static BRAIN_SIZE: u8 = 50;

pub static INPUTS_SIZE: u8 = 4;

pub static OUTPUTS_SIZE: u8 = 5;

pub static WINDOW_SIZE: (AtomicUsize, AtomicUsize) = (
    AtomicUsize::new(800), AtomicUsize::new(640)
);

pub static WINDOW_CENTER: (AtomicUsize, AtomicUsize) = (
    AtomicUsize::new(400), AtomicUsize::new(320)
);

pub fn get_window_width() -> usize {
    return WINDOW_SIZE.0.load(Ordering::SeqCst);
}

pub fn get_window_height() -> usize {
    return WINDOW_SIZE.1.load(Ordering::SeqCst);
}

pub fn set_window_width(width: usize) {
    WINDOW_SIZE.0.store(width, Ordering::SeqCst);
    WINDOW_CENTER.0.store(width / 2, Ordering::SeqCst);
}

pub fn set_window_height(height: usize) {
    WINDOW_SIZE.1.store(height, Ordering::SeqCst);
    WINDOW_CENTER.1.store(height / 2, Ordering::SeqCst);
}