use eframe::{
    App, CreationContext, NativeOptions,
    egui::{
        self, Align, Button, CentralPanel, Color32, Frame, Layout, Margin, RichText, Rounding,
        ScrollArea, SidePanel, Stroke, TextEdit, TopBottomPanel, Ui, Vec2,
    },
};
use egui_terminal::{TermHandler, Terminal};

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        initial_window_size: Some(Vec2::new(1320.0, 820.0)),
        min_window_size: Some(Vec2::new(1100.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "memphis-v2",
        options,
        Box::new(|cc| Box::new(MemphisApp::new(cc))),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RightTab {
    AllFiles,
    Changes,
    Checks,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomTab {
    Setup,
    Spotlight,
    Terminal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MessageRole {
    System,
    User,
    Assistant,
}

struct Message {
    role: MessageRole,
    text: String,
}

struct FileEntry {
    name: &'static str,
    preview: &'static str,
}

struct ChangeItem {
    path: &'static str,
    summary: &'static str,
    kind: &'static str,
}

struct CheckItem {
    name: &'static str,
    status: &'static str,
    detail: &'static str,
}

#[derive(Clone, Copy)]
enum AppIcon {
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
}

struct MemphisApp {
    workspaces: Vec<&'static str>,
    files: Vec<FileEntry>,
    changes: Vec<ChangeItem>,
    checks: Vec<CheckItem>,
    selected_workspace: usize,
    selected_file: usize,
    right_tab: RightTab,
    bottom_tab: BottomTab,
    input_text: String,
    messages: Vec<Message>,
    status_text: String,
    terminal: TermHandler,
}

impl MemphisApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        configure_visuals(&cc.egui_ctx);

        Self {
            workspaces: vec![
                "the-question-backend",
                "jizhi-server",
                "skills-manager",
                "bazi-mcp",
                "bulk-mind-backend",
                "json-formatter",
                "bulk-mind-next",
                "react-components",
                "paperclip",
                "coclaw",
                "atomic-lobster",
                "Fix bun start",
                "wisequest",
                "bulk-mind",
                "Jax",
                "Memphis v2",
            ],
            files: vec![
                FileEntry {
                    name: ".claude",
                    preview: "# Claude\n\n- workspace: memphis-v2\n- status: active\n- notes: UI interaction prototype",
                },
                FileEntry {
                    name: ".codex",
                    preview: "[agent]\nmode = \"interactive\"\nprofile = \"desktop-ui\"\n",
                },
                FileEntry {
                    name: ".context",
                    preview: "attachments/\nnotes/\nshared-state/\n\nThis folder stores transient collaboration artifacts.",
                },
                FileEntry {
                    name: ".opencode",
                    preview: "router = \"local\"\nwindow = \"memphis-v2\"\nlayout = \"3-column\"\n",
                },
                FileEntry {
                    name: ".git",
                    preview: "gitdir: /Users/d0ublecl1ck/conductor/.git/worktrees/clone-ui",
                },
                FileEntry {
                    name: "README.md",
                    preview: "# Jax\n\nMemphis v2 desktop prototype built with Rust + egui.\n\n## Goals\n- clone screenshot\n- make layout interactive\n- prepare richer stateful desktop shell\n",
                },
            ],
            changes: vec![
                ChangeItem {
                    path: "src/main.rs",
                    summary: "Add interactive panels, file preview, and simulated checks UI",
                    kind: "Modified",
                },
                ChangeItem {
                    path: "Cargo.toml",
                    summary: "Enable eframe desktop app shell",
                    kind: "Modified",
                },
                ChangeItem {
                    path: "Cargo.lock",
                    summary: "Lock egui and eframe dependency graph",
                    kind: "Generated",
                },
            ],
            checks: vec![
                CheckItem {
                    name: "cargo check",
                    status: "Passed",
                    detail: "Rust crate compiles successfully under the current profile.",
                },
                CheckItem {
                    name: "cargo fmt --all",
                    status: "Passed",
                    detail: "Source formatting is normalized.",
                },
                CheckItem {
                    name: "UI fidelity review",
                    status: "In Progress",
                    detail: "Still room for tighter parity on spacing and iconography.",
                },
            ],
            selected_workspace: 15,
            selected_file: 5,
            right_tab: RightTab::AllFiles,
            bottom_tab: BottomTab::Terminal,
            input_text: "技术栈使用 Rust + egui 做客户端，把这个页面复刻下".to_owned(),
            messages: vec![
                Message {
                    role: MessageRole::System,
                    text: "You're in a new copy of Jax called memphis-v2".to_owned(),
                },
                Message {
                    role: MessageRole::Assistant,
                    text: "Branched d0ublecl1ck/memphis-v2 from origin/main".to_owned(),
                },
                Message {
                    role: MessageRole::Assistant,
                    text: "Created memphis-v2 and copied 21 files".to_owned(),
                },
                Message {
                    role: MessageRole::Assistant,
                    text: "Optional: add a setup script ↗".to_owned(),
                },
            ],
            status_text: "交互模式已启用".to_owned(),
            terminal: create_terminal(),
        }
    }

    fn send_message(&mut self) {
        let trimmed = self.input_text.trim();
        if trimmed.is_empty() {
            self.status_text = "输入为空，未发送".to_owned();
            return;
        }

        let user_text = trimmed.to_owned();
        self.messages.push(Message {
            role: MessageRole::User,
            text: user_text.clone(),
        });
        self.messages.push(Message {
            role: MessageRole::Assistant,
            text: format!(
                "已接收需求：{}。当前工作区是 {}，当前文件是 {}。",
                user_text,
                self.current_workspace(),
                self.current_file().name
            ),
        });

        self.status_text = format!("消息已发送到 {}", self.current_workspace());
        self.bottom_tab = BottomTab::Terminal;
        self.input_text.clear();
    }

    fn current_workspace(&self) -> &str {
        self.workspaces
            .get(self.selected_workspace)
            .copied()
            .unwrap_or("Memphis v2")
    }

    fn current_file(&self) -> &FileEntry {
        self.files.get(self.selected_file).unwrap_or(&self.files[0])
    }

    fn append_assistant_message(&mut self, text: impl Into<String>) {
        self.messages.push(Message {
            role: MessageRole::Assistant,
            text: text.into(),
        });
    }

    fn draw_top_bar(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            draw_traffic_lights(ui);
            ui.add_space(6.0);
            chip(ui, Some(AppIcon::PanelLeft), None, Some(28.0));
            chip(ui, Some(AppIcon::HardDrive), Some("1.37 GB"), None);
            icon(ui, AppIcon::ArrowLeft, 14.0);
            icon(ui, AppIcon::ArrowRight, 14.0);
            ui.add_space(10.0);
            ui.label(
                RichText::new(format!(
                    "d0ublecl1ck/{}",
                    self.current_workspace().to_lowercase().replace(' ', "-")
                ))
                .color(colors::TEXT)
                .size(17.0),
            );
            ui.add_space(12.0);
            icon(ui, AppIcon::ChevronRight, 14.0);
            ui.label(RichText::new("origin/main").color(colors::MUTED).size(14.0));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                chip(ui, None, Some("/memphis-v2"), Some(126.0));
                ui.add_space(8.0);
                chip(ui, Some(AppIcon::PanelLeft), None, Some(28.0));
            });
        });
    }

    fn draw_left_sidebar(&mut self, ui: &mut Ui) {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            icon(ui, AppIcon::Activity, 14.0);
            ui.label(RichText::new("Activity").color(colors::TEXT).size(15.0));
        });
        ui.add_space(18.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Workspaces").color(colors::SUBTLE).size(13.0));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                icon(ui, AppIcon::ChevronDown, 12.0);
                ui.add_space(4.0);
                icon(ui, AppIcon::ListFilter, 12.0);
            });
        });
        ui.add_space(8.0);

        for index in 0..self.workspaces.len() {
            let item = self.workspaces[index];
            let selected = index == self.selected_workspace;
            if workspace_item(ui, item, selected).clicked() {
                self.selected_workspace = index;
                self.status_text = format!("已切换到工作区：{}", item);
                self.append_assistant_message(format!("工作区已切换到 {}。", item));
            }
            ui.add_space(3.0);
        }

        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.horizontal(|ui| {
                icon(ui, AppIcon::Settings, 14.0);
                icon(ui, AppIcon::Activity, 14.0);
            });
        });
    }

    fn draw_right_sidebar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if tab_button(ui, "All files", self.right_tab == RightTab::AllFiles).clicked() {
                self.right_tab = RightTab::AllFiles;
                self.status_text = "已打开文件列表".to_owned();
            }
            if tab_button(ui, "Changes 3", self.right_tab == RightTab::Changes).clicked() {
                self.right_tab = RightTab::Changes;
                self.status_text = "已打开变更列表".to_owned();
            }
            if tab_button(ui, "Checks", self.right_tab == RightTab::Checks).clicked() {
                self.right_tab = RightTab::Checks;
                self.status_text = "已打开检查列表".to_owned();
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                icon(ui, AppIcon::Search, 14.0);
                ui.add_space(12.0);
                ui.label(RichText::new("⇅").color(colors::MUTED).size(14.0));
            });
        });
        ui.add_space(14.0);

        match self.right_tab {
            RightTab::AllFiles => {
                for index in 0..self.files.len() {
                    let file = &self.files[index];
                    let selected = index == self.selected_file;
                    if file_row(ui, file.name, selected).clicked() {
                        self.selected_file = index;
                        self.status_text = format!("已选中文件：{}", file.name);
                    }
                    ui.add_space(2.0);
                }
            }
            RightTab::Changes => {
                for item in &self.changes {
                    change_row(ui, item);
                    ui.add_space(8.0);
                }
            }
            RightTab::Checks => {
                for item in &self.checks {
                    check_row(ui, item);
                    ui.add_space(8.0);
                }
            }
        }

        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Spotlight").color(colors::TEXT).size(13.0));
                ui.add_space(6.0);
                chip(ui, Some(AppIcon::Search), Some("Spotlight"), Some(108.0));
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if bottom_tab_button(ui, "Setup", self.bottom_tab == BottomTab::Setup).clicked() {
                    self.bottom_tab = BottomTab::Setup;
                }
                if bottom_tab_button(ui, "Spotlight", self.bottom_tab == BottomTab::Spotlight)
                    .clicked()
                {
                    self.bottom_tab = BottomTab::Spotlight;
                }
                if bottom_tab_button(ui, "Terminal", self.bottom_tab == BottomTab::Terminal)
                    .clicked()
                {
                    self.bottom_tab = BottomTab::Terminal;
                }
                icon(ui, AppIcon::Plus, 14.0);
            });
            ui.add_space(10.0);
            bottom_panel_content(ui, self.bottom_tab, &self.status_text, &mut self.terminal);
        });
    }

    fn draw_center_panel(&mut self, ui: &mut Ui) {
        tab_strip(ui, self.current_file().name);
        ui.add_space(18.0);

        ui.columns(2, |columns| {
            let left = &mut columns[0];
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(left, |ui| {
                    for message in &self.messages {
                        message_bubble(ui, message);
                        ui.add_space(12.0);
                    }
                });

            let right = &mut columns[1];
            file_preview_panel(right, self.current_file());
        });

        ui.add_space(12.0);
        if composer(ui, &mut self.input_text, &mut self.status_text) {
            self.send_message();
        }
    }
}

impl App for MemphisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_bar")
            .exact_height(42.0)
            .frame(
                Frame::none()
                    .fill(colors::SURFACE)
                    .inner_margin(Margin::symmetric(16.0, 8.0))
                    .stroke(Stroke::new(1.0, colors::BORDER)),
            )
            .show(ctx, |ui| self.draw_top_bar(ui));

        SidePanel::left("workspace_sidebar")
            .exact_width(268.0)
            .resizable(false)
            .frame(
                Frame::none()
                    .fill(colors::SURFACE)
                    .inner_margin(Margin::symmetric(16.0, 12.0))
                    .stroke(Stroke::new(1.0, colors::BORDER)),
            )
            .show(ctx, |ui| self.draw_left_sidebar(ui));

        SidePanel::right("files_sidebar")
            .exact_width(370.0)
            .resizable(false)
            .frame(
                Frame::none()
                    .fill(colors::PANEL)
                    .inner_margin(Margin::symmetric(14.0, 12.0))
                    .stroke(Stroke::new(1.0, colors::BORDER)),
            )
            .show(ctx, |ui| self.draw_right_sidebar(ui));

        CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(colors::BACKGROUND)
                    .inner_margin(Margin::symmetric(20.0, 12.0)),
            )
            .show(ctx, |ui| self.draw_center_panel(ui));
    }
}

fn configure_visuals(ctx: &egui::Context) {
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

fn icon_text(icon: AppIcon) -> &'static str {
    match icon {
        AppIcon::Activity => "A",
        AppIcon::PanelLeft => "P",
        AppIcon::HardDrive => "HD",
        AppIcon::ArrowLeft => "<",
        AppIcon::ArrowRight => ">",
        AppIcon::ChevronRight => ">",
        AppIcon::ChevronDown => "v",
        AppIcon::ListFilter => "LF",
        AppIcon::GitBranch => "GB",
        AppIcon::Folder => "D",
        AppIcon::FileText => "F",
        AppIcon::Search => "S",
        AppIcon::Plus => "+",
        AppIcon::Terminal => "T",
        AppIcon::SendHorizontal => "->",
        AppIcon::Settings => "*",
        AppIcon::Bot => "B",
        AppIcon::UserRound => "U",
        AppIcon::BadgeInfo => "I",
        AppIcon::Gauge => "G",
    }
}

fn icon(ui: &mut Ui, icon_kind: AppIcon, size: f32) {
    ui.label(
        RichText::new(icon_text(icon_kind))
            .size(size)
            .color(colors::MUTED),
    );
}

fn draw_traffic_lights(ui: &mut Ui) {
    let colors = [
        Color32::from_rgb(255, 95, 86),
        Color32::from_rgb(255, 189, 46),
        Color32::from_rgb(39, 201, 63),
    ];

    ui.horizontal(|ui| {
        for color in colors {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), egui::Sense::hover());
            ui.painter().circle_filled(rect.center(), 5.2, color);
            ui.add_space(4.0);
        }
    });
}

fn chip(ui: &mut Ui, icon_kind: Option<AppIcon>, label: Option<&str>, fixed_width: Option<f32>) {
    let width = fixed_width.unwrap_or(92.0);
    Frame::none()
        .fill(colors::PANEL_ALT)
        .rounding(Rounding::same(10.0))
        .stroke(Stroke::new(1.0, colors::BORDER_SOFT))
        .inner_margin(Margin::symmetric(10.0, 6.0))
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

fn tab_strip(ui: &mut Ui, current_file: &str) {
    Frame::none()
        .fill(colors::SURFACE)
        .rounding(Rounding::same(10.0))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::symmetric(14.0, 8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                icon(ui, AppIcon::FileText, 13.0);
                ui.add_space(6.0);
                ui.label(RichText::new(current_file).color(colors::TEXT).size(15.0));
                ui.add_space(16.0);
                icon(ui, AppIcon::Plus, 14.0);
            });
        });
}

fn workspace_item(ui: &mut Ui, text: &str, selected: bool) -> egui::Response {
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

    Frame::none()
        .fill(fill)
        .rounding(Rounding::same(8.0))
        .stroke(stroke)
        .inner_margin(Margin::same(0.0))
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

fn tab_button(ui: &mut Ui, text: &str, active: bool) -> egui::Response {
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

fn file_row(ui: &mut Ui, text: &str, selected: bool) -> egui::Response {
    let fill = if selected {
        colors::SELECTED
    } else {
        Color32::TRANSPARENT
    };
    let icon_prefix = if text.starts_with('.') { "[D]" } else { "[F]" };

    Frame::none()
        .fill(fill)
        .rounding(Rounding::same(6.0))
        .inner_margin(Margin::same(0.0))
        .show(ui, |ui| {
            ui.add_sized(
                [ui.available_width(), 34.0],
                Button::new(
                    RichText::new(format!("{} {}", icon_prefix, text))
                        .color(colors::TEXT)
                        .size(14.5),
                )
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::NONE),
            )
        })
        .inner
}

fn change_row(ui: &mut Ui, item: &ChangeItem) {
    Frame::none()
        .fill(colors::PANEL_ALT)
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(10.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                icon(ui, AppIcon::GitBranch, 14.0);
                ui.label(RichText::new(item.kind).color(colors::ACCENT).size(12.5));
                ui.label(RichText::new(item.path).color(colors::TEXT).size(14.0));
            });
            ui.add_space(4.0);
            ui.label(RichText::new(item.summary).color(colors::SUBTLE).size(13.0));
        });
}

fn check_row(ui: &mut Ui, item: &CheckItem) {
    let status_color = match item.status {
        "Passed" => Color32::from_rgb(72, 186, 120),
        "In Progress" => Color32::from_rgb(241, 181, 67),
        _ => colors::MUTED,
    };

    Frame::none()
        .fill(colors::PANEL_ALT)
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(10.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(item.name).color(colors::TEXT).size(14.0));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(item.status).color(status_color).size(12.5));
                });
            });
            ui.add_space(4.0);
            ui.label(RichText::new(item.detail).color(colors::SUBTLE).size(13.0));
        });
}

fn bottom_tab_button(ui: &mut Ui, text: &str, active: bool) -> egui::Response {
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

fn bottom_panel_content(
    ui: &mut Ui,
    bottom_tab: BottomTab,
    status_text: &str,
    terminal: &mut TermHandler,
) {
    Frame::none()
        .fill(colors::PANEL_ALT)
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(10.0))
        .show(ui, |ui| match bottom_tab {
            BottomTab::Terminal => {
                ui.label(
                    RichText::new("memphis-v2  d0ublecl1ck/memphis-v2  ready")
                        .color(colors::TEXT)
                        .size(12.5),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    icon(ui, AppIcon::Terminal, 14.0);
                    ui.label(RichText::new(status_text).color(colors::SUBTLE).size(13.0));
                });
                ui.add_space(8.0);
                let terminal_size = Vec2::new(ui.available_width(), 180.0);
                ui.add_sized(terminal_size, Terminal::new(terminal));
            }
            BottomTab::Setup => {
                ui.label(RichText::new("Setup").color(colors::TEXT).size(13.5));
                ui.add_space(8.0);
                ui.label(
                    RichText::new("这里可以继续挂接项目初始化脚本与环境检查。")
                        .color(colors::SUBTLE)
                        .size(13.0),
                );
            }
            BottomTab::Spotlight => {
                ui.label(RichText::new("Spotlight").color(colors::TEXT).size(13.5));
                ui.add_space(8.0);
                ui.label(
                    RichText::new("后续可接入命令面板、全局搜索和快捷动作。")
                        .color(colors::SUBTLE)
                        .size(13.0),
                );
            }
        });
}

fn file_preview_panel(ui: &mut Ui, file: &FileEntry) {
    Frame::none()
        .fill(colors::SURFACE)
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::same(14.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                icon(ui, AppIcon::FileText, 14.0);
                ui.label(RichText::new("Preview").color(colors::TEXT).size(14.0));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(file.name).color(colors::SUBTLE).size(12.5));
                });
            });
            ui.add_space(10.0);
            Frame::none()
                .fill(colors::PANEL)
                .rounding(Rounding::same(10.0))
                .inner_margin(Margin::same(12.0))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(file.preview)
                            .color(colors::TEXT)
                            .size(13.5)
                            .monospace(),
                    );
                });
        });
}

fn message_bubble(ui: &mut Ui, message: &Message) {
    let (fill, text_color, border, bubble_icon) = match message.role {
        MessageRole::System => (
            colors::CARD,
            colors::TEXT,
            colors::BORDER_SOFT,
            AppIcon::BadgeInfo,
        ),
        MessageRole::User => (
            colors::SURFACE,
            colors::TEXT,
            colors::SELECTED_BORDER,
            AppIcon::UserRound,
        ),
        MessageRole::Assistant => (
            Color32::TRANSPARENT,
            colors::TEXT,
            Color32::TRANSPARENT,
            AppIcon::Bot,
        ),
    };

    Frame::none()
        .fill(fill)
        .rounding(Rounding::same(if message.role == MessageRole::System {
            14.0
        } else {
            10.0
        }))
        .stroke(Stroke::new(
            if border == Color32::TRANSPARENT {
                0.0
            } else {
                1.0
            },
            border,
        ))
        .inner_margin(Margin::same(14.0))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                icon(ui, bubble_icon, 14.0);
                ui.add_space(4.0);
                ui.label(RichText::new(&message.text).color(text_color).size(15.5));
            });
        });
}

fn composer(ui: &mut Ui, input_text: &mut String, status_text: &mut String) -> bool {
    let mut should_send = false;

    Frame::none()
        .fill(colors::SURFACE)
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::same(16.0))
        .show(ui, |ui| {
            let response = ui.add_sized(
                [ui.available_width(), 92.0],
                TextEdit::multiline(input_text)
                    .hint_text("输入你的指令…")
                    .desired_rows(4)
                    .frame(false),
            );

            if response.changed() {
                *status_text = "正在编辑输入内容".to_owned();
            }

            let send_by_enter = response.has_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command);
            if send_by_enter {
                should_send = true;
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                footer_pill(ui, Some(AppIcon::Bot), "GPT-5.4");
                footer_pill(ui, Some(AppIcon::Activity), "Fast");
                footer_pill(ui, Some(AppIcon::Gauge), "Medium");
                ui.label(RichText::new("⌇").color(colors::MUTED).size(16.0));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let send_clicked = ui
                        .add(
                            Button::new(
                                RichText::new(icon_text(AppIcon::SendHorizontal)).size(13.0),
                            )
                            .fill(colors::INPUT_ACTION)
                            .rounding(Rounding::same(8.0)),
                        )
                        .clicked();
                    ui.add_space(8.0);
                    icon(ui, AppIcon::Plus, 16.0);
                    if send_clicked {
                        should_send = true;
                    }
                });
            });
        });

    should_send
}

fn footer_pill(ui: &mut Ui, icon_kind: Option<AppIcon>, text: &str) {
    Frame::none()
        .fill(Color32::TRANSPARENT)
        .rounding(Rounding::same(6.0))
        .inner_margin(Margin::symmetric(2.0, 2.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(icon_kind) = icon_kind {
                    icon(ui, icon_kind, 13.0);
                }
                ui.label(RichText::new(text).color(colors::SUBTLE).size(13.0));
            });
        });
}

fn create_terminal() -> TermHandler {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "zsh".to_owned());
    TermHandler::new_from_str(&shell)
}

mod colors {
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
}
