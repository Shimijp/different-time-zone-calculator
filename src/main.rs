use crate::time_zones::TimeZones;

mod time_zones;
mod city_timezones;

fn main() {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([480.0, 320.0])
            .with_title("time zones"),
        ..Default::default()
    };
    eframe::run_native(
        "time zone calculator",                       // app id, used for storage
        options,
        Box::new(|_cc| Ok(Box::new(TimeZones::new()))),
    )
        .expect("failed to run app")
}
