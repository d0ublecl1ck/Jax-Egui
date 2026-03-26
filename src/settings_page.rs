use eframe::egui::{
    self, Align, Button, Color32, ComboBox, CornerRadius, Frame, Layout, Margin, RichText,
    ScrollArea, Sense, Stroke, Ui, Vec2,
};

use crate::components::{AppIcon, chip, colors};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    Models,
    Providers,
    Appearance,
    Git,
    Account,
    Experimental,
    Advanced,
    Updates,
}

impl SettingsTab {
    fn title(self) -> &'static str {
        match self {
            SettingsTab::General => "General",
            SettingsTab::Models => "Models",
            SettingsTab::Providers => "Providers",
            SettingsTab::Appearance => "Appearance",
            SettingsTab::Git => "Git",
            SettingsTab::Account => "Account",
            SettingsTab::Experimental => "Experimental",
            SettingsTab::Advanced => "Advanced",
            SettingsTab::Updates => "Check for updates",
        }
    }

    fn icon(self) -> AppIcon {
        match self {
            SettingsTab::General => AppIcon::SlidersHorizontal,
            SettingsTab::Models => AppIcon::Sparkles,
            SettingsTab::Providers => AppIcon::PlugZap,
            SettingsTab::Appearance => AppIcon::Palette,
            SettingsTab::Git => AppIcon::GitBranch,
            SettingsTab::Account => AppIcon::UserRound,
            SettingsTab::Experimental => AppIcon::FlaskConical,
            SettingsTab::Advanced => AppIcon::ShieldCheck,
            SettingsTab::Updates => AppIcon::RefreshCw,
        }
    }
}

pub struct SettingsState {
    pub active_tab: SettingsTab,
    pub send_key: usize,
    pub desktop_notifications: bool,
    pub sound_effects: bool,
    pub auto_convert_long_text: bool,
    pub strip_absolute_right: bool,
    pub always_show_context_usage: bool,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            active_tab: SettingsTab::General,
            send_key: 0,
            desktop_notifications: true,
            sound_effects: true,
            auto_convert_long_text: true,
            strip_absolute_right: false,
            always_show_context_usage: true,
        }
    }
}

pub fn draw_settings_page(ui: &mut Ui, settings: &mut SettingsState) -> bool {
    let mut back_to_app = false;
    ui.horizontal_top(|ui| {
        back_to_app = draw_settings_sidebar(ui, settings);
        ui.add_space(18.0);
        draw_settings_content(ui, settings);
    });
    back_to_app
}

fn draw_settings_sidebar(ui: &mut Ui, settings: &mut SettingsState) -> bool {
    let mut back_to_app = false;
    Frame::new()
        .fill(colors::SURFACE)
        .corner_radius(CornerRadius::same(12))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(210.0);
            ui.add_space(4.0);

            let back_button = Button::new(
                RichText::new("←  Back to app")
                    .color(colors::SUBTLE)
                    .size(14.0),
            )
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::NONE);
            if ui
                .add_sized([ui.available_width(), 30.0], back_button)
                .clicked()
            {
                back_to_app = true;
            }
            ui.add_space(10.0);

            for tab in [
                SettingsTab::General,
                SettingsTab::Models,
                SettingsTab::Providers,
                SettingsTab::Appearance,
                SettingsTab::Git,
                SettingsTab::Account,
            ] {
                let selected = settings.active_tab == tab;
                if settings_nav_item(ui, tab.title(), tab.icon(), selected).clicked() {
                    settings.active_tab = tab;
                }
            }

            ui.add_space(12.0);
            ui.label(RichText::new("More").color(colors::MUTED).size(12.0));
            ui.add_space(4.0);

            for tab in [
                SettingsTab::Experimental,
                SettingsTab::Advanced,
                SettingsTab::Updates,
            ] {
                let selected = settings.active_tab == tab;
                if settings_nav_item(ui, tab.title(), tab.icon(), selected).clicked() {
                    settings.active_tab = tab;
                }
            }
        });
    back_to_app
}

fn draw_settings_content(ui: &mut Ui, settings: &mut SettingsState) {
    Frame::new()
        .fill(colors::PANEL)
        .corner_radius(CornerRadius::same(12))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::same(16))
        .show(ui, |ui| {
            ui.set_width((ui.available_width() - 2.0).max(580.0));
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(settings.active_tab.title())
                            .color(colors::TEXT)
                            .size(34.0),
                    );
                    ui.add_space(14.0);

                    match settings.active_tab {
                        SettingsTab::General => draw_general_section(ui, settings),
                        _ => {
                            ui.label(
                                RichText::new("该分组稍后接入。当前先实现 General 页面。")
                                    .color(colors::SUBTLE)
                                    .size(14.0),
                            );
                        }
                    }
                });
        });
}

fn draw_general_section(ui: &mut Ui, settings: &mut SettingsState) {
    setting_row(
        ui,
        "Send messages with",
        "Choose which key combination sends messages",
        |ui| {
            ComboBox::from_id_salt("send-message-with")
                .selected_text(if settings.send_key == 0 {
                    "⌘ Enter"
                } else {
                    "Ctrl Enter"
                })
                .width(138.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.send_key, 0, "⌘ Enter");
                    ui.selectable_value(&mut settings.send_key, 1, "Ctrl Enter");
                });
        },
    );

    ui.label(
        RichText::new("Use ↵ for new lines")
            .color(colors::MUTED)
            .size(12.0),
    );
    ui.add_space(10.0);
    separator(ui);

    setting_row(
        ui,
        "Desktop notifications",
        "Get notified when AI finishes working in a chat.",
        |ui| {
            switch(ui, &mut settings.desktop_notifications);
        },
    );
    separator(ui);

    setting_row(
        ui,
        "Sound effects",
        "Play a sound when AI finishes working in a chat.",
        |ui| {
            chip(ui, None, Some("Choo Ch..."), Some(88.0));
            ui.add_space(8.0);
            ui.add(Button::new(RichText::new("Test").size(13.0)).fill(Color32::TRANSPARENT));
            ui.add_space(8.0);
            switch(ui, &mut settings.sound_effects);
        },
    );
    separator(ui);

    setting_row(
        ui,
        "Auto-convert long text",
        "Convert pasted text over 5000 characters into text attachments.",
        |ui| {
            switch(ui, &mut settings.auto_convert_long_text);
        },
    );
    separator(ui);

    setting_row(
        ui,
        "I'm not absolutely right, thank you very much",
        "Strip 'You're absolutely right' from AI messages",
        |ui| {
            switch(ui, &mut settings.strip_absolute_right);
        },
    );
    separator(ui);

    setting_row(
        ui,
        "Always show context usage",
        "Always show context percent used in a chat (Claude only).",
        |ui| {
            switch(ui, &mut settings.always_show_context_usage);
        },
    );
}

fn settings_nav_item(
    ui: &mut Ui,
    text: &str,
    icon_kind: AppIcon,
    selected: bool,
) -> egui::Response {
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
        .inner_margin(Margin::symmetric(8, 2))
        .show(ui, |ui| {
            ui.add_sized(
                [ui.available_width(), 30.0],
                Button::image_and_text(
                    crate::components::icon_image(icon_kind, 13.0),
                    RichText::new(text)
                        .color(if selected {
                            colors::TEXT
                        } else {
                            colors::SUBTLE
                        })
                        .size(13.5),
                )
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::NONE),
            )
        })
        .inner
}

fn setting_row(ui: &mut Ui, title: &str, description: &str, mut trailing: impl FnMut(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(title).color(colors::TEXT).size(16.0));
            ui.add_space(2.0);
            ui.label(RichText::new(description).color(colors::SUBTLE).size(13.0));
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| trailing(ui));
    });
    ui.add_space(12.0);
}

fn switch(ui: &mut Ui, value: &mut bool) -> egui::Response {
    let desired_size = Vec2::new(34.0, 20.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *value = !*value;
    }

    let radius = rect.height() / 2.0;
    let track_color = if *value {
        Color32::from_rgb(224, 224, 224)
    } else {
        colors::PANEL_ALT
    };
    let knob_color = if *value {
        Color32::from_rgb(26, 26, 26)
    } else {
        Color32::from_rgb(153, 153, 153)
    };

    ui.painter().rect(
        rect,
        CornerRadius::same(radius as u8),
        track_color,
        Stroke::new(1.0, colors::BORDER_SOFT),
        egui::StrokeKind::Middle,
    );

    let knob_x = if *value {
        rect.right() - radius
    } else {
        rect.left() + radius
    };
    ui.painter().circle_filled(
        egui::pos2(knob_x, rect.center().y),
        radius - 3.0,
        knob_color,
    );

    response
}

fn separator(ui: &mut Ui) {
    ui.add_space(4.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        Stroke::new(1.0, colors::BORDER),
    );
    ui.add_space(12.0);
}
