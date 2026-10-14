//! Cargo lifecycle for the self-management verbs.
//! requirement-rust-cli-interface. These verbs call cargo. They do not use sudo.

use crate::status_log::StatusLog;
use std::process::{Command, Stdio};
use std::rc::Rc;

pub const DISTRIBUTION: &str = "rust3d";

pub struct SelfManage {
    pub app_name: String,
    pub version: String,
    logger: Option<Rc<StatusLog>>,
    runner: Option<fn(&[String]) -> (i32, String, String)>,
}

impl SelfManage {
    pub fn new(app_name: &str, version: &str, logger: Option<Rc<StatusLog>>) -> Self {
        if let Some(logger) = &logger {
            logger.log_message("instantiated", "INFO", "SelfManage");
        }
        Self {
            app_name: app_name.to_string(),
            version: version.to_string(),
            logger,
            runner: None,
        }
    }

    pub fn set_runner(&mut self, runner: fn(&[String]) -> (i32, String, String)) {
        self.runner = Some(runner);
    }

    pub fn local_version(&self) -> String {
        format!("{} {}", self.app_name, self.version)
    }

    pub fn argv_for(&self, verb: &str) -> Vec<String> {
        let dist = DISTRIBUTION.to_string();
        match verb {
            "version-check" => vec!["cargo".into(), "search".into(), dist, "--limit".into(), "1".into()],
            "self-update" => vec!["cargo".into(), "install".into(), dist, "--force".into()],
            "self-install" => vec!["cargo".into(), "install".into(), dist],
            "self-uninstall" => vec!["cargo".into(), "uninstall".into(), dist],
            _ => Vec::new(),
        }
    }

    pub fn run_cargo(&self, verb: &str) -> (i32, String) {
        let argv = self.argv_for(verb);
        let (code, out, err) = if let Some(runner) = self.runner {
            runner(&argv)
        } else {
            subprocess(&argv)
        };
        let mut chunks = vec![format!("$ {}", argv.join(" "))];
        if verb == "version-check" {
            chunks.insert(0, self.local_version());
        }
        if !out.trim().is_empty() {
            chunks.push(out.trim().to_string());
        }
        if !err.trim().is_empty() {
            chunks.push(err.trim().to_string());
        }
        if code != 0 && out.trim().is_empty() && err.trim().is_empty() {
            chunks.push(format!("cargo exited {code}"));
        }
        (code, chunks.join("\n"))
    }

    pub fn emit(&self, verb: &str, out: &mut dyn std::io::Write) -> i32 {
        if let Some(logger) = &self.logger {
            logger.log_message(&format!("cargo {verb}"), "INFO", "SelfManage");
        }
        let (code, text) = self.run_cargo(verb);
        let _ = writeln!(out, "{text}");
        code
    }
}

fn subprocess(argv: &[String]) -> (i32, String, String) {
    if argv.is_empty() {
        return (1, String::new(), String::new());
    }
    match Command::new(&argv[0])
        .args(&argv[1..])
        .stdin(Stdio::null())
        .output()
    {
        Ok(done) => (
            done.status.code().unwrap_or(1),
            String::from_utf8_lossy(&done.stdout).to_string(),
            String::from_utf8_lossy(&done.stderr).to_string(),
        ),
        Err(err) => (1, String::new(), err.to_string()),
    }
}
