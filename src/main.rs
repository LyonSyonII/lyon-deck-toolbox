use std::os::unix::process::CommandExt;

use eframe::{
    egui::{
        self as ui, Button, CentralPanel, Frame, Hyperlink, Layout, RichText, ScrollArea, Style,
        TextStyle, Ui,
    },
    emath::Align,
    epaint::Vec2,
};
use serde::{Deserialize, Serialize};
use steam_deck_tools::{ExpectRepo, StyleHelper, REPO, WAIT_KEY};

#[derive(Deserialize, Serialize)]
struct Tool {
    title: String,
    description: String,
    repo: String,
    needs_root: bool,
}

struct App {
    tools: Vec<Tool>,
    enable_install: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext, tools: Vec<Tool>) -> Self {
        let pixels_per_point = cc.integration_info.native_pixels_per_point.unwrap_or(1.);
        cc.egui_ctx.set_style(ui::Style::default());
        cc.egui_ctx
            .set_small_font_style(16., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx
            .set_body_font_style(22.5, eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx
            .set_heading_font_style(54., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx
            .set_button_font_style(30., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.divide_font_sizes_by(pixels_per_point);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        App {
            tools,
            enable_install: true,
        }
    }
}

fn download_from_repo(file: impl AsRef<std::path::Path>) -> String {
    let file = file.as_ref();
    println!("Downloading latest {file:?} from {REPO}...");
    git_download::repo(REPO)
        .add_file("tools.yaml", "tmp.sdt")
        .branch_name("main")
        .exec()
        .expect_repo(&format!("Failed downloading {file:?}"));
    let input = std::fs::read_to_string("tmp").expect_repo(&format!("Failed opening {file:?}"));
    std::fs::remove_file("tmp.sdt").expect_repo(&format!("Failed removing temporary {file:?}"));
    input
}

#[allow(clippy::field_reassign_with_default)]
fn main() {
    let input = download_from_repo("tools.yaml");
    let tools: Vec<Tool> = serde_yaml::from_str(&input).expect_repo("Failed parsing 'tools.yaml'");
    
    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    //native_options.initial_window_size = Some(Vec2::new(300., 300.));
    eframe::run_native(
        "Steam Deck Tools",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, tools))),
    )
}

fn tool(ui: &mut Ui, app: &App, tool: &Tool) {
    let heading = ui.style().text_styles.get(&TextStyle::Heading).unwrap().size;
    let body = ui.style().text_styles.get(&TextStyle::Body).unwrap().size;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(&tool.title).strong().size(heading * 0.67));
            if ui
                .add_enabled(app.enable_install, Button::new(RichText::new("Install")))
                .clicked()
            {
                //println!("Downloading {} install script...", tool.title);
                let file = format!("install_scripts/{}.sh", tool.title.to_ascii_lowercase());
                let script = download_from_repo(file);
                std::process::Command::new("konsole")
                    .args(["--hold", "-e", "sh", "-c", &script])
                    .spawn()
                    .unwrap();
            }
        });
        ui.label(RichText::from(&tool.description));
        ui.add(Hyperlink::from_label_and_url(
            RichText::new("Repo").small(),
            &tool.repo,
        ));
    });
    ui.separator();
}

fn tools(ui: &mut Ui, app: &App) {
    ui.separator();
    ScrollArea::vertical().show(ui, |ui| {
        for t in &app.tools {
            tool(ui, app, t);
        }
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Steam Deck Tools")
                        .underline()
                        .strong()
                        .heading(),
                );
                ui.label(
                    RichText::new("Click the 'Install' button of each tool to install it.").small(),
                );
            });

            tools(ui, self);
        });
    }
}
