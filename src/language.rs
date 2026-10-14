//! Menu language. requirement-rust-cli-language.
//! English is the default. LANG is not read. RUST3D_LANG wins and does not write.

use crate::lang_data::{self, CODES};
use crate::status_log::StatusLog;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::PathBuf;

pub const APP_DIR: &str = "rust3d";
pub const LEAF_NAME: &str = "language";
pub const ENV_NAME: &str = "RUST3D_LANG";

pub struct MenuLanguage {
    pub home: String,
    code: String,
}

impl MenuLanguage {
    pub fn new(home: Option<&str>, logger: Option<&StatusLog>) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "MenuLanguage");
        }
        let home = home
            .map(|item| item.to_string())
            .unwrap_or_else(|| std::env::var("HOME").unwrap_or_default());
        let mut language = Self {
            home,
            code: "en".to_string(),
        };
        language.load();
        language
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn codes(&self) -> Vec<&'static str> {
        CODES.iter().map(|row| row.code).collect()
    }

    pub fn endonym(&self, code: &str) -> &'static str {
        CODES
            .iter()
            .find(|row| row.code == code)
            .map(|row| row.endonym)
            .unwrap_or("")
    }

    pub fn code_for_number(&self, number: i32) -> Option<&'static str> {
        CODES.iter().find(|row| row.number == number).map(|row| row.code)
    }

    pub fn leaf_path(&self) -> String {
        if self.home.is_empty() {
            return String::new();
        }
        PathBuf::from(&self.home)
            .join(".local")
            .join(APP_DIR)
            .join(LEAF_NAME)
            .display()
            .to_string()
    }

    pub fn load(&mut self) {
        if let Ok(forced) = std::env::var(ENV_NAME) {
            if self.codes().contains(&forced.as_str()) {
                self.code = forced;
                return;
            }
        }
        self.code = self.read_leaf();
    }

    fn read_leaf(&self) -> String {
        let path = self.leaf_path();
        if path.is_empty() {
            return "en".to_string();
        }
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(_) => return "en".to_string(),
        };
        let line = text.lines().next().unwrap_or("").replace('\r', "");
        let line = line.trim();
        if self.codes().contains(&line) {
            return line.to_string();
        }
        "en".to_string()
    }

    pub fn save(&mut self, code: &str) -> String {
        if !self.codes().contains(&code) {
            return self.text("save_failed");
        }
        let path = self.leaf_path();
        if path.is_empty() {
            return self.text("save_failed");
        }
        let previous = self.code.clone();
        let folder = PathBuf::from(&path);
        let folder = folder.parent().map(|item| item.to_path_buf()).unwrap_or_default();
        let write = (|| {
            fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(&folder)?;
            fs::set_permissions(&folder, fs::Permissions::from_mode(0o700))?;
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&path)?;
            file.write_all(format!("{code}\n").as_bytes())?;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
            Ok::<(), std::io::Error>(())
        })();
        if write.is_err() {
            self.code = previous;
            return self.text("save_failed");
        }
        self.code = code.to_string();
        self.text("saved")
    }

    pub fn text(&self, key: &str) -> String {
        lang_data::lookup_text(&self.code, key)
    }

    pub fn path_label(&self) -> String {
        self.text("path")
    }

    pub fn lang_long(&self, endonym: &str) -> String {
        self.text("lang_long").replace("{name}", endonym)
    }

    pub fn row_short(&self, kind: &str, english: &str) -> String {
        match lang_data::short_key(kind) {
            Some(key) => self.text(key),
            None => english.to_string(),
        }
    }

    pub fn row_long(&self, kind: &str, english: &str) -> String {
        match lang_data::long_key(kind) {
            Some(key) => self.text(key),
            None => english.to_string(),
        }
    }

    pub fn token_kind(&self, token: &str, layer: &str) -> Option<String> {
        if layer == "lang" {
            return lang_data::lang_token(token).map(|code| format!("set-{code}"));
        }
        if layer == "front" {
            return lang_data::front_token(token).map(|kind| kind.to_string());
        }
        None
    }
}
