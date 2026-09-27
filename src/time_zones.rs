use std::f32::consts::{FRAC_PI_2, TAU};
use egui_autocomplete::AutoCompleteTextEdit;

    use chrono::{DateTime, Local, TimeZone, Timelike};
    use eframe::{egui, Frame};
    use chrono_tz::{Tz, TZ_VARIANTS};
    use eframe::egui::Ui;
use egui::RichText;
use crate::city_timezones::{timezone_of, CITY_TIMEZONES};

    pub struct TimeZones
    {
        pub search_query: String,
        local_tz: DateTime<Local>,
        local_tz_name: String,
        available_tz: Vec<Tz>,
        available_tz_names: Vec<String>,
        selected_tz : Vec<(String,Tz)>,



    }

    impl TimeZones
    {
        pub fn new() -> Self
        {
            let local_tz  = Local::now();
            let quary = "";
            let  available_tz = TZ_VARIANTS.to_vec();
            let selected_tz = vec![];
            let available_tz_names = CITY_TIMEZONES
                .iter()
                .map(|x| x.0.to_string())
                .collect();
            let full_tz = iana_time_zone::get_timezone()
                .expect("failed to get time zone");
            let tz_str : Vec<&str> = full_tz
                .split('/')
                .collect();
            let tz_str_name = tz_str[1].to_string();
            TimeZones{
                local_tz,
                search_query : quary.to_string(),
                available_tz,
                selected_tz,
                available_tz_names,
                local_tz_name : tz_str_name

            }
        }

        pub fn update(&mut self)
        {
            self.local_tz = Local::now();

        }
        pub fn updated_selected(&mut self)
        {
            if let Some(actual_query) = timezone_of(&self.search_query)
            {
                for item in &self.available_tz
                {
                    if item.name().contains(actual_query) && !self.selected_tz.contains(&(actual_query.to_string(), *item))
                    {
                        self.selected_tz.push((self.search_query.to_string(),item.clone()));
                    }
                }
            }

        }

    }

    impl eframe::App for TimeZones
    {
        fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {

            ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));
            self.update();


            egui::CentralPanel::default().show(ui, |ui|
                {
                    ui.heading("time zone calculator ");
                    ui.add_space(10.0);
                    ui.horizontal(|ui|
                        {
                            ui.label("local time: ");
                            ui.add_space(10.0);
                            ui.label(self.local_tz.time().format("%H:%M:%S").to_string());
                            ui.add_space(20.0);
                            ui.add(AutoCompleteTextEdit::new(
                                &mut self.search_query,
                                &self.available_tz_names,

                            )
                                .highlight_matches(true)
                                .set_text_edit_properties(|text_edit : egui::TextEdit|
                                    {
                                        text_edit
                                            .hint_text("search for a time zone a major city or content i.e Berlin , Asis/Jerusalm")

                                    })
                            );
                            if ui.button("add time zone").clicked()
                            {
                                self.updated_selected()
                            }

                        });
                    egui::ScrollArea::vertical().show(ui, |ui|
                        {
                            let _ = &self.selected_tz.retain(|item|
                                {
                                    let name = &item.0;
                                    let time = Local::now().with_timezone(&item.1);
                                    let time_str = time.format("%H:%M:%S").to_string();
                                    let final_str = format!("Time in {name} : {time_str}");

                                    ui.horizontal(|ui|
                                        {
                                            ui.label(
                                                RichText::new(final_str)
                                                    .size(20.0)
                                            );
                                            !ui.button("🗑️").clicked()



                                        })
                                        .inner
                                });


                        });

                });
        }
    }
    fn draw_clock(ui: &mut Ui, tz: DateTime<Tz>) {
        let desired_size = egui::vec2(100.0, 100.0);
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        let center = rect.center();
        let radius = 50.0;

        let time = tz.time();


        let hours_rad = (((time.hour() % 12) as f32 + time.minute() as f32 / 60.0) / 12.0) * TAU - FRAC_PI_2;
        let minutes_rad = ((time.minute() as f32 + time.second() as f32 / 60.0) / 60.0) * TAU - FRAC_PI_2;
        let seconds_rad = (time.second() as f32 / 60.0) * TAU - FRAC_PI_2;

        let painter = ui.painter();

        painter.circle_stroke(center, radius, egui::Stroke::new(2.0, egui::Color32::GRAY));

        let hour_end = center + egui::vec2(hours_rad.cos() * (radius * 0.5), hours_rad.sin() * (radius * 0.5));
        painter.line_segment([center, hour_end], egui::Stroke::new(3.0, egui::Color32::BLUE));

        let minute_end = center + egui::vec2(minutes_rad.cos() * (radius * 0.75), minutes_rad.sin() * (radius * 0.75));
        painter.line_segment([center, minute_end], egui::Stroke::new(2.0, egui::Color32::LIGHT_GRAY));

        let second_end = center + egui::vec2(seconds_rad.cos() * (radius * 0.9), seconds_rad.sin() * (radius * 0.9));
        painter.line_segment([center, second_end], egui::Stroke::new(1.0, egui::Color32::RED));
    }
