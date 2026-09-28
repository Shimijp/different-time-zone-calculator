//! Embeds the app icon (and file details) into the Windows .exe,
//! so Explorer, shortcuts and a pinned taskbar entry show it.
//! The icon in the window title bar is set at runtime in main.rs.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/icon.ico");

    // Build scripts run on the host, so check the *target* OS rather than cfg!(windows).
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/icon.ico")
            .set("FileDescription", "time zone calculator")
            .set("ProductName", "time zone calculator")
            .compile()
            .expect("failed to embed assets/icon.ico (is windres / rc.exe on PATH?)");
    }
}
