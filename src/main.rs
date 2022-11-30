use eframe::{
    egui::{self as ui, Button, Hyperlink, Layout, RichText, ScrollArea, Ui, CentralPanel, Frame, Style, TextStyle},
    emath::Align,
    epaint::Vec2,
};
use steam_deck_tools::{ StyleHelper, REPO, ExpectRepo};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct Tool {
    title: String,
    description: String,
    repo: String,
    needs_root: bool,
    install_script: String
}

struct App {
    tools: Vec<Tool>,
    enable_install: bool
}

impl App {
    fn new(cc: &eframe::CreationContext, tools: Vec<Tool>) -> Self {
        let pixels_per_point = cc.integration_info.native_pixels_per_point.unwrap_or(1.);
        cc.egui_ctx.set_style(ui::Style::default());
        cc.egui_ctx.set_small_font_style(16., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_body_font_style(22.5, eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_heading_font_style(54., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_button_font_style(30., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.divide_font_sizes_by(pixels_per_point);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        App { tools, enable_install: true }
    }
    
    fn install_tools(&self, all: bool) {
        
    }
}

#[allow(clippy::field_reassign_with_default)]
fn main() {
    println!("Downloading latest tools from {REPO}...");
    git_download::repo("https://github.com/LyonSyonII/steam-deck-tools").add_file("tools.yaml", "tools.yaml").branch_name("main").exec().expect_repo("Failed downloading 'tools.yaml'");
    let input = std::fs::read("tools.yaml").expect_repo("Failed opening 'tools.yaml'");
    std::fs::remove_file("tools.yaml").expect_repo("Failed removing temporary 'tools.yaml'");
    let tools: Vec<Tool> = serde_yaml::from_slice(input.as_slice()).expect_repo("Failed parsing 'tools.yaml'");
    //std::fs::write("tools.yaml", yaml).unwrap();

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
            if ui.add_enabled(app.enable_install, Button::new(RichText::new("Install"))).clicked() { 
                std::process::Command::new("sh").arg("-c").arg(&tool.install_script).output().expect_repo(&format!("Failed running installer for {}", tool.title));
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
                ui.label(RichText::new("Steam Deck Tools").underline().strong().heading());
                ui.label(RichText::new("Click the 'Install' button of each tool to install it.").small());
            });
            
            tools(ui, self);
        });
    }
}
