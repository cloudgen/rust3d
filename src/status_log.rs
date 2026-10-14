//! In-tree status log. requirement-rust-cli-logging.
//! One process writes one StatusLog. The daily file keeps the lines.

use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

pub const LOGGER_VERSION: &str = "1.0.1";

pub struct StatusLog {
    log_name: String,
    base_dir: PathBuf,
    log_dir: PathBuf,
    quiet: AtomicBool,
    debug: bool,
}

impl StatusLog {
    /// Construct one logger. `logname` is the product name. An empty folder
    /// argument leaves resolution to this function.
    pub fn new(logname: &str, is_quiet: bool, basedir: &str, logdir: &str) -> Self {
        let log_name = kebab(logname);
        let base_dir = if !basedir.is_empty() {
            PathBuf::from(basedir)
        } else if is_root() {
            PathBuf::from(format!("/var/{log_name}"))
        } else {
            home_dir().join(".local").join(&log_name)
        };
        let log_dir = if !logdir.is_empty() {
            PathBuf::from(logdir)
        } else {
            base_dir.join("log")
        };
        let debug = debug_from_env();
        let logger = Self {
            log_name,
            base_dir,
            log_dir,
            quiet: AtomicBool::new(is_quiet),
            debug,
        };
        if let Ok(created) = ensure_dir(&logger.log_dir) {
            if created && !logger.is_quiet() {
                println!("Created directory: {}", logger.log_dir.display());
            }
        }
        logger
    }

    pub fn class_version() -> &'static str {
        LOGGER_VERSION
    }

    pub fn log_name(&self) -> &str {
        &self.log_name
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn set_quiet(&self, quiet: bool) {
        self.quiet.store(quiet, Ordering::SeqCst);
    }

    pub fn is_quiet(&self) -> bool {
        self.quiet.load(Ordering::SeqCst)
    }

    pub fn log_message(&self, message: &str, level: &str, component: &str) {
        let level = if level.is_empty() {
            "INFO".to_string()
        } else {
            level.to_ascii_uppercase()
        };
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!(
            "[{now}] pid:{} [{level}] @{component} :] {message}",
            std::process::id()
        );
        let _ = append_daily(&self.log_dir, &self.log_name, &line);
        if self.is_quiet() {
            return;
        }
        if level == "ERROR" || level == "FATAL" {
            eprintln!("{line}");
        } else {
            println!("{line}");
        }
    }
}

fn kebab(name: &str) -> String {
    let mut out = String::new();
    for (index, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0 {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn debug_from_env() -> bool {
    for key in ["DEBUG", "debug"] {
        if let Ok(value) = std::env::var(key) {
            let token = value.to_ascii_lowercase();
            if token == "1" || token == "true" || token == "show" {
                return true;
            }
        }
    }
    false
}

fn is_root() -> bool {
    #[cfg(unix)]
    {
        libc_geteuid() == 0
    }
    #[cfg(not(unix))]
    {
        false
    }
}

#[cfg(unix)]
fn libc_geteuid() -> u32 {
    extern "C" {
        fn geteuid() -> u32;
    }
    unsafe { geteuid() }
}

pub fn home_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        if !home.trim().is_empty() {
            return PathBuf::from(home);
        }
    }
    PathBuf::from(".")
}

fn ensure_dir(path: &Path) -> std::io::Result<bool> {
    if path.is_dir() {
        return Ok(false);
    }
    fs::create_dir_all(path)?;
    Ok(true)
}

fn append_daily(log_dir: &Path, name: &str, line: &str) -> std::io::Result<()> {
    if !log_dir.is_dir() {
        return Ok(());
    }
    let stamp = Local::now().format("%Y%m%d");
    let path = log_dir.join(format!("{name}-{stamp}.log"));
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{line}")?;
    Ok(())
}
