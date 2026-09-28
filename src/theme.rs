//! Look & feel only: fonts, colours and a couple of styled buttons.
//! Nothing in here touches app state.

use std::sync::Arc;

use egui::{
    Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin, Response, RichText,
    Shadow, Stroke, TextStyle, Ui, Vec2, vec2,
};

// ---------------------------------------------------------------- palette

pub const BG: Color32 = Color32::from_rgb(0x0B, 0x0F, 0x17);
pub const SURFACE: Color32 = Color32::from_rgb(0x14, 0x1A, 0x26);
pub const SURFACE_HI: Color32 = Color32::from_rgb(0x1C, 0x24, 0x34);
pub const INPUT_BG: Color32 = Color32::from_rgb(0x0F, 0x14, 0x1E);
pub const BORDER: Color32 = Color32::from_rgb(0x25, 0x2E, 0x41);
pub const BORDER_HI: Color32 = Color32::from_rgb(0x3A, 0x46, 0x5E);

pub const TEXT: Color32 = Color32::from_rgb(0xE8, 0xEC, 0xF3);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x8A, 0x94, 0xA8);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x62, 0x6C, 0x80);

pub const ACCENT: Color32 = Color32::from_rgb(0xFF, 0xB2, 0x3F);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0xFF, 0xC4, 0x6B);
pub const ACCENT_PRESSED: Color32 = Color32::from_rgb(0xE8, 0x9A, 0x26);
pub const ON_ACCENT: Color32 = Color32::from_rgb(0x1E, 0x14, 0x03);

pub const DANGER: Color32 = Color32::from_rgb(0xFF, 0x5F, 0x6D);

// time-of-day icon tints
pub const SUNRISE: Color32 = Color32::from_rgb(0xFF, 0x9E, 0x5E);
pub const SUN: Color32 = ACCENT;
pub const SUNSET: Color32 = Color32::from_rgb(0xFF, 0x7A, 0x8A);
pub const MOON: Color32 = Color32::from_rgb(0xA9, 0xB8, 0xFF);

// analog clock
pub const SECOND_HAND: Color32 = Color32::from_rgb(0xFF, 0x5A, 0x3C);

/// Colours for an analog clock face. Light face by day, dark face by night.
pub struct ClockFace {
    pub fill: Color32,
    pub rim: Color32,
    pub ticks: Color32,
    pub hands: Color32,
}

impl ClockFace {
    pub fn for_hour(hour: u32) -> Self {
        if (6..18).contains(&hour) {
            ClockFace {
                fill: Color32::from_rgb(0xF5, 0xF2, 0xEA),
                rim: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                ticks: Color32::from_rgb(0x3A, 0x40, 0x4C),
                hands: Color32::from_rgb(0x14, 0x18, 0x20),
            }
        } else {
            ClockFace {
                fill: Color32::from_rgb(0x07, 0x0A, 0x11),
                rim: Color32::from_rgb(0x2E, 0x38, 0x4E),
                ticks: Color32::from_rgb(0x7C, 0x87, 0xA0),
                hands: Color32::from_rgb(0xEE, 0xF1, 0xF7),
            }
        }
    }
}

// ---------------------------------------------------------------- fonts

const SEMIBOLD: &str = "semibold";
const BOLD: &str = "bold";
const DIGITS: &str = "digits";

/// Monospaced bold digits, so the time doesn't jiggle as it ticks.
pub fn digits(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(DIGITS.into()))
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMIBOLD.into()))
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    for (name, bytes) in [
        ("Inter-Regular", &include_bytes!("../assets/fonts/Inter-Regular.ttf")[..]),
        ("Inter-SemiBold", &include_bytes!("../assets/fonts/Inter-SemiBold.ttf")[..]),
        ("Inter-Bold", &include_bytes!("../assets/fonts/Inter-Bold.ttf")[..]),
        ("JetBrainsMono-Bold", &include_bytes!("../assets/fonts/JetBrainsMono-Bold.ttf")[..]),
    ] {
        fonts
            .font_data
            .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    }

    // egui's built-in fonts stay behind ours as fallbacks, so the emoji keep working.
    let fallbacks = fonts.families[&FontFamily::Proportional].clone();
    let with_fallbacks = |primary: &str| {
        let mut family = vec![primary.to_owned()];
        family.extend(fallbacks.iter().cloned());
        family
    };

    fonts
        .families
        .insert(FontFamily::Proportional, with_fallbacks("Inter-Regular"));
    fonts
        .families
        .insert(FontFamily::Name(SEMIBOLD.into()), with_fallbacks("Inter-SemiBold"));
    fonts
        .families
        .insert(FontFamily::Name(BOLD.into()), with_fallbacks("Inter-Bold"));
    fonts
        .families
        .insert(FontFamily::Name(DIGITS.into()), with_fallbacks("JetBrainsMono-Bold"));

    ctx.set_fonts(fonts);
}

// ---------------------------------------------------------------- style

/// Call once at startup.
pub fn install(ctx: &egui::Context) {
    install_fonts(ctx);
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, apply_style);
}

fn apply_style(style: &mut egui::Style) {
    style.text_styles = [
        (TextStyle::Heading, FontId::new(26.0, FontFamily::Name(BOLD.into()))),
        (TextStyle::Body, FontId::proportional(15.0)),
        (TextStyle::Button, semibold(15.0)),
        (TextStyle::Small, FontId::proportional(12.0)),
        (TextStyle::Monospace, FontId::monospace(14.0)),
    ]
    .into();

    let spacing = &mut style.spacing;
    spacing.item_spacing = vec2(10.0, 10.0);
    spacing.button_padding = vec2(16.0, 9.0);
    spacing.interact_size.y = 38.0;
    spacing.menu_margin = Margin::same(6);
    // scrollbar takes its own lane instead of floating over the delete buttons
    spacing.scroll = egui::style::ScrollStyle::solid();
    spacing.scroll.bar_inner_margin = 8.0;

    let visuals = &mut style.visuals;
    visuals.panel_fill = BG;
    visuals.window_fill = SURFACE;
    visuals.window_stroke = Stroke::new(1.0, BORDER);
    visuals.window_corner_radius = 12.into();
    visuals.menu_corner_radius = 12.into();
    visuals.popup_shadow = Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(140),
    };
    visuals.extreme_bg_color = INPUT_BG;
    visuals.text_edit_bg_color = Some(INPUT_BG);
    visuals.faint_bg_color = SURFACE;
    visuals.weak_text_color = Some(TEXT_FAINT);
    visuals.hyperlink_color = ACCENT;
    visuals.selection.bg_fill = ACCENT.linear_multiply(0.28);
    visuals.selection.stroke = Stroke::new(1.5, ACCENT);
    visuals.text_cursor.stroke = Stroke::new(2.0, ACCENT);

    let widgets = &mut visuals.widgets;

    widgets.noninteractive.bg_fill = SURFACE;
    widgets.noninteractive.weak_bg_fill = SURFACE;
    widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);
    widgets.noninteractive.corner_radius = 10.into();

    widgets.inactive.bg_fill = SURFACE_HI;
    widgets.inactive.weak_bg_fill = SURFACE_HI;
    widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    widgets.inactive.fg_stroke = Stroke::new(1.5, TEXT);
    widgets.inactive.corner_radius = 10.into();

    widgets.hovered.bg_fill = Color32::from_rgb(0x24, 0x2E, 0x42);
    widgets.hovered.weak_bg_fill = Color32::from_rgb(0x24, 0x2E, 0x42);
    widgets.hovered.bg_stroke = Stroke::new(1.0, BORDER_HI);
    widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);
    widgets.hovered.corner_radius = 10.into();
    widgets.hovered.expansion = 0.0;

    widgets.active.bg_fill = Color32::from_rgb(0x2B, 0x36, 0x4D);
    widgets.active.weak_bg_fill = Color32::from_rgb(0x2B, 0x36, 0x4D);
    widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    widgets.active.fg_stroke = Stroke::new(2.0, Color32::WHITE);
    widgets.active.corner_radius = 10.into();
    widgets.active.expansion = 0.0;

    widgets.open = widgets.active;
}

/// Top padding that vertically centres a stack of text lines (one font per line)
/// inside `height`. egui can't centre a multi-widget column on its own.
pub fn centre_pad(ui: &Ui, height: f32, lines: &[FontId]) -> f32 {
    let text: f32 = ui
        .ctx()
        .fonts_mut(|fonts| lines.iter().map(|font| fonts.row_height(font)).sum());
    let gaps = ui.spacing().item_spacing.y * lines.len().saturating_sub(1) as f32;
    ((height - text - gaps) / 2.0).max(0.0)
}

// ---------------------------------------------------------------- frames

/// Background + outer padding of the whole window.
pub fn page_frame() -> Frame {
    Frame::new().fill(BG).inner_margin(Margin::symmetric(28, 24))
}

/// A raised rounded panel (toolbar, time zone cards).
pub fn card_frame() -> Frame {
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(16)
        .inner_margin(Margin::symmetric(16, 14))
        .shadow(Shadow {
            offset: [0, 4],
            blur: 16,
            spread: 0,
            color: Color32::from_black_alpha(70),
        })
}

// ---------------------------------------------------------------- buttons

/// Like `ui.scope`, but without creating a child `Ui`, so wrapping rows still wrap the widget.
fn with_style<R>(ui: &mut Ui, restyle: impl FnOnce(&mut egui::Style), add: impl FnOnce(&mut Ui) -> R) -> R {
    let saved = ui.style().clone();
    restyle(ui.style_mut());
    let result = add(ui);
    ui.set_style(saved);
    result
}

/// Filled amber call-to-action button.
pub fn primary_button(ui: &mut Ui, text: &str) -> Response {
    with_style(
        ui,
        |style| {
            let widgets = &mut style.visuals.widgets;
            for (state, fill) in [
                (&mut widgets.inactive, ACCENT),
                (&mut widgets.hovered, ACCENT_HOVER),
                (&mut widgets.active, ACCENT_PRESSED),
            ] {
                state.weak_bg_fill = fill;
                state.bg_fill = fill;
                state.bg_stroke = Stroke::NONE;
                state.fg_stroke.color = ON_ACCENT;
            }
        },
        |ui| {
            ui.add(
                egui::Button::new(RichText::new(text).font(semibold(15.0)))
                    .wrap_mode(egui::TextWrapMode::Extend)
                    .corner_radius(10)
                    .min_size(vec2(0.0, 40.0)),
            )
        },
    )
}

pub const ICON_BUTTON_SIZE: f32 = 40.0;

/// Round, quiet icon button that turns red on hover. Used for delete.
pub fn danger_icon_button(ui: &mut Ui, icon: &str) -> Response {
    with_style(
        ui,
        |style| {
            let red_tint = DANGER.linear_multiply(0.16);
            let widgets = &mut style.visuals.widgets;
            widgets.inactive.weak_bg_fill = SURFACE;
            widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
            widgets.inactive.fg_stroke.color = TEXT_MUTED;
            for state in [&mut widgets.hovered, &mut widgets.active] {
                state.weak_bg_fill = red_tint;
                state.bg_stroke = Stroke::new(1.0, DANGER);
                state.fg_stroke.color = DANGER;
            }
            // no padding: the button is exactly ICON_BUTTON_SIZE, so rows can reserve room for it
            style.spacing.button_padding = Vec2::ZERO;
        },
        |ui| {
            ui.add(
                egui::Button::new(RichText::new(icon).size(17.0))
                    .corner_radius(ICON_BUTTON_SIZE / 2.0)
                    .min_size(Vec2::splat(ICON_BUTTON_SIZE)),
            )
            .on_hover_text("remove")
        },
    )
}
