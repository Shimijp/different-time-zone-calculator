use crate::time_zones::TimeZones;

mod time_zones;
mod city_timezones;
mod theme;

fn main() {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([480.0, 320.0])
            .with_title("time zones")
            // title bar / taskbar icon (build.rs embeds the same icon into the .exe)
            .with_icon(
                eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
                    .expect("assets/icon.png is not a valid png"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "time zone calculator",                       // app id, used for storage
        options,
        Box::new(|cc| {
            theme::install(&cc.egui_ctx);
            Ok(Box::new(TimeZones::new()))
        }),
    )
        .expect("failed to run app")
}
