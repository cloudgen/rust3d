//! rust3d builds one glTF model and one HTML viewer from outline images.
//! requirement-domain-rust3d

pub const VERSION: &str = "1.0.1";
pub const APP_NAME: &str = "rust3d";
pub const CONSOLE_NAME: &str = "rust3d";
pub const AUTHOR_NAME: &str = "Wilgat Wong";
pub const AUTHOR_EMAIL: &str = "wilgat.wong@gmail.com";
pub const HOMEPAGE: &str = "https://github.com/cloudgen/rust3d";
pub const LAST_UPDATE: &str = "2026-10-11";
pub const DOMAIN_SENTENCE: &str =
    "Build a glTF model and an HTML viewer from outline images in a folder";
pub const BASIC_USAGE: &str = "rust3d model";

pub mod about;
pub mod cli;
pub mod lang_data;
pub mod language;
pub mod menu;
pub mod model;
pub mod self_manage;
pub mod status_log;
pub mod system_log;

use std::sync::Mutex;

/// Tests that change the working directory or the process environment take this lock.
pub static PROCESS_LOCK: Mutex<()> = Mutex::new(());
