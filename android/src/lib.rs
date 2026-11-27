//! Android entry point for BestRS.
//!
//! This crate holds only the Android-specific shell: `android_main` and logging. The
//! product logic lives in the shared `ytm_rs` library, which this crate links in later
//! phases. Task 00 is a toolchain smoke test, so the screen is deliberately empty apart
//! from its name.
#![cfg(target_os = "android")]

slint::include_modules!();

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("bestrs"),
    );

    // android-activity initializes ndk-context with the JavaVM and the Application
    // context before it calls android_main. Calling
    // `ndk_context::initialize_android_context` here would assert because it is already
    // set; cpal/AAudio and the JNI helpers can use it from this point on.

    if let Err(error) = slint::android::init(app) {
        log::error!("Slint Android backend failed: {error}");
        return;
    }

    let window = match MainWindow::new() {
        Ok(window) => window,
        Err(error) => {
            log::error!("could not create the Slint window: {error}");
            return;
        }
    };
    if let Err(error) = window.run() {
        log::error!("Slint event loop failed: {error}");
    }
}
