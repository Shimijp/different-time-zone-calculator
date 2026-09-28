use egui_autocomplete::AutoCompleteTextEdit;
use std::f32::consts::{FRAC_PI_2, TAU};

use crate::city_timezones::{CITY_TIMEZONES, timezone_of};
use crate::theme;
use chrono::{DateTime, Local, Timelike};
use chrono_tz::{TZ_VARIANTS, Tz};
use eframe::egui::Ui;
use eframe::{Frame, egui};
use egui::RichText;

pub struct TimeZones {
    pub search_query: String,
    local_tz: DateTime<Local>,
    available_tz: Vec<Tz>,
    available_tz_names: Vec<String>,
    selected_tz: Vec<(String, Tz)>,
}

impl TimeZones {
    pub fn new() -> Self {
        let local_tz = Local::now();
        let quary = "";
        let available_tz = TZ_VARIANTS.to_vec();
        let selected_tz = vec![];
        let available_tz_names = CITY_TIMEZONES.iter().map(|x| x.0.to_string()).collect();
        TimeZones {
            local_tz,
            search_query: quary.to_string(),
            available_tz,
            selected_tz,
            available_tz_names,

        }
    }

    pub fn update(&mut self) {
        self.local_tz = Local::now();
    }
    pub fn updated_selected(&mut self) {
        if let Some(actual_query) = timezone_of(&self.search_query) {
            for item in &self.available_tz {
                if item.name().contains(actual_query)
                    && !self
                        .selected_tz
                        .contains(&(self.search_query.to_string(), *item))
                {
                    self.selected_tz
                        .push((self.search_query.to_string(), item.clone()));
                }
            }
        }
    }
}

impl eframe::App for TimeZones {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_secs(1));
        self.update();

        egui::CentralPanel::default().frame(theme::page_frame()).show(ui, |ui| {
            ui.heading("time zone calculator ");
            ui.add_space(14.0);
            theme::card_frame().show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("local time: ").color(theme::TEXT_MUTED));
                    ui.add_space(4.0);
                    ui.label(RichText::new(self.local_tz.time().format("%H:%M:%S").to_string())
                        .strong()
                        .color(theme::ACCENT)
                        .font(theme::digits(30.0))
                        );
                    ui.add_space(20.0);
                    ui.add(
                        AutoCompleteTextEdit::new(&mut self.search_query, &self.available_tz_names)
                            .highlight_matches(true)
                            .set_text_edit_properties(|text_edit: egui::TextEdit| {
                                text_edit
                                    .hint_text("🔍  search for a city/country")
                                    .desired_width(280.0)
                                    .margin(egui::Margin::symmetric(12, 10))
                            }),
                    );
                    if theme::primary_button(ui, "+  add time zone").clicked() {
                        self.updated_selected()
                    }
                });
            });
            ui.add_space(18.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 12.0;
                let _ = &self.selected_tz.retain(|item| {
                    let name = &item.0;
                    let time = Local::now().with_timezone(&item.1);
                    let time_str = time.format("%H:%M:%S").to_string();

                    ui.horizontal(|ui| {
                        theme::card_frame()
                            .show(ui, |ui| {
                                // leave room on the right for the delete button
                                ui.set_width(
                                    ui.available_width() - theme::ICON_BUTTON_SIZE - ui.spacing().item_spacing.x,
                                );
                                draw_clock(ui, time);
                                ui.add_space(10.0);
                                let clock_height = ui.min_rect().height();
                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 2.0;
                                    let lines = [theme::digits(34.0), egui::FontId::proportional(15.0)];
                                    ui.add_space(theme::centre_pad(ui, clock_height, &lines));
                                    ui.label(
                                        RichText::new(time_str)
                                            .font(lines[0].clone())
                                            .strong()
                                            .color(theme::TEXT),
                                    );
                                    ui.label(RichText::new(name).font(lines[1].clone()).color(theme::TEXT_MUTED));
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(6.0);
                                    match time.hour() {
                                        5..=6 => ui.label(RichText::new("🌅").size(30.0).color(theme::SUNRISE)),
                                        7..=17 => ui.label(RichText::new("☀️").size(30.0).color(theme::SUN)),
                                        18..=19 => ui.label(RichText::new("🌇").size(30.0).color(theme::SUNSET)),
                                        _ => ui.label(RichText::new("🌙").size(30.0).color(theme::MOON)),
                                    }
                                });
                            });

                        !theme::danger_icon_button(ui, "🗑️").clicked()
                    })
                    .inner
                });
            });
        });
        //press enter to search
        if ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
            self.updated_selected()
        }
    }
}
fn draw_clock(ui: &mut Ui, tz: DateTime<Tz>) {
    let desired_size = egui::vec2(88.0, 88.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    let center = rect.center();
    let radius = 41.0;

    let time = tz.time();

    let hours_rad =
        (((time.hour() % 12) as f32 + time.minute() as f32 / 60.0) / 12.0) * TAU - FRAC_PI_2;
    let minutes_rad =
        ((time.minute() as f32 + time.second() as f32 / 60.0) / 60.0) * TAU - FRAC_PI_2;
    let seconds_rad = (time.second() as f32 / 60.0) * TAU - FRAC_PI_2;

    let painter = ui.painter();
    let face = theme::ClockFace::for_hour(time.hour());

    // soft drop shadow, face, rim
    painter.circle_filled(center + egui::vec2(0.0, 2.0), radius + 1.5, egui::Color32::from_black_alpha(90));
    painter.circle_filled(center, radius, face.fill);
    painter.circle_stroke(center, radius, egui::Stroke::new(2.0, face.rim));

    // hour ticks, heavier at 12 / 3 / 6 / 9
    for i in 0..12 {
        let angle = i as f32 / 12.0 * TAU;
        let dir = egui::vec2(angle.cos(), angle.sin());
        let (inner, width) = if i % 3 == 0 { (0.74, 2.6) } else { (0.84, 1.4) };
        painter.line_segment(
            [center + dir * radius * inner, center + dir * radius * 0.92],
            egui::Stroke::new(width, face.ticks),
        );
    }

    let hour_end = center
        + egui::vec2(
            hours_rad.cos() * (radius * 0.5),
            hours_rad.sin() * (radius * 0.5),
        );
    painter.line_segment(
        [center, hour_end],
        egui::Stroke::new(4.5, face.hands),
    );
    painter.circle_filled(hour_end, 2.25, face.hands);

    let minute_end = center
        + egui::vec2(
            minutes_rad.cos() * (radius * 0.75),
            minutes_rad.sin() * (radius * 0.75),
        );
    painter.line_segment(
        [center, minute_end],
        egui::Stroke::new(3.0, face.hands),
    );
    painter.circle_filled(minute_end, 1.5, face.hands);

    let second_end = center
        + egui::vec2(
            seconds_rad.cos() * (radius * 0.9),
            seconds_rad.sin() * (radius * 0.9),
        );
    let second_tail = center - (second_end - center) * 0.2;
    painter.line_segment(
        [second_tail, second_end],
        egui::Stroke::new(1.5, theme::SECOND_HAND),
    );

    // centre cap
    painter.circle_filled(center, 3.5, theme::SECOND_HAND);
    painter.circle_filled(center, 1.3, face.fill);
}

