fn main() {
    let manifest_dir =
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let main_slint = manifest_dir.join("../ui/android/main.slint");
    let palette_slint = manifest_dir.join("../ui/android/palette.slint");

    println!("cargo:rerun-if-changed={}", main_slint.display());
    println!("cargo:rerun-if-changed={}", palette_slint.display());

    // 16 KB page sizes are required by Android 15+ and by Google Play. The NDK r27
    // still defaults to 4 KB, so ask the linker explicitly.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
    }

    slint_build::compile(&main_slint).expect("failed to compile ui/android/main.slint");
}
