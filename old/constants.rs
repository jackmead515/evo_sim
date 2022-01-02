use std::sync::atomic::{AtomicUsize, AtomicIsize, Ordering};

pub static DEBUG: bool = true;

pub static BRAIN_SIZE: u8 = 20;

pub static INPUTS_SIZE: u8 = 4;

pub static OUTPUTS_SIZE: u8 = 7;

pub static WINDOW_SIZE: (AtomicIsize, AtomicIsize) = (
    AtomicIsize::new(800), AtomicIsize::new(640)
);

pub static WINDOW_CENTER: (AtomicIsize, AtomicIsize) = (
    AtomicIsize::new(400), AtomicIsize::new(320)
);

pub fn get_window_width() -> isize {
    return WINDOW_SIZE.0.load(Ordering::SeqCst);
}

pub fn get_window_height() -> isize {
    return WINDOW_SIZE.1.load(Ordering::SeqCst);
}

pub fn set_window_width(width: isize) {
    WINDOW_SIZE.0.store(width, Ordering::SeqCst);
    WINDOW_CENTER.0.store(width / 2, Ordering::SeqCst);
}

pub fn set_window_height(height: isize) {
    WINDOW_SIZE.1.store(height, Ordering::SeqCst);
    WINDOW_CENTER.1.store(height / 2, Ordering::SeqCst);
}