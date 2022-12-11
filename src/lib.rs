use std::{fmt::Debug, ops::Deref};

use anyhow::{Context, Error, Result, bail};

use eframe::{
    egui::{Button, RichText, TextStyle},
    epaint::{FontFamily, FontId, Vec2},
};

type FontStyle = (f32, FontFamily);
#[derive(Default, Clone, Debug)]
pub struct FontStyles {
    small: (f32, FontFamily),
    body: (f32, FontFamily),
    monospace: (f32, FontFamily),
    button: (f32, FontFamily),
    heading: (f32, FontFamily),
}

impl FontStyles {
    pub fn new(
        small: FontStyle,
        body: FontStyle,
        monospace: FontStyle,
        button: FontStyle,
        heading: FontStyle,
    ) -> FontStyles {
        FontStyles {
            small,
            body,
            monospace,
            button,
            heading,
        }
    }

    pub fn with_small_style(mut self, size: f32) -> FontStyles {
        self.small = (size, FontFamily::Proportional);
        self
    }

    pub fn with_body_style(mut self, size: f32) -> FontStyles {
        self.body = (size, FontFamily::Proportional);
        self
    }

    pub fn with_monospace_style(mut self, size: f32) -> FontStyles {
        self.monospace = (size, FontFamily::Proportional);
        self
    }

    pub fn with_button_style(mut self, size: f32) -> FontStyles {
        self.heading = (size, FontFamily::Proportional);
        self
    }

    pub fn with_heading_style(mut self, size: f32) -> FontStyles {
        self.small = (size, FontFamily::Proportional);
        self
    }
}

pub trait StyleHelper {
    fn set_small_font_style(&self, size: f32);
    fn set_body_font_style(&self, size: f32);
    fn set_monospace_font_style(&self, size: f32);
    fn set_button_font_style(&self, size: f32);
    fn set_heading_font_style(&self, size: f32);
    fn set_font_styles(&self, styles: FontStyles);
    fn divide_font_sizes_by(&self, by: f32);

    fn with_small_font_style(self, size: f32) -> Self;
    fn with_body_font_style(self, size: f32) -> Self;
    fn with_monospace_font_style(self, size: f32) -> Self;
    fn with_button_font_style(self, size: f32) -> Self;
    fn with_heading_font_style(self, size: f32) -> Self;
    fn with_font_styles(self, styles: FontStyles) -> Self;
}

impl StyleHelper for eframe::egui::Context {
    fn set_small_font_style(&self, size: f32) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Small).unwrap() =
            FontId::new(size, FontFamily::Proportional);
        self.set_style(style);
    }

    fn set_body_font_style(&self, size: f32) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Body).unwrap() =
            FontId::new(size, FontFamily::Proportional);
        self.set_style(style);
    }

    fn set_monospace_font_style(&self, size: f32) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Monospace).unwrap() =
            FontId::new(size, FontFamily::Proportional);
        self.set_style(style);
    }

    fn set_button_font_style(&self, size: f32) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Button).unwrap() =
            FontId::new(size, FontFamily::Proportional);
        self.set_style(style);
    }

    fn set_heading_font_style(&self, size: f32) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Heading).unwrap() =
            FontId::new(size, FontFamily::Proportional);
        self.set_style(style);
    }

    fn set_font_styles(&self, styles: FontStyles) {
        self.set_small_font_style(styles.small.0);
        self.set_body_font_style(styles.body.0);
        self.set_monospace_font_style(styles.monospace.0);
        self.set_button_font_style(styles.button.0);
        self.set_heading_font_style(styles.heading.0);
    }

    fn divide_font_sizes_by(&self, by: f32) {
        let mut style = self.style().deref().clone();
        for font in &mut style.text_styles.values_mut() {
            font.size /= by;
        }
        self.set_style(style)
    }

    fn with_small_font_style(self, size: f32) -> Self {
        self.set_small_font_style(size);
        self
    }

    fn with_body_font_style(self, size: f32) -> Self {
        self.set_body_font_style(size);
        self
    }

    fn with_monospace_font_style(self, size: f32) -> Self {
        self.set_monospace_font_style(size);
        self
    }

    fn with_button_font_style(self, size: f32) -> Self {
        self.set_button_font_style(size);
        self
    }

    fn with_heading_font_style(self, size: f32) -> Self {
        self.set_heading_font_style(size);
        self
    }

    fn with_font_styles(self, styles: FontStyles) -> Self {
        self.set_font_styles(styles);
        self
    }
}

pub trait UiHelper {
    fn button_sized(&mut self, label: impl Into<String>, size: Vec2) -> eframe::egui::Response;
}

impl UiHelper for eframe::egui::Ui {
    fn button_sized(&mut self, label: impl Into<String>, size: Vec2) -> eframe::egui::Response {
        self.add_sized(size, Button::new(RichText::new(label.into())))
    }
}

pub const REPO: &str = "https://github.com/LyonSyonII/steam-deck-tools";
pub const REPO_RAW: &str = "https://raw.githubusercontent.com/LyonSyonII/steam-deck-tools/main";

pub trait ExpectRepo<T, E> {
    fn repo_context(self, msg: impl AsRef<str>) -> anyhow::Result<T>;
}

impl<T, E> ExpectRepo<T, E> for Result<T, E>
where
    T: Sync + Send,
    E: Into<anyhow::Error> + Sync + Send + 'static,
{
    fn repo_context(self, msg: impl AsRef<str>) -> anyhow::Result<T> {
        let msg = msg.as_ref();
        let err: Result<T, anyhow::Error> = self.map_err(|e| e.into());
        err.with_context(|| format!("Unexpected error: {msg}. Please open an issue on {REPO}"))
    }
}

pub fn download_from_repo(file: impl AsRef<str>) -> Result<String> {
    let file = file.as_ref();
    let url = format!("{REPO_RAW}/{file}");
    println!("Downloading latest '{file}' from {url}");
    curl(&url).repo_context(format!("Failed downloading '{file}' from {url}"))
}

pub fn install_tool(title: impl AsRef<str>, needs_root: bool) -> Result<()> {
    let mut script = if needs_root {
        download_from_repo("install_scripts/needs_root.sh")?
    } else {
        String::new()
    };
    let title = title.as_ref();
    let file = format!("install_scripts/{}.sh", title.to_ascii_lowercase().replace(' ', ""));
    script.push_str(&download_from_repo(file)?);

    let output = std::process::Command::new("konsole")
        .args(["-e", "sh", "-c", &script])
        .output()
        .repo_context(format!("Failed running '{title}' install script."))?;
    if output.status.success() {
        println!("Success: {:?}", output.status.code());
        Ok(())
    } else {
        println!("Fail: {:?}", output.status.code());
        Err(anyhow::Error::msg("")).repo_context(format!("Failed running '{title} install script'"))
    }
}

pub fn curl(url: &str) -> Result<String> {
    let out = std::process::Command::new("curl")
        .args(["-L", url])
        .output()?;
    String::from_utf8(out.stdout).map_err(Error::msg)
}
