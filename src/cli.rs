//! rust3d CLI. requirement-rust-cli-interface.
//! fn main writes StatusLog::new before the argument walk and before any product object.

use crate::about::AboutPage;
use crate::menu::{
    MenuPainter, MenuSession, TermScreen, TextScreen, Tui, KEY_ENTER, KEY_INTERRUPT,
};
use crate::model::{convert_folder, folder_for_pick, selected_choice_line};
use crate::self_manage::SelfManage;
use crate::status_log::StatusLog;
use crate::system_log::SystemLog;
use crate::{APP_NAME, VERSION};
use std::io::{self, Write};
use std::rc::Rc;
use std::sync::atomic::{AtomicU8, Ordering};

static FORCE_TTY: AtomicU8 = AtomicU8::new(0);

pub fn force_tty(value: Option<bool>) {
    match value {
        None => FORCE_TTY.store(0, Ordering::SeqCst),
        Some(true) => FORCE_TTY.store(1, Ordering::SeqCst),
        Some(false) => FORCE_TTY.store(2, Ordering::SeqCst),
    }
}

pub const PRODUCT_VERBS: &[&str] = &[
    "help",
    "version",
    "about",
    "model",
    "self-install",
    "version-check",
    "self-update",
    "self-uninstall",
];

pub struct Cli {
    pub logger: Option<Rc<StatusLog>>,
    pub about: AboutPage,
    pub self_manage: SelfManage,
    pub system_log: SystemLog,
    pub tui: Tui,
}

impl Cli {
    pub fn new(logger: Option<Rc<StatusLog>>) -> Self {
        if let Some(logger) = &logger {
            logger.log_message("instantiated", "INFO", "Cli");
        }
        let borrowed = logger.as_deref();
        Self {
            about: AboutPage::product(borrowed),
            self_manage: SelfManage::new(APP_NAME, VERSION, logger.clone()),
            system_log: SystemLog::new(logger.clone()),
            tui: Tui::new(None, borrowed),
            logger,
        }
    }

    pub fn stdout_is_tty() -> bool {
        match FORCE_TTY.load(Ordering::SeqCst) {
            1 => return true,
            2 => return false,
            _ => {}
        }
        use std::io::IsTerminal;
        io::stdout().is_terminal()
    }

    pub fn opens_text_screen(argv: &[String]) -> bool {
        if argv
            .iter()
            .any(|token| token == "--help" || token == "-h" || token == "--version")
        {
            return false;
        }
        if !Self::stdout_is_tty() {
            return false;
        }
        let mut verb: Option<&str> = None;
        let mut saw_force = false;
        for token in argv {
            if token == "--force" {
                saw_force = true;
                continue;
            }
            if token.starts_with('-') {
                return false;
            }
            if verb.is_none() {
                verb = Some(token);
                continue;
            }
            return false;
        }
        if saw_force && verb != Some("self-uninstall") {
            return false;
        }
        if verb == Some("about") {
            return true;
        }
        verb.is_none()
    }

    pub fn report_error(
        logger: Option<&StatusLog>,
        message: &str,
        next: &str,
        err: &mut dyn Write,
    ) -> i32 {
        if let Some(logger) = logger {
            logger.log_message(&format!("{message} Next: {next}"), "ERROR", "main");
        }
        let _ = writeln!(err, "ERROR: {message}");
        let _ = writeln!(err, "   Next: {next}");
        1
    }

    pub fn help_text() -> &'static str {
        "rust3d — build a glTF model and an HTML viewer from outline images.\n\
With no arguments on a terminal, opens the text menu.\n\
With no arguments and no terminal, prints this help and stops.\n\
Product verbs: help, version, about, model, self-install,\n\
version-check, self-update, self-uninstall.\n\
help prints this usage. version prints the installed version.\n\
about prints a page. model reads one folder and does not\n\
draw the menu. It writes model.glb and viewer.html in that\n\
folder's model directory. Menu row 1 asks for the current\n\
folder or a subfolder.\n\
model with no folder uses the current directory.\n\
version-check runs: cargo search rust3d --limit 1\n\
self-update runs: cargo install rust3d --force\n\
self-install runs: cargo install rust3d\n\
self-uninstall runs: cargo uninstall rust3d\n\
and needs --force. Empty arguments do not install or update.\n"
    }

    pub fn run(&mut self, argv: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
        if argv
            .iter()
            .any(|token| token == "--help" || token == "-h" || token == "help")
        {
            let _ = write!(out, "{}", Self::help_text());
            return 0;
        }
        if argv.iter().any(|token| token == "--version") {
            let _ = writeln!(out, "{APP_NAME} {VERSION}");
            return 0;
        }
        let parsed = match parse_args(argv) {
            Ok(parsed) => parsed,
            Err(message) => {
                return Self::report_error(self.logger.as_deref(), &message, "rust3d help", err);
            }
        };
        self.dispatch(parsed, out, err)
    }

    fn dispatch(&mut self, args: Args, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
        let verb = args.verb.as_deref();
        if args.force && verb != Some("self-uninstall") {
            return Self::report_error(
                self.logger.as_deref(),
                "--force is only for self-uninstall.",
                "rust3d self-uninstall --force",
                err,
            );
        }
        if (args.views.is_some() || args.grid.is_some() || args.output.is_some())
            && verb != Some("model")
        {
            return Self::report_error(
                self.logger.as_deref(),
                "--views, --grid, and --output are only for model.",
                "rust3d model --grid 96",
                err,
            );
        }
        if args.target.is_some() && verb != Some("model") {
            return Self::report_error(
                self.logger.as_deref(),
                "A folder is only for model.",
                "rust3d model",
                err,
            );
        }
        match verb {
            Some("about") => {
                if Self::stdout_is_tty() {
                    self.open_text_menu(true, err)
                } else {
                    let _ = writeln!(out, "{}", self.about.framework_about());
                    0
                }
            }
            Some("model") => self.verb_model(args, out),
            Some("version") => {
                let _ = writeln!(out, "{}", self.self_manage.local_version());
                0
            }
            Some("self-uninstall") => {
                if !args.force {
                    return Self::report_error(
                        self.logger.as_deref(),
                        "self-uninstall removes this package with cargo.",
                        "rust3d self-uninstall --force",
                        err,
                    );
                }
                self.self_manage.emit("self-uninstall", out)
            }
            Some("version-check") | Some("self-update") | Some("self-install") => {
                self.self_manage.emit(verb.unwrap(), out)
            }
            Some(token) => Self::report_error(
                self.logger.as_deref(),
                &format!("Unknown verb '{token}'."),
                &format!("rust3d help — verbs: {}", PRODUCT_VERBS.join(", ")),
                err,
            ),
            None => {
                if !Self::stdout_is_tty() {
                    let _ = write!(out, "{}", Self::help_text());
                    return 0;
                }
                self.open_text_menu(false, err)
            }
        }
    }

    /// Full-screen menu. Empty argv on a terminal opens the front board.
    /// `about_first` opens the about result page, then the same board.
    fn open_text_menu(&mut self, about_first: bool, err: &mut dyn Write) -> i32 {
        let mut screen = match TermScreen::enter() {
            Ok(screen) => screen,
            Err(_) => {
                return Self::report_error(
                    self.logger.as_deref(),
                    "The text menu could not open on this terminal.",
                    "rust3d help",
                    err,
                );
            }
        };
        let outcome = self.drive_menu(&mut screen, about_first);
        screen.leave();
        match outcome {
            MenuStop::Exit(code) => code,
            MenuStop::Interrupt => 130,
            MenuStop::TooSmall => Self::report_error(
                self.logger.as_deref(),
                "The text screen is too small for the menu and the input box.",
                "rust3d help",
                err,
            ),
        }
    }

    fn drive_menu(&mut self, screen: &mut TermScreen, about_first: bool) -> MenuStop {
        let about_text = self.about.framework_about();
        let version_text = format!("{APP_NAME} {VERSION}");
        let mut session = MenuSession::new(
            APP_NAME,
            VERSION,
            MenuPainter::new(None, self.logger.as_deref()),
            Box::new(move || version_text.clone()),
            Box::new(move || about_text.clone()),
            self.logger.as_deref(),
        );
        if about_first {
            let page = (session.on_about)();
            session.model.show_result(&page);
        }
        loop {
            let (height, width) = screen.getmaxyx();
            if !session.painter.screen_can_hold(height, width) {
                return MenuStop::TooSmall;
            }
            session
                .painter
                .paint(screen, &mut session.model, APP_NAME, VERSION);
            let key = if clock_board(&session.model) {
                match screen.poll_key(1000) {
                    None => continue,
                    Some(key) => key,
                }
            } else {
                screen.read_key()
            };
            if key == KEY_INTERRUPT {
                return MenuStop::Interrupt;
            }
            if key == -1 {
                return MenuStop::Exit(0);
            }
            let action = {
                let painter = &session.painter;
                session.model.apply_key(painter, key, height)
            };
            let Some(action) = action else {
                continue;
            };
            match self.take_menu_action(&mut session, screen, &action) {
                MenuStep::Stay => continue,
                MenuStep::Stop(stop) => return stop,
            }
        }
    }

    fn take_menu_action(
        &mut self,
        session: &mut MenuSession,
        screen: &mut TermScreen,
        action: &str,
    ) -> MenuStep {
        if action == "exit" {
            return MenuStep::Stop(MenuStop::Exit(0));
        }
        if let Some(name) = action.strip_prefix("pick:") {
            return self.pick_folder(session, screen, name);
        }
        match action {
            "view-log" => self.view_log(session, screen),
            "clear-log" => self.clear_log(session, screen),
            "log-folder" => {
                session.model.show_result(&self.system_log.folder_text());
                MenuStep::Stay
            }
            "help" => {
                session.model.show_result(Self::help_text());
                MenuStep::Stay
            }
            "version-check" | "self-update" | "self-install" | "self-uninstall" => {
                let mut buf = Vec::new();
                let _ = self.self_manage.emit(action, &mut buf);
                session.model.show_result(&String::from_utf8_lossy(&buf));
                MenuStep::Stay
            }
            other => {
                let _ = session.take_action(Some(other.to_string()));
                MenuStep::Stay
            }
        }
    }

    fn pick_folder(
        &mut self,
        session: &mut MenuSession,
        screen: &mut TermScreen,
        name: &str,
    ) -> MenuStep {
        let choice = if name == "." { "current" } else { name };
        let sentence = selected_choice_line(choice, "Building the 3D model");
        session.model.phase = "working".into();
        session.model.result_text = sentence;
        session
            .painter
            .paint(screen, &mut session.model, APP_NAME, VERSION);
        let text = match folder_for_pick(name, None) {
            Ok(folder) => {
                let (_code, lines) = convert_folder(
                    &folder.display().to_string(),
                    None,
                    None,
                    None,
                    choice,
                    None::<&mut fn(&str)>,
                );
                lines.join("\n")
            }
            Err(err) => format!("ERROR: {err}\n   Next: rust3d model"),
        };
        session.model.show_result(&text);
        MenuStep::Stay
    }

    fn view_log(&mut self, session: &mut MenuSession, screen: &mut TermScreen) -> MenuStep {
        let folder = self.system_log.log_dir();
        if folder.is_empty() {
            session.model.show_result("No log folder.");
            return MenuStep::Stay;
        }
        let files = self.system_log.log_files();
        if files.is_empty() {
            session
                .model
                .show_result(&format!("No log file in {folder}."));
            return MenuStep::Stay;
        }
        let mut lines = vec!["Log files:".to_string()];
        for (index, path) in files.iter().enumerate() {
            let name = path
                .file_name()
                .and_then(|item| item.to_str())
                .unwrap_or("");
            lines.push(format!("{}. {name}", index + 1));
        }
        lines.push(String::new());
        lines.push("Choose a log file:".to_string());
        loop {
            match self.read_prompt(session, screen, "system-log", &lines) {
                PromptEnd::Stop(stop) => return MenuStep::Stop(stop),
                PromptEnd::Cancel => {
                    self.restore_log_board(session);
                    return MenuStep::Stay;
                }
                PromptEnd::Text(raw) => match raw.parse::<usize>() {
                    Ok(index) if (1..=files.len()).contains(&index) => {
                        let path = &files[index - 1];
                        let name = path
                            .file_name()
                            .and_then(|item| item.to_str())
                            .unwrap_or("")
                            .to_string();
                        let Some(body) = self.system_log.read_log(path) else {
                            session.model.show_result(&format!(
                                "ERROR: {name} is not a log file in the log folder."
                            ));
                            return MenuStep::Stay;
                        };
                        let body = if body.is_empty() {
                            "(empty)".to_string()
                        } else {
                            body
                        };
                        session.model.show_result(&format!("{name}\n\n{body}"));
                        return MenuStep::Stay;
                    }
                    _ => {
                        session.model.error = format!("Enter 1–{}", files.len());
                    }
                },
            }
        }
    }

    fn clear_log(&mut self, session: &mut MenuSession, screen: &mut TermScreen) -> MenuStep {
        let folder = self.system_log.log_dir();
        if folder.is_empty() {
            session.model.show_result("No log folder.");
            return MenuStep::Stay;
        }
        let files = self.system_log.log_files();
        if files.is_empty() {
            session
                .model
                .show_result(&format!("No log file in {folder}."));
            return MenuStep::Stay;
        }
        let mut lines = vec!["Log files:".to_string()];
        for (index, path) in files.iter().enumerate() {
            let name = path
                .file_name()
                .and_then(|item| item.to_str())
                .unwrap_or("");
            lines.push(format!("{}. {name}", index + 1));
        }
        lines.push(String::new());
        lines.push("Choose a log file to clear:".to_string());
        let picked = loop {
            match self.read_prompt(session, screen, "system-log", &lines) {
                PromptEnd::Stop(stop) => return MenuStep::Stop(stop),
                PromptEnd::Cancel => {
                    self.restore_log_board(session);
                    return MenuStep::Stay;
                }
                PromptEnd::Text(raw) => match raw.parse::<usize>() {
                    Ok(index) if (1..=files.len()).contains(&index) => break index - 1,
                    _ => session.model.error = format!("Enter 1–{}", files.len()),
                },
            }
        };
        let path = files[picked].clone();
        let name = path
            .file_name()
            .and_then(|item| item.to_str())
            .unwrap_or("")
            .to_string();
        let question = vec![format!("Clear {name}? (y/n)")];
        loop {
            match self.read_prompt(session, screen, "system-log", &question) {
                PromptEnd::Stop(stop) => return MenuStep::Stop(stop),
                PromptEnd::Cancel => {
                    self.restore_log_board(session);
                    return MenuStep::Stay;
                }
                PromptEnd::Text(raw)
                    if raw.is_empty()
                        || raw.eq_ignore_ascii_case("n")
                        || raw.eq_ignore_ascii_case("no") =>
                {
                    self.restore_log_board(session);
                    return MenuStep::Stay;
                }
                PromptEnd::Text(raw)
                    if raw.eq_ignore_ascii_case("y") || raw.eq_ignore_ascii_case("yes") =>
                {
                    if self.system_log.clear_log(&path) {
                        session.model.show_result(&format!("Cleared {name}."));
                    } else {
                        session.model.show_result(&format!(
                            "ERROR: {name} is not a log file in the log folder."
                        ));
                    }
                    return MenuStep::Stay;
                }
                PromptEnd::Text(_) => {
                    session.model.error = "Please enter y or n".to_string();
                }
            }
        }
    }

    fn restore_log_board(&self, session: &mut MenuSession) {
        session.model.phase = "board".to_string();
        session.model.layer = "log".to_string();
        session.model.focus = "list".to_string();
        session.model.buffer.clear();
        session.model.cursor = 0;
        session.model.error.clear();
    }

    fn read_prompt(
        &self,
        session: &mut MenuSession,
        screen: &mut TermScreen,
        title: &str,
        lines: &[String],
    ) -> PromptEnd {
        session.model.buffer.clear();
        session.model.cursor = 0;
        session.model.focus = "input".to_string();
        loop {
            let (height, width) = screen.getmaxyx();
            if !session.painter.screen_can_hold(height, width) {
                return PromptEnd::Stop(MenuStop::TooSmall);
            }
            session.painter.paint_prompt(
                screen,
                &mut session.model,
                title,
                lines,
                APP_NAME,
                VERSION,
            );
            let key = screen.read_key();
            if key == KEY_INTERRUPT {
                return PromptEnd::Stop(MenuStop::Interrupt);
            }
            if key == 27 || key == -1 {
                session.model.buffer.clear();
                session.model.cursor = 0;
                session.model.focus = "list".to_string();
                session.model.error.clear();
                return PromptEnd::Cancel;
            }
            if key == KEY_ENTER || key == 10 || key == 13 {
                let text = session.model.buffer.trim().to_string();
                session.model.buffer.clear();
                session.model.cursor = 0;
                session.model.error.clear();
                session.model.focus = "list".to_string();
                return PromptEnd::Text(text);
            }
            session.model.edit_key(key);
            session.model.error.clear();
        }
    }

    fn verb_model(&self, args: Args, out: &mut dyn Write) -> i32 {
        let folder = args.target.unwrap_or_else(|| ".".to_string());
        let mut spoken = Vec::new();
        let mut notice = |line: &str| {
            let _ = writeln!(out, "{line}");
            let _ = out.flush();
            spoken.push(line.to_string());
        };
        let (code, lines) = convert_folder(
            &folder,
            args.output.as_deref(),
            args.views.as_deref(),
            args.grid,
            "model",
            Some(&mut notice),
        );
        let rest = &lines[spoken.len().min(lines.len())..];
        if !rest.is_empty() {
            let _ = writeln!(out, "{}", rest.join("\n"));
        }
        code
    }
}

enum MenuStop {
    Exit(i32),
    Interrupt,
    TooSmall,
}

enum MenuStep {
    Stay,
    Stop(MenuStop),
}

enum PromptEnd {
    Text(String),
    Cancel,
    Stop(MenuStop),
}

fn clock_board(model: &crate::menu::MenuModel) -> bool {
    model.phase == "board" && matches!(model.layer.as_str(), "front" | "self" | "log" | "lang")
}

struct Args {
    verb: Option<String>,
    target: Option<String>,
    views: Option<String>,
    grid: Option<i64>,
    output: Option<String>,
    force: bool,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut args = Args {
        verb: None,
        target: None,
        views: None,
        grid: None,
        output: None,
        force: false,
    };
    let mut index = 0;
    while index < argv.len() {
        let token = &argv[index];
        if token == "--force" {
            args.force = true;
        } else if token == "--views" {
            index += 1;
            args.views = Some(
                argv.get(index)
                    .cloned()
                    .ok_or_else(|| "missing value for --views".to_string())?,
            );
        } else if let Some(value) = token.strip_prefix("--views=") {
            args.views = Some(value.to_string());
        } else if token == "--grid" {
            index += 1;
            let raw = argv
                .get(index)
                .ok_or_else(|| "missing value for --grid".to_string())?;
            args.grid = Some(raw.parse::<i64>().unwrap_or(-1));
        } else if let Some(value) = token.strip_prefix("--grid=") {
            args.grid = Some(value.parse::<i64>().unwrap_or(-1));
        } else if token == "--output" {
            index += 1;
            args.output = Some(
                argv.get(index)
                    .cloned()
                    .ok_or_else(|| "missing value for --output".to_string())?,
            );
        } else if let Some(value) = token.strip_prefix("--output=") {
            args.output = Some(value.to_string());
        } else if token.starts_with('-') {
            return Err(format!("Unknown argument {token}."));
        } else if args.verb.is_none() {
            args.verb = Some(token.clone());
        } else if args.target.is_none() {
            args.target = Some(token.clone());
        } else {
            return Err("Too many arguments.".to_string());
        }
        index += 1;
    }
    Ok(args)
}

/// This function instantiates StatusLog. The statement below is the construct.
pub fn main_with(argv: Vec<String>, basedir: &str, logdir: &str) -> i32 {
    let screen = Cli::opens_text_screen(&argv);
    let logger = Rc::new(StatusLog::new(APP_NAME, screen, basedir, logdir));
    let appname = logger.log_name().to_string();
    let resolved_base = logger.base_dir().display().to_string();
    let _resolved_log = logger.log_dir().display().to_string();
    if logger.is_debug() {
        logger.log_message(
            &format!("{appname} v{VERSION} (src/cli.rs)"),
            "INFO",
            "main",
        );
        logger.log_message(
            &format!("Using {}", StatusLog::class_version()),
            "INFO",
            "main",
        );
        logger.log_message(&format!("Base {resolved_base}"), "DEBUG", "main");
        logger.log_message("debug mode", "INFO", "main");
    }
    let mut app = Cli::new(Some(logger));
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let stderr = io::stderr();
    let mut err = stderr.lock();
    app.run(&argv, &mut out, &mut err)
}

pub fn main() -> i32 {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    main_with(argv, "", "")
}
