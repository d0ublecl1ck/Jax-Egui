use eframe::egui::{
    self, Align, Button, Color32, CornerRadius, Frame, Image, ImageSource, Layout, Margin,
    RichText, Stroke, Ui, Vec2,
};

pub fn configure_visuals(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);
    style.visuals = egui::Visuals::dark();
    style.visuals.override_text_color = Some(colors::TEXT);
    style.visuals.panel_fill = colors::BACKGROUND;
    style.visuals.window_fill = colors::BACKGROUND;
    style.visuals.widgets.noninteractive.bg_fill = colors::BACKGROUND;
    style.visuals.widgets.inactive.bg_fill = colors::PANEL_ALT;
    style.visuals.widgets.hovered.bg_fill = colors::HOVER;
    style.visuals.widgets.active.bg_fill = colors::SELECTED;
    style.visuals.widgets.noninteractive.fg_stroke.color = colors::TEXT;
    style.visuals.widgets.inactive.fg_stroke.color = colors::TEXT;
    style.visuals.widgets.hovered.fg_stroke.color = colors::TEXT;
    style.visuals.widgets.active.fg_stroke.color = colors::TEXT;
    style.visuals.selection.bg_fill = colors::ACCENT;
    ctx.set_style(style);
}

#[derive(Clone, Copy)]
pub enum AppIcon {
    Activity,
    PanelLeft,
    HardDrive,
    ArrowLeft,
    ArrowRight,
    ChevronRight,
    ChevronDown,
    ListFilter,
    GitBranch,
    Folder,
    FileText,
    Search,
    Plus,
    Terminal,
    SendHorizontal,
    Settings,
    Bot,
    UserRound,
    BadgeInfo,
    Gauge,
    SlidersHorizontal,
    Sparkles,
    PlugZap,
    Palette,
    ShieldCheck,
    FlaskConical,
    RefreshCw,
}

fn icon_source(icon: AppIcon) -> ImageSource<'static> {
    match icon {
        AppIcon::Activity => egui::include_image!("../assets/icons/svg/activity.svg"),
        AppIcon::PanelLeft => egui::include_image!("../assets/icons/svg/panel-left.svg"),
        AppIcon::HardDrive => egui::include_image!("../assets/icons/svg/hard-drive.svg"),
        AppIcon::ArrowLeft => egui::include_image!("../assets/icons/svg/arrow-left.svg"),
        AppIcon::ArrowRight => egui::include_image!("../assets/icons/svg/arrow-right.svg"),
        AppIcon::ChevronRight => egui::include_image!("../assets/icons/svg/chevron-right.svg"),
        AppIcon::ChevronDown => egui::include_image!("../assets/icons/svg/chevron-down.svg"),
        AppIcon::ListFilter => egui::include_image!("../assets/icons/svg/list-filter.svg"),
        AppIcon::GitBranch => egui::include_image!("../assets/icons/svg/git-branch.svg"),
        AppIcon::Folder => egui::include_image!("../assets/icons/svg/folder.svg"),
        AppIcon::FileText => egui::include_image!("../assets/icons/svg/file-text.svg"),
        AppIcon::Search => egui::include_image!("../assets/icons/svg/search.svg"),
        AppIcon::Plus => egui::include_image!("../assets/icons/svg/plus.svg"),
        AppIcon::Terminal => egui::include_image!("../assets/icons/svg/terminal.svg"),
        AppIcon::SendHorizontal => egui::include_image!("../assets/icons/svg/send-horizontal.svg"),
        AppIcon::Settings => egui::include_image!("../assets/icons/svg/settings.svg"),
        AppIcon::Bot => egui::include_image!("../assets/icons/svg/bot.svg"),
        AppIcon::UserRound => egui::include_image!("../assets/icons/svg/user-round.svg"),
        AppIcon::BadgeInfo => egui::include_image!("../assets/icons/svg/badge-info.svg"),
        AppIcon::Gauge => egui::include_image!("../assets/icons/svg/gauge.svg"),
        AppIcon::SlidersHorizontal => egui::include_image!("../assets/icons/svg/list-filter.svg"),
        AppIcon::Sparkles => egui::include_image!("../assets/icons/svg/activity.svg"),
        AppIcon::PlugZap => egui::include_image!("../assets/icons/svg/git-branch.svg"),
        AppIcon::Palette => egui::include_image!("../assets/icons/svg/panel-left.svg"),
        AppIcon::ShieldCheck => egui::include_image!("../assets/icons/svg/badge-info.svg"),
        AppIcon::FlaskConical => egui::include_image!("../assets/icons/svg/terminal.svg"),
        AppIcon::RefreshCw => egui::include_image!("../assets/icons/svg/arrow-right.svg"),
    }
}

pub fn icon_image(icon: AppIcon, size: f32) -> Image<'static> {
    Image::new(icon_source(icon)).fit_to_exact_size(Vec2::splat(size))
}

pub fn icon(ui: &mut Ui, icon_kind: AppIcon, size: f32) {
    ui.add(icon_image(icon_kind, size));
}

pub fn draw_traffic_lights(ui: &mut Ui) {
    let lights = [
        Color32::from_rgb(255, 95, 86),
        Color32::from_rgb(255, 189, 46),
        Color32::from_rgb(39, 201, 63),
    ];

    ui.horizontal(|ui| {
        for color in lights {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), egui::Sense::hover());
            ui.painter().circle_filled(rect.center(), 5.2, color);
            ui.add_space(4.0);
        }
    });
}

pub fn chip(
    ui: &mut Ui,
    icon_kind: Option<AppIcon>,
    label: Option<&str>,
    fixed_width: Option<f32>,
) {
    let width = fixed_width.unwrap_or(92.0);
    Frame::new()
        .fill(colors::PANEL_ALT)
        .corner_radius(CornerRadius::same(10))
        .stroke(Stroke::new(1.0, colors::BORDER_SOFT))
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width(width);
            ui.horizontal_centered(|ui| {
                if let Some(icon_kind) = icon_kind {
                    icon(ui, icon_kind, 12.0);
                }
                if let Some(text) = label {
                    ui.label(RichText::new(text).color(colors::TEXT).size(12.5));
                }
            });
        });
}

pub fn workspace_item(ui: &mut Ui, text: &str, selected: bool) -> egui::Response {
    let fill = if selected {
        colors::SELECTED
    } else {
        Color32::TRANSPARENT
    };
    let stroke = if selected {
        Stroke::new(1.0, colors::SELECTED_BORDER)
    } else {
        Stroke::NONE
    };

    Frame::new()
        .fill(fill)
        .corner_radius(CornerRadius::same(8))
        .stroke(stroke)
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.add_sized(
                [ui.available_width(), 34.0],
                Button::new(
                    RichText::new(format!("{}  {}", text.chars().next().unwrap_or('•'), text))
                        .color(if selected {
                            colors::TEXT
                        } else {
                            colors::SUBTLE
                        })
                        .size(14.5),
                )
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::NONE),
            )
        })
        .inner
}

pub fn tab_button(ui: &mut Ui, text: &str, active: bool) -> egui::Response {
    ui.add(
        Button::new(
            RichText::new(text)
                .color(if active { colors::TEXT } else { colors::MUTED })
                .size(14.0),
        )
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE),
    )
}

pub fn bottom_tab_button(ui: &mut Ui, text: &str, active: bool) -> egui::Response {
    ui.add(
        Button::new(
            RichText::new(text)
                .color(if active { colors::TEXT } else { colors::SUBTLE })
                .size(13.5),
        )
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE),
    )
}

pub fn file_row(ui: &mut Ui, text: &str, selected: bool) -> egui::Response {
    let fill = if selected {
        colors::SELECTED
    } else {
        Color32::TRANSPARENT
    };
    let icon_kind = if text.starts_with('.') {
        AppIcon::Folder
    } else {
        AppIcon::FileText
    };

    Frame::new()
        .fill(fill)
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.add_sized(
                [ui.available_width(), 34.0],
                Button::image_and_text(
                    icon_image(icon_kind, 14.0),
                    RichText::new(text).color(colors::TEXT).size(14.5),
                )
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::NONE),
            )
        })
        .inner
}

pub fn footer_pill(ui: &mut Ui, icon_kind: Option<AppIcon>, text: &str) {
    Frame::new()
        .fill(Color32::TRANSPARENT)
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::symmetric(2, 2))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(icon_kind) = icon_kind {
                    icon(ui, icon_kind, 13.0);
                }
                ui.label(RichText::new(text).color(colors::SUBTLE).size(13.0));
            });
        });
}

pub fn trailing_label(ui: &mut Ui, text: &str, color: Color32) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.label(RichText::new(text).color(color).size(12.5));
    });
}

pub mod colors {
    use eframe::egui::Color32;

    pub const BACKGROUND: Color32 = Color32::from_rgb(16, 14, 14);
    pub const SURFACE: Color32 = Color32::from_rgb(33, 29, 27);
    pub const PANEL: Color32 = Color32::from_rgb(17, 14, 14);
    pub const PANEL_ALT: Color32 = Color32::from_rgb(41, 36, 33);
    pub const CARD: Color32 = Color32::from_rgb(48, 42, 38);
    pub const BORDER: Color32 = Color32::from_rgb(57, 49, 45);
    pub const BORDER_SOFT: Color32 = Color32::from_rgb(68, 58, 53);
    pub const TEXT: Color32 = Color32::from_rgb(227, 220, 212);
    pub const SUBTLE: Color32 = Color32::from_rgb(170, 161, 152);
    pub const MUTED: Color32 = Color32::from_rgb(134, 127, 121);
    pub const SELECTED: Color32 = Color32::from_rgb(61, 56, 52);
    pub const SELECTED_BORDER: Color32 = Color32::from_rgb(84, 76, 71);
    pub const HOVER: Color32 = Color32::from_rgb(52, 46, 42);
    pub const ACCENT: Color32 = Color32::from_rgb(88, 160, 251);
    pub const INPUT_ACTION: Color32 = Color32::from_rgb(72, 68, 63);
    pub const SUCCESS: Color32 = Color32::from_rgb(72, 186, 120);
    pub const WARNING: Color32 = Color32::from_rgb(241, 181, 67);
}
