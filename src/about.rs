//! About page: identity, host check, and the star box.
//! requirement-rust-about. The page stays in English.

use crate::status_log::{home_dir, StatusLog};
use crate::{APP_NAME, AUTHOR_NAME, BASIC_USAGE, CONSOLE_NAME, DOMAIN_SENTENCE, HOMEPAGE, LAST_UPDATE, VERSION};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CheckSystem {
    pub app_name: String,
    pub version: String,
    pub console_name: String,
}

impl CheckSystem {
    pub fn new(logger: Option<&StatusLog>, app_name: &str, version: &str, console_name: &str) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "CheckSystem");
        }
        Self {
            app_name: app_name.to_string(),
            version: version.to_string(),
            console_name: console_name.to_string(),
        }
    }

    pub fn current_user(&self) -> String {
        match std::env::var("USER") {
            Ok(user) if !user.trim().is_empty() => user,
            _ => String::new(),
        }
    }

    fn cache_home() -> Option<PathBuf> {
        match std::env::var("HOME") {
            Ok(home) if !home.trim().is_empty() => Some(PathBuf::from(home)),
            _ => None,
        }
    }

    pub fn shell_text(&self) -> String {
        std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn os_text(&self) -> String {
        if let Ok(text) = fs::read_to_string("/etc/os-release") {
            for line in text.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    return value.trim_matches('"').to_string();
                }
            }
        }
        std::env::consts::OS.to_string()
    }

    pub fn arch_label(&self) -> String {
        std::env::consts::ARCH.to_string()
    }

    pub fn libc_label(&self) -> String {
        if Path::new("/lib/x86_64-linux-gnu/libc.so.6").exists()
            || Path::new("/lib64/libc.so.6").exists()
        {
            return "GNU libc".to_string();
        }
        "unknown".to_string()
    }

    pub fn rustc_version(&self) -> String {
        command_stdout("rustc", &["--version"]).unwrap_or_else(|| "not found".to_string())
    }

    pub fn target_triple(&self) -> String {
        if let Some(text) = command_stdout("rustc", &["-vV"]) {
            for line in text.lines() {
                if let Some(host) = line.strip_prefix("host: ") {
                    return host.trim().to_string();
                }
            }
        }
        format!("{}-unknown-{}", self.arch_label(), std::env::consts::OS)
    }

    pub fn command_location(&self, name: &str) -> String {
        which(name).unwrap_or_default()
    }

    pub fn inside_docker(&self) -> bool {
        Path::new("/.dockerenv").exists()
    }

    pub fn binary_type(&self) -> String {
        format!("{}-{}", self.arch_label(), self.libc_label())
    }

    pub fn self_location(&self) -> String {
        std::env::current_exe()
            .map(|path| path.display().to_string())
            .unwrap_or_default()
    }

    pub fn process_id(&self) -> String {
        std::process::id().to_string()
    }

    fn cache_leaf(&self, username: &str, pid: &str) -> String {
        if username.is_empty() {
            format!("cache-{}-{}", self.app_name, pid)
        } else {
            format!("cache-{}-{}-{}", self.app_name, username, pid)
        }
    }

    pub fn cache_folder_preferred(&self) -> String {
        Path::new("/dev/shm")
            .join("cache")
            .join(self.cache_leaf(&self.current_user(), &self.process_id()))
            .display()
            .to_string()
    }

    pub fn cache_folder_first_fallback(&self) -> String {
        Path::new("/tmp")
            .join("cache")
            .join(self.cache_leaf(&self.current_user(), &self.process_id()))
            .display()
            .to_string()
    }

    pub fn cache_folder_second_fallback(&self) -> String {
        let Some(home) = Self::cache_home() else {
            return String::new();
        };
        home.join(".cache")
            .join(format!("cache-{}-{}", self.app_name, self.process_id()))
            .display()
            .to_string()
    }

    pub fn cache_folder_used(&self) -> String {
        if Path::new("/dev/shm").is_dir() {
            return self.cache_folder_preferred();
        }
        if Path::new("/tmp").is_dir() {
            return self.cache_folder_first_fallback();
        }
        self.cache_folder_second_fallback()
    }

    pub fn persistence_storage(&self) -> String {
        let Some(home) = Self::cache_home() else {
            return String::new();
        };
        home.join(".local").join(&self.app_name).display().to_string()
    }

    pub fn tty_interactive(&self) -> &'static str {
        use std::io::IsTerminal;
        if std::io::stdout().is_terminal() {
            "yes"
        } else {
            "no"
        }
    }

    pub fn is_source_checkout(&self, path: &str) -> bool {
        if !path.contains("/src/") && !path.contains("/target/") {
            return false;
        }
        let mut cursor = PathBuf::from(path);
        for _ in 0..8 {
            if cursor.join("Cargo.toml").is_file() {
                return true;
            }
            if !cursor.pop() {
                return false;
            }
        }
        false
    }

    pub fn install_kind(&self, path: &str) -> &'static str {
        let resolved = fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
        let text = resolved.display().to_string();
        if self.is_source_checkout(&text) {
            return "uninstalled";
        }
        let home = home_dir();
        let home_text = home.display().to_string();
        let in_home = text == home_text || text.starts_with(&(home_text + "/"));
        let base = resolved.file_name().and_then(|name| name.to_str()).unwrap_or("");
        if in_home && (text.contains("/.cargo/") || base == self.console_name || base == self.app_name) {
            return "local";
        }
        if text.starts_with("/usr/") || text.starts_with("/opt/") || text.starts_with("/bin/") {
            return "global";
        }
        "uninstalled"
    }

    pub fn check_system_lines(&self, stamp: Option<&str>) -> Vec<String> {
        let stamp = stamp
            .map(|item| item.to_string())
            .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string());
        let arch = self.arch_label();
        vec![
            format!("{stamp} {} (v{})  [CHECK SYSTEM]:", self.app_name, self.version),
            "  Now checking your operation system!".to_string(),
            format!("    Rustc: {}", self.rustc_version()),
            format!("    C Library: {}", self.libc_label()),
            format!("    Operation System: {}", self.os_text()),
            format!("    Architecture: {arch}"),
            format!("    Current User: {}", self.current_user()),
            format!("    Shell: {}", self.shell_text()),
            format!("    Rustc Executable: {}", self.command_location("rustc")),
            format!("    cargo location: {}", self.command_location("cargo")),
            format!("    rustup location: {}", self.command_location("rustup")),
            format!("    Inside docker container: {}", self.inside_docker()),
            format!("    Target triple: {}", self.target_triple()),
            format!("    Binary Type: {}", self.binary_type()),
            format!("    Location: {}", self.self_location()),
            format!("    PID: {}", self.process_id()),
            format!("    Cache folder used: {}", self.cache_folder_used()),
            format!("    Cache folder (preferred): {}", self.cache_folder_preferred()),
            format!("    Cache folder (1st fallback): {}", self.cache_folder_first_fallback()),
            format!("    Cache folder (2nd fallback): {}", self.cache_folder_second_fallback()),
            format!("    Persistence storage: {}", self.persistence_storage()),
            format!("    TTY / Interactive: {}", self.tty_interactive()),
        ]
    }
}

pub struct AboutPage {
    pub check: CheckSystem,
    pub app_name: String,
    pub version: String,
    pub author: String,
    pub updated: String,
    pub homepage: String,
    pub download_url: String,
    pub usage: String,
    pub console_name: String,
}

impl AboutPage {
    pub fn product(logger: Option<&StatusLog>) -> Self {
        Self::new(
            CheckSystem::new(logger, APP_NAME, VERSION, CONSOLE_NAME),
            APP_NAME,
            VERSION,
            AUTHOR_NAME,
            LAST_UPDATE,
            HOMEPAGE,
            "",
            BASIC_USAGE,
            CONSOLE_NAME,
            logger,
        )
    }

    pub fn new(
        check: CheckSystem,
        app_name: &str,
        version: &str,
        author: &str,
        updated: &str,
        homepage: &str,
        download_url: &str,
        usage: &str,
        console_name: &str,
        logger: Option<&StatusLog>,
    ) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "AboutPage");
        }
        Self {
            check,
            app_name: app_name.to_string(),
            version: version.to_string(),
            author: author.to_string(),
            updated: updated.to_string(),
            homepage: homepage.to_string(),
            download_url: download_url.to_string(),
            usage: usage.to_string(),
            console_name: console_name.to_string(),
        }
    }

    pub fn install_sentence(&self, kind: &str) -> &'static str {
        match kind {
            "global" => "You are using the GLOBAL INSTALLED version, location:",
            "local" => "You are using the LOCAL INSTALLED version, location:",
            _ => "You are using an UNINSTALLED version, location:",
        }
    }

    pub fn about_box_lines(&self, location: Option<&str>, kind: Option<&str>, download_url: Option<&str>) -> Vec<String> {
        let location = location.unwrap_or("").to_string();
        let location = if location.is_empty() {
            self.check.self_location()
        } else {
            location
        };
        let kind = kind.unwrap_or_else(|| self.check.install_kind(&location));
        let download_url = download_url.unwrap_or(&self.download_url);
        let msg1 = format!("{} ({}) by {} on {}", self.app_name, self.version, self.author, self.updated);
        let mut msg = vec![
            msg1,
            String::new(),
            self.install_sentence(kind).to_string(),
            format!("    {location}"),
            String::new(),
            "Basic Usage:".to_string(),
            format!("    {}", self.usage),
            String::new(),
            "Please visit our homepage: ".to_string(),
            format!("    \"{}\"", self.homepage),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        if !download_url.is_empty() {
            msg = vec![
                format!("{} ({}) by {} on {}", self.app_name, self.version, self.author, self.updated),
                String::new(),
                self.install_sentence(kind).to_string(),
                format!("    {location}"),
                String::new(),
                "Basic Usage:".to_string(),
                format!("    {}", self.usage),
                String::new(),
                "Please visit our homepage: ".to_string(),
                format!("    \"{}\"", self.homepage),
                String::new(),
                "Installation command:".to_string(),
                format!("    cargo install {}", self.app_name),
                String::new(),
            ];
        }
        let max_line = if download_url.is_empty() {
            if self.homepage.is_empty() { msg.len().saturating_sub(3) } else { msg.len().saturating_sub(2) }
        } else {
            msg.len().saturating_sub(1)
        };
        let max_line = max_line.max(1).min(msg.len());
        let max_len = msg.iter().take(max_line).map(|line| line.chars().count()).max().unwrap_or(0);
        let star = "*".repeat(max_len + 4);
        let blank = " ".repeat(max_len);
        let mut lines = vec![star.clone(), format!("* {blank} *")];
        for row in msg.iter().take(max_line) {
            let pad = " ".repeat(max_len - row.chars().count());
            lines.push(format!("* {row}{pad} *"));
        }
        lines.push(format!("* {blank} *"));
        lines.push(star);
        lines
    }

    pub fn framework_about(&self) -> String {
        let mut lines = vec![
            format!("{} {}", self.app_name, self.version),
            format!("Domain: {DOMAIN_SENTENCE}"),
            "Runtime tools: none".to_string(),
            format!("Entry points: {}", self.console_name),
            String::new(),
        ];
        lines.extend(self.check.check_system_lines(None));
        lines.push(String::new());
        lines.extend(self.about_box_lines(None, None, None));
        lines.join("\n")
    }
}

fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() && output.stdout.is_empty() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

fn which(name: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate.display().to_string());
        }
    }
    None
}
