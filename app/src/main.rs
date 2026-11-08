// path: app/src/main.rs
#[cfg(not(target_os = "android"))]
fn main() {
    ludamob_window::run();
}

#[cfg(target_os = "android")]
fn main() {
    // На Android используется android_main из lib (cdylib).
    // Этот бинарь не запускается на Android.
}
