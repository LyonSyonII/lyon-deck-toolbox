use eframe::{
    egui::{self as ui, CentralPanel, Hyperlink, RichText, ScrollArea, TextStyle, Ui},
    epaint::Vec2,
};
use serde::Deserialize;
use steam_deck_tools::{download_from_repo, install_tool, ExpectRepo, StyleHelper, UiHelper};

#[derive(Deserialize)]
struct Tool {
    title: String,
    description: String,
    repo: String,
    needs_root: bool,
}

#[allow(dead_code)]
struct App {
    tools: Vec<Tool>,
}

impl App {
    fn new(cc: &eframe::CreationContext, tools: Vec<Tool>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.);
        cc.egui_ctx.set_style(ui::Style::default());
        cc.egui_ctx.set_small_font_style(16.);
        cc.egui_ctx.set_body_font_style(22.5);
        cc.egui_ctx.set_heading_font_style(54.);
        cc.egui_ctx.set_button_font_style(30.);
        //cc.egui_ctx.divide_font_sizes_by(pixels_per_point);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        App {
            tools
        }
    }

    fn tool(&self, ui: &mut Ui, tool: &Tool) {
        // SAFETY: unwrap is safe, text_styles is guaranteed to contain TextStyle::Heading
        let heading = ui
            .style()
            .text_styles
            .get(&TextStyle::Heading)
            .unwrap()
            .size;
        // SAFETY: unwrap is safe, text_styles is guaranteed to contain TextStyle::Body
        let _body = ui.style().text_styles.get(&TextStyle::Body).unwrap().size;
        let description = tool.description.replace("\\n", "\n");

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new(&tool.title).strong().size(heading * 0.67));
                ui.label(description);
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Repo").small(),
                    &tool.repo,
                ));
            });
            // SAFETY: 'columns' array is guaranteed to have 2 elements
            if columns[1].button_sized("Install", columns[0].min_size()).clicked() {
                // TODO! Show error window/popup if error is found
                install_tool(&tool.title, tool.needs_root).unwrap();
            }
        });
    }

    fn tools(&self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            for t in &self.tools {
                ui.separator();
                ui.add_space(15.);
                self.tool(ui, t);
                ui.add_space(15.);
            }
            ui.separator();
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Steam Deck Tools")
                        .heading()
                        .underline()
                        .strong()
                );
                ui.small("Click the 'Install' button of each tool to install it.")
            });
            ui.add_space(20.);
            
            self.tools(ui);
        });
    }
}

#[allow(clippy::field_reassign_with_default)]
fn main() -> anyhow::Result<()> {
    let input = download_from_repo("tools.yaml")?;
    println!("Parsing 'tools.yaml'");
    let tools: Vec<Tool> = serde_yaml::from_str(&input).repo_context("Failed parsing 'tools.yaml'")?;
    println!("Starting GUI");
    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    native_options.initial_window_size = Some(Vec2::new(1280., 800.));
    eframe::run_native(
        "Steam Deck Tools",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, tools))),
    );
    Ok(())
}