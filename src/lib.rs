use std::{fmt::Debug, ops::Deref};

use eframe::{
    egui::TextStyle,
    epaint::{FontFamily, FontId},
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

    pub fn with_small_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.small = (size, family);
        self
    }

    pub fn with_body_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.body = (size, family);
        self
    }

    pub fn with_monospace_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.monospace = (size, family);
        self
    }

    pub fn with_button_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.heading = (size, family);
        self
    }

    pub fn with_heading_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.small = (size, family);
        self
    }
}

pub trait StyleHelper {
    fn set_small_font_style(&self, size: f32, family: FontFamily);
    fn set_body_font_style(&self, size: f32, family: FontFamily);
    fn set_monospace_font_style(&self, size: f32, family: FontFamily);
    fn set_button_font_style(&self, size: f32, family: FontFamily);
    fn set_heading_font_style(&self, size: f32, family: FontFamily);
    fn set_font_styles(&self, styles: FontStyles);
    fn divide_font_sizes_by(&self, by: f32);

    fn with_small_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_body_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_monospace_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_button_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_heading_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_font_styles(self, styles: FontStyles) -> Self;
}

impl StyleHelper for eframe::egui::Context {
    fn set_small_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Small).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_body_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Body).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_monospace_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Monospace).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_button_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Button).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_heading_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Heading).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_font_styles(&self, styles: FontStyles) {
        self.set_small_font_style(styles.small.0, styles.small.1);
        self.set_body_font_style(styles.body.0, styles.body.1);
        self.set_monospace_font_style(styles.monospace.0, styles.monospace.1);
        self.set_button_font_style(styles.button.0, styles.button.1);
        self.set_heading_font_style(styles.heading.0, styles.heading.1);
    }

    fn divide_font_sizes_by(&self, by: f32) {
        let mut style = self.style().deref().clone();
        for font in &mut style.text_styles.values_mut() {
            font.size /= by;
        }
        self.set_style(style)
    }

    fn with_small_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_small_font_style(size, family);
        self
    }

    fn with_body_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_body_font_style(size, family);
        self
    }

    fn with_monospace_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_monospace_font_style(size, family);
        self
    }

    fn with_button_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_button_font_style(size, family);
        self
    }

    fn with_heading_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_heading_font_style(size, family);
        self
    }

    fn with_font_styles(self, styles: FontStyles) -> Self {
        self.set_font_styles(styles);
        self
    }
}

pub const REPO: &str = "https://github.com/LyonSyonII/steam-deck-tools";
pub const REPO_RAW: &str = "https://raw.githubusercontent.com/LyonSyonII/steam-deck-tools/main";

pub trait ExpectRepo<T, E> {
    fn expect_repo(self, msg: &str) -> T;
}

impl<T, E: Debug> ExpectRepo<T, E> for Result<T, E> {
    fn expect_repo(self, msg: &str) -> T {
        let msg = &format!("Unexpected error: {msg}. Please open an issue on {REPO}");
        self.expect(msg)
    }
}

use curl::easy::{Handler, WriteError};

struct Collector(String);

impl Collector {
    fn new() -> Collector {
        Collector(String::new())
    }
}

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.push_str(std::str::from_utf8(data).expect_repo(""));
        Ok(data.len())
    }
}

pub fn download_from_repo(file: impl AsRef<str>) -> String {
    let file = file.as_ref();
    println!("Downloading latest '{file}' from {REPO}...");
    let mut handle = curl::easy::Easy2::new(Collector::new());
    let url = format!("{REPO_RAW}/{file}");
    handle.url(&url).expect_repo(&format!("{url} is not a valid url"));
    handle.perform().expect_repo(&format!("Failed downloading '{file}' from {REPO}"));
    handle.get_ref().0.to_owned()
}

pub fn install_tool(title: impl AsRef<str>, needs_root: bool) {
    let mut script = if needs_root {
        download_from_repo("install_scripts/needs_root.sh")
    } else {
        String::new()
    };
    let title = title.as_ref();
    let file = format!("install_scripts/{}.sh", title.to_ascii_lowercase());
    script.push_str(&download_from_repo(file));

    std::process::Command::new("konsole")
        .args(["-e", "sh", "-c", &script])
        .output()
        .expect_repo(&format!("Failed running '{title}' install script."));
}
