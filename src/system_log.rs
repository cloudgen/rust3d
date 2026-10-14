//! Log folder actions. requirement-rust-cli-logging.

use crate::status_log::StatusLog;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct SystemLog {
    logger: Option<Rc<StatusLog>>,
    log_dir: PathBuf,
}

impl SystemLog {
    pub fn new(logger: Option<Rc<StatusLog>>) -> Self {
        if let Some(logger) = &logger {
            logger.log_message("instantiated", "INFO", "SystemLog");
        }
        let log_dir = logger
            .as_ref()
            .map(|item| item.log_dir().to_path_buf())
            .unwrap_or_default();
        Self { logger, log_dir }
    }

    pub fn log_dir(&self) -> String {
        if self.log_dir.as_os_str().is_empty() {
            String::new()
        } else {
            self.log_dir.display().to_string()
        }
    }

    pub fn log_files(&self) -> Vec<PathBuf> {
        let root = match self.log_dir.canonicalize() {
            Ok(path) if path.is_dir() => path,
            _ => return Vec::new(),
        };
        let mut found = Vec::new();
        let Ok(entries) = fs::read_dir(&root) else {
            return found;
        };
        for entry in entries.flatten() {
            if let Some(owned) = self.owned(&entry.path()) {
                found.push(owned);
            }
        }
        found.sort_by_key(|item| item.file_name().map(|name| name.to_os_string()).unwrap_or_default());
        found
    }

    pub fn read_log(&self, path: &Path) -> Option<String> {
        let owned = self.owned(path)?;
        self.note("read log", &owned);
        fs::read_to_string(&owned).ok()
    }

    pub fn clear_log(&self, path: &Path) -> bool {
        let Some(owned) = self.owned(path) else {
            return false;
        };
        self.note("clear log", &owned);
        fs::write(&owned, "").is_ok()
    }

    pub fn folder_text(&self) -> String {
        let folder = self.log_dir();
        if folder.is_empty() {
            "No log folder.".to_string()
        } else {
            format!("Log folder: {folder}")
        }
    }

    fn owned(&self, path: &Path) -> Option<PathBuf> {
        if self.log_dir.as_os_str().is_empty() {
            return None;
        }
        let root = self.log_dir.canonicalize().ok()?;
        let candidate = path.canonicalize().ok()?;
        if candidate.parent() != Some(root.as_path()) {
            return None;
        }
        if candidate.extension().and_then(|ext| ext.to_str()) != Some("log") {
            return None;
        }
        if !candidate.is_file() {
            return None;
        }
        Some(candidate)
    }

    fn note(&self, operation: &str, path: &Path) {
        if let Some(logger) = &self.logger {
            logger.log_message(&format!("{operation} path={}", path.display()), "INFO", "menu");
        }
    }
}
