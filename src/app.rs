use eframe::{
    App, CreationContext, NativeOptions,
    egui::{
        self, Align, Button, CentralPanel, Color32, CornerRadius, Frame, Layout, Margin, RichText,
        ScrollArea, SidePanel, Stroke, TextEdit, TopBottomPanel, Ui, ViewportBuilder,
    },
};

use crate::components::{
    AppIcon, bottom_tab_button, chip, colors, configure_visuals, draw_traffic_lights, file_row,
    footer_pill, icon, icon_image, tab_button, trailing_label, workspace_item,
};
use crate::settings_page::{SettingsState, draw_settings_page};

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActivePage {
    Workspace,
    Settings,
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

pub struct MemphisApp {
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
    active_page: ActivePage,
    settings_state: SettingsState,
}

impl MemphisApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        configure_visuals(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);

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
                    summary: "Split UI into app/components/settings modules",
                    kind: "Modified",
                },
                ChangeItem {
                    path: "src/settings_page.rs",
                    summary: "Add dedicated settings page and controls",
                    kind: "Added",
                },
                ChangeItem {
                    path: "src/components.rs",
                    summary: "Extract color palette, icon helpers, and shared widgets",
                    kind: "Added",
                },
            ],
            checks: vec![
                CheckItem {
                    name: "cargo check",
                    status: "Pending",
                    detail: "Run after editing to verify compilation.",
                },
                CheckItem {
                    name: "cargo fmt --all",
                    status: "Pending",
                    detail: "Normalize formatting for new module split.",
                },
                CheckItem {
                    name: "Settings fidelity review",
                    status: "In Progress",
                    detail: "Calibrating spacing/toggles against screenshot.",
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
                    text: "Branched d0ublecl1ck/settings-page from origin/main".to_owned(),
                },
                Message {
                    role: MessageRole::Assistant,
                    text: "Added a dedicated settings page with modularized files".to_owned(),
                },
            ],
            status_text: "交互模式已启用".to_owned(),
            active_page: ActivePage::Workspace,
            settings_state: SettingsState::new(),
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
                self.active_page = ActivePage::Workspace;
                self.status_text = format!("已切换到工作区：{}", item);
                self.append_assistant_message(format!("工作区已切换到 {}。", item));
            }
            ui.add_space(3.0);
        }

        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            if ui
                .add(
                    Button::new(
                        RichText::new("Settings")
                            .color(if self.active_page == ActivePage::Settings {
                                colors::TEXT
                            } else {
                                colors::SUBTLE
                            })
                            .size(13.0),
                    )
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE),
                )
                .clicked()
            {
                self.active_page = ActivePage::Settings;
                self.status_text = "已进入设置页".to_owned();
            }
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
            bottom_panel_content(ui, self.bottom_tab, &self.status_text);
        });
    }

    fn draw_workspace_center(&mut self, ui: &mut Ui) {
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
                Frame::new()
                    .fill(colors::SURFACE)
                    .inner_margin(Margin::symmetric(16, 8))
                    .stroke(Stroke::new(1.0, colors::BORDER)),
            )
            .show(ctx, |ui| self.draw_top_bar(ui));

        SidePanel::left("workspace_sidebar")
            .exact_width(268.0)
            .resizable(false)
            .frame(
                Frame::new()
                    .fill(colors::SURFACE)
                    .inner_margin(Margin::symmetric(16, 12))
                    .stroke(Stroke::new(1.0, colors::BORDER)),
            )
            .show(ctx, |ui| self.draw_left_sidebar(ui));

        if self.active_page == ActivePage::Workspace {
            SidePanel::right("files_sidebar")
                .exact_width(370.0)
                .resizable(false)
                .frame(
                    Frame::new()
                        .fill(colors::PANEL)
                        .inner_margin(Margin::symmetric(14, 12))
                        .stroke(Stroke::new(1.0, colors::BORDER)),
                )
                .show(ctx, |ui| self.draw_right_sidebar(ui));
        }

        CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(colors::BACKGROUND)
                    .inner_margin(Margin::symmetric(20, 12)),
            )
            .show(ctx, |ui| match self.active_page {
                ActivePage::Workspace => self.draw_workspace_center(ui),
                ActivePage::Settings => {
                    if draw_settings_page(ui, &mut self.settings_state) {
                        self.active_page = ActivePage::Workspace;
                        self.status_text = "已返回工作台".to_owned();
                    }
                }
            });
    }
}

pub fn run() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1320.0, 820.0])
            .with_min_inner_size([1100.0, 720.0])
            .with_title("memphis-v2"),
        ..Default::default()
    };

    eframe::run_native(
        "memphis-v2",
        options,
        Box::new(|cc| Ok(Box::new(MemphisApp::new(cc)))),
    )
}

fn tab_strip(ui: &mut Ui, current_file: &str) {
    Frame::new()
        .fill(colors::SURFACE)
        .corner_radius(CornerRadius::same(10))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::symmetric(14, 8))
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

fn change_row(ui: &mut Ui, item: &ChangeItem) {
    Frame::new()
        .fill(colors::PANEL_ALT)
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(10))
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
        "Passed" => colors::SUCCESS,
        "In Progress" => colors::WARNING,
        _ => colors::MUTED,
    };

    Frame::new()
        .fill(colors::PANEL_ALT)
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(item.name).color(colors::TEXT).size(14.0));
                trailing_label(ui, item.status, status_color);
            });
            ui.add_space(4.0);
            ui.label(RichText::new(item.detail).color(colors::SUBTLE).size(13.0));
        });
}

fn bottom_panel_content(ui: &mut Ui, bottom_tab: BottomTab, status_text: &str) {
    Frame::new()
        .fill(colors::PANEL_ALT)
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| match bottom_tab {
            BottomTab::Terminal => {
                ui.label(
                    RichText::new("memphis-v2  d0ublecl1ck/settings-page  ready")
                        .color(colors::TEXT)
                        .size(12.5),
                );
                ui.add_space(8.0);
                icon(ui, AppIcon::Terminal, 14.0);
                ui.label(RichText::new(status_text).color(colors::SUBTLE).size(13.0));
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
    Frame::new()
        .fill(colors::SURFACE)
        .corner_radius(CornerRadius::same(12))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                icon(ui, AppIcon::FileText, 14.0);
                ui.label(RichText::new("Preview").color(colors::TEXT).size(14.0));
                trailing_label(ui, file.name, colors::SUBTLE);
            });
            ui.add_space(10.0);
            Frame::new()
                .fill(colors::PANEL)
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(12))
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

    Frame::new()
        .fill(fill)
        .corner_radius(CornerRadius::same(if message.role == MessageRole::System {
            14
        } else {
            10
        }))
        .stroke(Stroke::new(
            if border == Color32::TRANSPARENT {
                0.0
            } else {
                1.0
            },
            border,
        ))
        .inner_margin(Margin::same(14))
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

    Frame::new()
        .fill(colors::SURFACE)
        .corner_radius(CornerRadius::same(12))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::same(16))
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
                            Button::image(icon_image(AppIcon::SendHorizontal, 15.0))
                                .fill(colors::INPUT_ACTION)
                                .corner_radius(CornerRadius::same(8)),
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
