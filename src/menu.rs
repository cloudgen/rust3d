//! Text menu. requirement-rust-tui and requirement-rust-cli-language.
//! Wide and Fullwidth count as two columns. Ambiguous stays one.

use crate::language::MenuLanguage;
use crate::model::list_subfolders;
use crate::status_log::StatusLog;
use chrono::Local;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{
    self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::Duration;
use unicode_width::UnicodeWidthChar;

pub const KEY_UP: i32 = 259;
pub const KEY_DOWN: i32 = 258;
pub const KEY_LEFT: i32 = 260;
pub const KEY_RIGHT: i32 = 261;
pub const KEY_HOME: i32 = 262;
pub const KEY_END: i32 = 360;
pub const KEY_BACKSPACE: i32 = 263;
pub const KEY_ENTER: i32 = 343;
pub const KEY_INTERRUPT: i32 = -2;

pub const ATTR_BOLD: i32 = 1;
pub const ATTR_REVERSE: i32 = 2;
pub const ATTR_ITALIC: i32 = 4;

pub const PATH_LABEL_EN: &str = "Path";
pub const PATH_LABEL_ZH_HANT: &str = "路徑";

const INPUT_MARK: &str = "> ";
const INPUT_HINT: &str = "Up/Down  •  Enter";
const FRAME_TOP_LEFT: &str = "╭";
const FRAME_TOP_RIGHT: &str = "╮";
const FRAME_BOTTOM_LEFT: &str = "╰";
const FRAME_BOTTOM_RIGHT: &str = "╯";
const FRAME_HORIZ: &str = "─";
const FRAME_VERT: &str = "│";
const CARET_BLOCK: &str = "█";
const FRAME_ROWS: i32 = 3;
const FOOTER_ROWS: i32 = 1;
const CHROME_ROWS: i32 = FRAME_ROWS + FOOTER_ROWS;
const MIN_MENU_ABOVE: i32 = 3;
pub const MIN_HEIGHT: i32 = MIN_MENU_ABOVE + CHROME_ROWS;

#[derive(Clone, Debug)]
pub struct MenuRow {
    pub number: i32,
    pub short: String,
    pub explain: String,
    pub kind: String,
}

const SELF_ROWS: &[(i32, &str, &str, &str)] = &[
    (62, "version", "show the installed version", "version"),
    (63, "about", "version and this computer", "about"),
    (
        64,
        "version-check",
        "compare this install with cargo",
        "version-check",
    ),
    (
        65,
        "self-update",
        "upgrade this package with cargo",
        "self-update",
    ),
    (
        66,
        "self-uninstall",
        "remove this package with cargo",
        "self-uninstall",
    ),
    (
        67,
        "self-install",
        "install this package with cargo",
        "self-install",
    ),
    (0, "Back", "return to the main menu", "back"),
];

const LOG_ROWS: &[(i32, &str, &str, &str)] = &[
    (31, "view-log", "list a log file and show it", "view-log"),
    (32, "clear-log", "empty one log file", "clear-log"),
    (33, "log-folder", "show the log folder", "log-folder"),
    (0, "Back", "return to the main menu", "back"),
];

const FRONT_ROWS: &[(i32, &str, &str, &str)] = &[
    (
        1,
        "model",
        "build a 3D model from outline images in a chosen folder",
        "model",
    ),
    (
        3,
        "system-log",
        "view, clear, and the log folder",
        "system-log",
    ),
    (
        6,
        "self-management",
        "version, about, and cargo lifecycle",
        "self-management",
    ),
    (7, "language", "display language for this menu", "language"),
    (9, "Exit", "leave", "exit"),
];

pub fn any_board(token: &str) -> Option<&'static str> {
    Some(match token {
        "help" => "help",
        "version" => "version",
        "about" => "about",
        "model" => "model",
        "self-install" => "self-install",
        "version-check" => "version-check",
        "self-update" => "self-update",
        "self-uninstall" => "self-uninstall",
        "self-management" => "self-management",
        "system-log" => "system-log",
        "view-log" => "view-log",
        "clear-log" => "clear-log",
        "log-folder" => "log-folder",
        _ => return None,
    })
}

pub trait TextScreen {
    fn getmaxyx(&self) -> (i32, i32);
    fn erase(&mut self);
    fn addstr(&mut self, y: i32, x: i32, text: &str, attr: i32);
    fn refresh(&mut self) {}
    fn curs_set(&mut self, _visibility: i32) {}
    fn move_cursor(&mut self, _y: i32, _x: i32) {}
}

pub struct MenuPainter {
    pub language: MenuLanguage,
    pub clock_text: Box<dyn Fn() -> String>,
}

impl MenuPainter {
    pub fn new(home: Option<&str>, logger: Option<&StatusLog>) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "MenuPainter");
        }
        Self {
            language: MenuLanguage::new(home, logger),
            clock_text: Box::new(|| Local::now().format("%H:%M:%S").to_string()),
        }
    }

    pub fn display_width(text: &str) -> usize {
        text.chars()
            .map(|ch| {
                if UnicodeWidthChar::width(ch) == Some(2) {
                    2
                } else {
                    1
                }
            })
            .sum()
    }

    pub fn clip_columns(text: &str, columns: usize) -> String {
        if columns == 0 {
            return String::new();
        }
        let mut kept = String::new();
        let mut used = 0usize;
        for ch in text.chars() {
            let width = if UnicodeWidthChar::width(ch) == Some(2) {
                2
            } else {
                1
            };
            if used + width > columns {
                break;
            }
            kept.push(ch);
            used += width;
        }
        kept
    }

    pub fn clip_columns_right(text: &str, columns: usize) -> String {
        if columns == 0 {
            return String::new();
        }
        let mut kept = Vec::new();
        let mut used = 0usize;
        for ch in text.chars().rev() {
            let width = if UnicodeWidthChar::width(ch) == Some(2) {
                2
            } else {
                1
            };
            if used + width > columns {
                break;
            }
            kept.push(ch);
            used += width;
        }
        kept.reverse();
        kept.into_iter().collect()
    }

    pub fn path_label(&self) -> String {
        self.language.path_label()
    }

    pub fn path_line(&self, placeable: Option<usize>) -> String {
        let prefix = format!("{}: ", self.path_label());
        let current = std::env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_default();
        let left = format!("{prefix}{current}");
        let right = (self.clock_text)();
        let gap = 2usize;
        let left_width = Self::display_width(&left);
        let right_width = Self::display_width(&right);
        let prefix_width = Self::display_width(&prefix);
        let Some(placeable) = placeable else {
            return format!("{left}  {right}");
        };
        if placeable >= left_width + gap + right_width {
            let spaces = placeable - left_width - right_width;
            return format!("{left}{}{right}", " ".repeat(spaces));
        }
        if placeable >= left_width {
            return left;
        }
        if placeable <= prefix_width {
            return Self::clip_columns(&prefix, placeable);
        }
        format!(
            "{prefix}{}",
            Self::clip_columns_right(&current, placeable - prefix_width)
        )
    }

    pub fn display_rows(&self, layer: &str) -> Vec<MenuRow> {
        if layer == "folders" {
            return self.folder_rows();
        }
        if layer == "lang" {
            let mut rows = Vec::new();
            for item in crate::lang_data::CODES {
                rows.push(MenuRow {
                    number: item.number,
                    short: item.endonym.to_string(),
                    explain: self.language.lang_long(item.endonym),
                    kind: format!("set-{}", item.code),
                });
            }
            rows.push(MenuRow {
                number: 0,
                short: self.language.row_short("back", "Back"),
                explain: self.language.row_long("back", "return to the main menu"),
                kind: "back".to_string(),
            });
            return rows;
        }
        let source = if layer == "self" {
            SELF_ROWS
        } else if layer == "log" {
            LOG_ROWS
        } else {
            FRONT_ROWS
        };
        source
            .iter()
            .map(|(number, short, explain, kind)| MenuRow {
                number: *number,
                short: self.language.row_short(kind, short),
                explain: self.language.row_long(kind, explain),
                kind: (*kind).to_string(),
            })
            .collect()
    }

    fn folder_rows(&self) -> Vec<MenuRow> {
        let mut rows = vec![MenuRow {
            number: 1,
            short: "current".to_string(),
            explain: self.language.text("current_long"),
            kind: "pick:.".to_string(),
        }];
        let mut number = 2;
        let cwd = std::env::current_dir().unwrap_or_default();
        for name in list_subfolders(&cwd) {
            rows.push(MenuRow {
                number,
                short: name.clone(),
                explain: self.language.text("subfolder_long"),
                kind: format!("pick:{name}"),
            });
            number += 1;
        }
        rows.push(MenuRow {
            number: 0,
            short: self.language.row_short("back", "Back"),
            explain: self.language.row_long("back", "return to the main menu"),
            kind: "back".to_string(),
        });
        rows
    }

    pub fn row_parts<'a>(&self, rows: &'a [MenuRow]) -> Vec<(String, &'a str, String, &'a str)> {
        let mut number_width = 1usize;
        let mut verb_width = 0usize;
        for row in rows {
            number_width = number_width.max(row.number.to_string().len());
            verb_width = verb_width.max(Self::display_width(&row.short));
        }
        rows.iter()
            .map(|row| {
                let text = row.number.to_string();
                let number_field = format!("{text:>number_width$}. ");
                let pad = verb_width.saturating_sub(Self::display_width(&row.short));
                (
                    number_field,
                    row.short.as_str(),
                    " ".repeat(pad),
                    row.explain.as_str(),
                )
            })
            .collect()
    }

    pub fn format_rows(&self, rows: &[MenuRow]) -> Vec<String> {
        self.row_parts(rows)
            .into_iter()
            .map(|(number_field, short, verb_pad, explain)| {
                format!("{number_field}{short}{verb_pad}: {explain}")
            })
            .collect()
    }

    pub fn board_title(&self, model: &MenuModel) -> String {
        if model.phase == "result" {
            return "result".to_string();
        }
        let key = match model.layer.as_str() {
            "self" => "title_self",
            "log" => "title_log",
            "lang" => "title_language",
            "folders" => "title_folders",
            _ => "title_main",
        };
        self.language.text(key)
    }

    fn banner(&self, model: &MenuModel) -> String {
        if !model.notice.is_empty() {
            return model.notice.clone();
        }
        model.error.clone()
    }

    fn put<S: TextScreen>(&self, screen: &mut S, y: i32, x: i32, text: &str, attr: i32) {
        let (height, width) = screen.getmaxyx();
        if y < 0 || y >= height || x >= width {
            return;
        }
        let room = (width - x).max(0) as usize;
        let clipped = Self::clip_columns(text, room);
        if !clipped.is_empty() {
            screen.addstr(y, x, &clipped, attr);
        }
    }

    fn result_overflow(&self, height: i32, line_count: usize) -> bool {
        line_count > (height - 3).max(1) as usize
    }

    fn result_room(&self, height: i32, line_count: usize) -> usize {
        if self.result_overflow(height, line_count) {
            (height - 4).max(1) as usize
        } else {
            (height - 3).max(1) as usize
        }
    }

    pub fn screen_can_hold(&self, height: i32, width: i32) -> bool {
        let min_placeable = 2 + 1 + INPUT_MARK.len() + 1;
        height >= MIN_HEIGHT && width as usize >= min_placeable
    }

    fn input_field(&self, model: &MenuModel, inner: usize) -> (String, Option<usize>) {
        let prefix = format!(" {INPUT_MARK}");
        let focused = model.focus == "input";
        let cursor = model.cursor.min(model.buffer.chars().count());
        let reserve = prefix.chars().count() + if focused { 1 } else { 0 };
        let text_room = inner.saturating_sub(reserve);
        let chars: Vec<char> = model.buffer.chars().collect();
        let mut start = 0usize;
        if text_room > 0 && cursor > text_room {
            start = cursor - text_room;
        }
        if text_room > 0 && start > chars.len().saturating_sub(text_room) {
            start = chars.len().saturating_sub(text_room);
        }
        let shown: String = if text_room == 0 {
            String::new()
        } else {
            chars.iter().skip(start).take(text_room).collect()
        };
        let rel = cursor.saturating_sub(start).min(shown.chars().count());
        let (body, caret_at) = if focused && inner >= reserve {
            let mut body = String::new();
            for (index, ch) in shown.chars().enumerate() {
                if index == rel {
                    body.push_str(CARET_BLOCK);
                }
                body.push(ch);
            }
            if rel == shown.chars().count() {
                body.push_str(CARET_BLOCK);
            }
            (body, Some(prefix.chars().count() + rel))
        } else {
            (shown, None)
        };
        let mut field: String = prefix + &body;
        let width = field.chars().count();
        if width < inner {
            field.push_str(&" ".repeat(inner - width));
        }
        let field: String = field.chars().take(inner).collect();
        let caret_at = caret_at.filter(|index| *index < inner);
        (field, caret_at)
    }

    fn status_line(&self, app_name: &str, version: &str, title: &str) -> String {
        format!("  {app_name} {version}  {FRAME_VERT}  {title}  {FRAME_VERT}  {INPUT_HINT}")
    }

    fn paint_box<S: TextScreen>(
        &self,
        screen: &mut S,
        model: &MenuModel,
        top: i32,
        app_name: &str,
        version: &str,
        title: &str,
    ) {
        let (_height, width) = screen.getmaxyx();
        let placeable = width.max(0) as usize;
        if top < 0 || placeable < 2 {
            return;
        }
        let inner = placeable - 2;
        let (field, caret_at) = self.input_field(model, inner);
        let lines = [
            format!(
                "{FRAME_TOP_LEFT}{}{FRAME_TOP_RIGHT}",
                FRAME_HORIZ.repeat(inner)
            ),
            format!("{FRAME_VERT}{field}{FRAME_VERT}"),
            format!(
                "{FRAME_BOTTOM_LEFT}{}{FRAME_BOTTOM_RIGHT}",
                FRAME_HORIZ.repeat(inner)
            ),
        ];
        for (offset, line) in lines.iter().enumerate() {
            self.put(screen, top + offset as i32, 0, line, 0);
        }
        self.put(
            screen,
            top + FRAME_ROWS,
            0,
            &self.status_line(app_name, version, title),
            0,
        );
        if let Some(caret_at) = caret_at {
            screen.move_cursor(top + 1, 1 + caret_at as i32);
        }
    }

    pub fn paint<S: TextScreen>(
        &self,
        screen: &mut S,
        model: &mut MenuModel,
        app_name: &str,
        version: &str,
    ) {
        screen.erase();
        let title = self.board_title(model);
        if model.phase == "working" {
            self.put(screen, 0, 0, "working", ATTR_BOLD);
            let (height, _) = screen.getmaxyx();
            let mut y = 2;
            for line in model.result_text.split('\n') {
                if y >= height {
                    break;
                }
                self.put(screen, y, 0, line, 0);
                y += 1;
            }
            screen.refresh();
            return;
        }
        if model.phase == "result" {
            self.put(screen, 0, 0, app_name, ATTR_BOLD);
            let mut cursor = Self::display_width(app_name) as i32;
            self.put(screen, 0, cursor, " (", 0);
            cursor += 2;
            self.put(screen, 0, cursor, version, ATTR_ITALIC);
            cursor += version.len() as i32;
            self.put(screen, 0, cursor, &format!(") — {title}"), 0);
            let lines: Vec<String> = if model.result_text.is_empty() {
                vec![String::new()]
            } else {
                model
                    .result_text
                    .split('\n')
                    .map(|item| item.to_string())
                    .collect()
            };
            let (height, _) = screen.getmaxyx();
            let room = self.result_room(height, lines.len());
            let max_offset = lines.len().saturating_sub(room);
            if model.result_offset > max_offset {
                model.result_offset = max_offset;
            }
            let window = &lines[model.result_offset..lines.len().min(model.result_offset + room)];
            let mut y = 2;
            for line in window {
                if y >= height - 1 {
                    break;
                }
                self.put(screen, y, 0, line, 0);
                y += 1;
            }
            if self.result_overflow(height, lines.len()) {
                self.put(
                    screen,
                    height - 2,
                    0,
                    "Up/Down scrolls this page.",
                    ATTR_ITALIC,
                );
                self.put(
                    screen,
                    height - 1,
                    0,
                    "Press a key to return to the main menu.",
                    ATTR_ITALIC,
                );
            } else {
                let mut hint_y = y + 1;
                if hint_y >= height {
                    hint_y = height - 1;
                }
                self.put(
                    screen,
                    hint_y,
                    0,
                    "Press a key to return to the main menu.",
                    ATTR_ITALIC,
                );
            }
            screen.refresh();
            return;
        }
        let (height, width) = screen.getmaxyx();
        self.put(
            screen,
            0,
            0,
            &self.path_line(Some(width.max(0) as usize)),
            0,
        );
        let box_top = height - CHROME_ROWS;
        let banner = self.banner(model);
        let mut stop = box_top;
        if !banner.is_empty() && box_top >= 4 {
            stop = box_top - 1;
        }
        let rows = model.rows(self);
        let parts = self.row_parts(&rows);
        let mut y = 2;
        if y < stop {
            for (idx, (number_field, short, verb_pad, explain)) in parts.iter().enumerate() {
                if y >= stop {
                    break;
                }
                let selected = idx == model.index;
                let base = if selected && model.focus == "list" {
                    ATTR_REVERSE
                } else if selected {
                    ATTR_BOLD
                } else {
                    0
                };
                self.put(screen, y, 0, number_field, base);
                let verb_x = Self::display_width(number_field) as i32;
                self.put(screen, y, verb_x, short, base | ATTR_BOLD);
                let colon_x = verb_x + Self::display_width(short) as i32;
                let colon = format!("{verb_pad}: ");
                self.put(screen, y, colon_x, &colon, base);
                let explain_x = colon_x + Self::display_width(verb_pad) as i32 + 2;
                self.put(screen, y, explain_x, explain, base | ATTR_ITALIC);
                y += 1;
            }
        }
        if !banner.is_empty() && box_top >= 4 {
            self.put(screen, box_top - 1, 0, &banner, 1);
        }
        self.paint_box(screen, model, box_top, app_name, version, &title);
        screen.curs_set(0);
        screen.refresh();
    }

    /// Question or file list. The clock stays off. The box stays on the bottom.
    pub fn paint_prompt<S: TextScreen>(
        &self,
        screen: &mut S,
        model: &mut MenuModel,
        title: &str,
        lines: &[String],
        app_name: &str,
        version: &str,
    ) {
        screen.erase();
        self.put(screen, 0, 0, app_name, ATTR_BOLD);
        let mut cursor = Self::display_width(app_name) as i32;
        self.put(screen, 0, cursor, " (", 0);
        cursor += 2;
        self.put(screen, 0, cursor, version, ATTR_ITALIC);
        cursor += version.len() as i32;
        self.put(screen, 0, cursor, &format!(") — {title}"), 0);
        let (height, _) = screen.getmaxyx();
        let box_top = height - CHROME_ROWS;
        let banner = self.banner(model);
        let mut stop = box_top;
        if !banner.is_empty() && box_top >= 4 {
            stop = box_top - 1;
        }
        let mut y = 2;
        for line in lines {
            if y >= stop {
                break;
            }
            self.put(screen, y, 0, line, 0);
            y += 1;
        }
        if !banner.is_empty() && box_top >= 4 {
            self.put(screen, box_top - 1, 0, &banner, ATTR_BOLD);
        }
        self.paint_box(screen, model, box_top, app_name, version, title);
        screen.curs_set(0);
        screen.refresh();
    }
}

pub struct MenuModel {
    pub layer: String,
    pub index: usize,
    pub buffer: String,
    pub cursor: usize,
    pub focus: String,
    pub error: String,
    pub notice: String,
    pub phase: String,
    pub result_text: String,
    pub result_offset: usize,
}

impl MenuModel {
    pub fn new(logger: Option<&StatusLog>) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "MenuModel");
        }
        Self {
            layer: "front".to_string(),
            index: 0,
            buffer: String::new(),
            cursor: 0,
            focus: "list".to_string(),
            error: String::new(),
            notice: String::new(),
            phase: "board".to_string(),
            result_text: String::new(),
            result_offset: 0,
        }
    }

    pub fn rows(&self, painter: &MenuPainter) -> Vec<MenuRow> {
        painter.display_rows(&self.layer)
    }

    pub fn show_result(&mut self, text: &str) {
        self.result_text = text.to_string();
        self.result_offset = 0;
        self.phase = "result".to_string();
        self.layer = "front".to_string();
        self.index = 0;
        self.buffer.clear();
        self.cursor = 0;
        self.focus = "list".to_string();
        self.error.clear();
    }

    pub fn show_front(&mut self) {
        self.phase = "board".to_string();
        self.layer = "front".to_string();
        self.index = 0;
        self.buffer.clear();
        self.cursor = 0;
        self.focus = "list".to_string();
        self.error.clear();
        self.result_offset = 0;
    }

    pub fn apply_key(
        &mut self,
        painter: &MenuPainter,
        key: i32,
        screen_height: i32,
    ) -> Option<String> {
        self.notice.clear();
        if key == 27 {
            if self.phase == "result" || self.layer != "front" {
                self.show_front();
            }
            return None;
        }
        if key == -1 {
            if self.phase == "result" || self.layer != "front" {
                self.show_front();
                return None;
            }
            return Some("exit".to_string());
        }
        if self.phase == "result" {
            let lines: Vec<&str> = if self.result_text.is_empty() {
                vec![""]
            } else {
                self.result_text.split('\n').collect()
            };
            let overflow = lines.len() > (screen_height - 3).max(1) as usize;
            if overflow && (key == KEY_UP || key == b'k' as i32) {
                self.scroll_result(painter, -1, screen_height);
                return None;
            }
            if overflow && (key == KEY_DOWN || key == b'j' as i32) {
                self.scroll_result(painter, 1, screen_height);
                return None;
            }
            self.show_front();
            return None;
        }
        if key == KEY_UP
            || key == KEY_DOWN
            || (self.focus == "list" && (key == b'k' as i32 || key == b'j' as i32))
        {
            self.arrow(painter, key);
            return None;
        }
        if key == KEY_LEFT || key == KEY_RIGHT || key == KEY_HOME || key == KEY_END {
            self.slide(key);
            return None;
        }
        if key == KEY_BACKSPACE || key == 127 || key == 8 {
            self.backspace();
            return None;
        }
        if key == 10 || key == 13 || key == KEY_ENTER {
            return self.commit(painter);
        }
        if let Some(ch) = char::from_u32(key as u32) {
            if !ch.is_control() {
                self.insert(ch);
                return None;
            }
        }
        self.error = painter.language.text("invalid");
        None
    }

    fn scroll_result(&mut self, painter: &MenuPainter, delta: i32, screen_height: i32) {
        let count = if self.result_text.is_empty() {
            1
        } else {
            self.result_text.split('\n').count()
        };
        let room = painter.result_room(screen_height, count);
        let max_offset = count.saturating_sub(room);
        let next = self.result_offset as i32 + delta;
        self.result_offset = next.clamp(0, max_offset as i32) as usize;
    }

    fn arrow(&mut self, painter: &MenuPainter, key: i32) {
        let last = self.rows(painter).len().saturating_sub(1);
        let down = key == KEY_DOWN || key == b'j' as i32;
        if self.focus == "input" {
            self.focus = "list".to_string();
            self.index = if down { 0 } else { last };
            return;
        }
        if down {
            if self.index < last {
                self.index += 1;
            } else {
                self.focus = "input".to_string();
                self.cursor = self.buffer.chars().count();
            }
            return;
        }
        if self.index > 0 {
            self.index -= 1;
            return;
        }
        self.focus = "input".to_string();
        self.cursor = self.buffer.chars().count();
    }

    fn slide(&mut self, key: i32) {
        if self.focus != "input" {
            return;
        }
        let len = self.buffer.chars().count();
        if key == KEY_LEFT {
            self.cursor = self.cursor.saturating_sub(1);
        } else if key == KEY_RIGHT {
            self.cursor = (self.cursor + 1).min(len);
        } else if key == KEY_HOME {
            self.cursor = 0;
        } else if key == KEY_END {
            self.cursor = len;
        }
    }

    fn insert(&mut self, ch: char) {
        if self.focus != "input" {
            self.focus = "input".to_string();
            self.cursor = self.buffer.chars().count();
        }
        let mut chars: Vec<char> = self.buffer.chars().collect();
        let at = self.cursor.min(chars.len());
        chars.insert(at, ch);
        self.buffer = chars.into_iter().collect();
        self.cursor = at + 1;
    }

    fn backspace(&mut self) {
        if self.focus != "input" {
            self.focus = "input".to_string();
            self.cursor = self.buffer.chars().count();
        }
        if self.cursor == 0 || self.buffer.is_empty() {
            self.cursor = 0;
            return;
        }
        let mut chars: Vec<char> = self.buffer.chars().collect();
        self.cursor = self.cursor.min(chars.len());
        chars.remove(self.cursor - 1);
        self.cursor -= 1;
        self.buffer = chars.into_iter().collect();
    }

    fn commit(&mut self, painter: &MenuPainter) -> Option<String> {
        let token = self.buffer.trim().to_string();
        self.buffer.clear();
        self.cursor = 0;
        let rows = self.rows(painter);
        if token.is_empty() {
            if rows.is_empty() {
                return None;
            }
            let row = &rows[self.index.min(rows.len() - 1)];
            return self.activate(row.number, &row.kind);
        }
        if token.chars().all(|ch| ch.is_ascii_digit()) {
            if let Ok(number) = token.parse::<i32>() {
                return self.activate_number(painter, number);
            }
        }
        for row in &rows {
            if token == row.short {
                return self.activate(row.number, &row.kind);
            }
        }
        if let Some(mapped) = painter.language.token_kind(&token, &self.layer) {
            return self.activate(0, &mapped);
        }
        if let Some(kind) = any_board(&token) {
            return self.activate(0, kind);
        }
        self.error = painter.language.text("invalid");
        self.focus = "input".to_string();
        self.cursor = self.buffer.chars().count();
        None
    }

    fn activate_number(&mut self, painter: &MenuPainter, number: i32) -> Option<String> {
        for row in self.rows(painter) {
            if row.number == number {
                return self.activate(number, &row.kind);
            }
        }
        self.error = painter.language.text("invalid");
        self.focus = "input".to_string();
        None
    }

    pub fn activate(&mut self, _number: i32, kind: &str) -> Option<String> {
        self.error.clear();
        self.focus = "list".to_string();
        self.cursor = 0;
        if kind == "exit" {
            return Some("exit".to_string());
        }
        if kind == "back" {
            self.layer = "front".to_string();
            self.index = 0;
            return None;
        }
        if kind == "self-management" {
            self.open_layer("self");
            return None;
        }
        if kind == "system-log" {
            self.open_layer("log");
            return None;
        }
        if kind == "language" {
            self.open_layer("lang");
            return None;
        }
        if kind == "model" {
            self.open_layer("folders");
            return None;
        }
        if kind.starts_with("set-") || kind == "version" || kind == "about" {
            return Some(kind.to_string());
        }
        Some(kind.to_string())
    }

    /// Edit the bottom box without running a row. Questions use this path.
    pub fn edit_key(&mut self, key: i32) {
        if key == KEY_LEFT || key == KEY_RIGHT || key == KEY_HOME || key == KEY_END {
            self.focus = "input".to_string();
            self.slide(key);
            return;
        }
        if key == KEY_BACKSPACE || key == 127 || key == 8 {
            self.backspace();
            return;
        }
        if let Some(ch) = char::from_u32(key as u32) {
            if !ch.is_control() {
                self.insert(ch);
            }
        }
    }

    fn open_layer(&mut self, layer: &str) {
        self.layer = layer.to_string();
        self.index = 0;
        self.focus = "list".to_string();
        self.buffer.clear();
        self.cursor = 0;
    }
}

pub struct MenuSession {
    pub app_name: String,
    pub version: String,
    pub painter: MenuPainter,
    pub model: MenuModel,
    pub on_version: Box<dyn Fn() -> String>,
    pub on_about: Box<dyn Fn() -> String>,
    pub on_kind: Option<Rc<dyn Fn(&str) -> Option<String>>>,
}

impl MenuSession {
    pub fn new(
        app_name: &str,
        version: &str,
        painter: MenuPainter,
        on_version: Box<dyn Fn() -> String>,
        on_about: Box<dyn Fn() -> String>,
        logger: Option<&StatusLog>,
    ) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "MenuSession");
        }
        Self {
            app_name: app_name.to_string(),
            version: version.to_string(),
            model: MenuModel::new(logger),
            painter,
            on_version,
            on_about,
            on_kind: None,
        }
    }

    pub fn take_action(&mut self, action: Option<String>) -> &'static str {
        let Some(action) = action else {
            return "stay";
        };
        if action == "exit" {
            return "exit";
        }
        if let Some(code) = action.strip_prefix("set-") {
            let message = self.painter.language.save(code);
            self.model.show_front();
            self.model.notice = message;
            return "stay";
        }
        if action == "version" {
            let text = (self.on_version)();
            self.model.show_result(&text);
            return "stay";
        }
        if action == "about" {
            let text = (self.on_about)();
            self.model.show_result(&text);
            return "stay";
        }
        if let Some(handler) = &self.on_kind {
            if let Some(outcome) = handler(&action) {
                if outcome == "leave" {
                    return "exit";
                }
                if !outcome.is_empty() {
                    self.model.show_result(&outcome);
                }
            }
        }
        "stay"
    }
}

pub struct Tui {
    pub painter: MenuPainter,
}

impl Tui {
    pub fn new(home: Option<&str>, logger: Option<&StatusLog>) -> Self {
        if let Some(logger) = logger {
            logger.log_message("instantiated", "INFO", "Tui");
        }
        Self {
            painter: MenuPainter::new(home, logger),
        }
    }

    pub fn front_lines(&self) -> Vec<String> {
        self.board_lines("front")
    }

    pub fn self_lines(&self) -> Vec<String> {
        self.board_lines("self")
    }

    pub fn log_lines(&self) -> Vec<String> {
        self.board_lines("log")
    }

    pub fn language_lines(&self) -> Vec<String> {
        self.board_lines("lang")
    }

    fn board_lines(&self, layer: &str) -> Vec<String> {
        let mut lines = vec![self.painter.path_line(None)];
        lines.extend(self.painter.format_rows(&self.painter.display_rows(layer)));
        lines
    }
}

/// Live terminal for the text menu. The frame glyphs stay on `MenuPainter`.
/// requirement-rust-tui. The writer is crossterm, the same terminal session HolaTui uses.
pub struct TermScreen {
    stdout: io::Stdout,
    active: bool,
}

impl TermScreen {
    pub fn enter() -> io::Result<Self> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;
        let _ = stdout.execute(terminal::DisableLineWrap);
        Ok(Self {
            stdout,
            active: true,
        })
    }

    pub fn leave(&mut self) {
        if !self.active {
            return;
        }
        self.active = false;
        let _ = self.stdout.execute(terminal::EnableLineWrap);
        let _ = self.stdout.execute(Show);
        let _ = self.stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        let _ = self.stdout.flush();
    }

    pub fn read_key(&mut self) -> i32 {
        loop {
            match event::read() {
                Ok(event) => {
                    if let Some(key) = map_event(event) {
                        return key;
                    }
                }
                Err(_) => return -1,
            }
        }
    }

    /// `None` means the wait ended with no key. The clock redraws. It is not Esc.
    pub fn poll_key(&mut self, timeout_ms: u64) -> Option<i32> {
        match event::poll(Duration::from_millis(timeout_ms)) {
            Ok(true) => match event::read() {
                Ok(event) => map_event(event),
                Err(_) => Some(-1),
            },
            Ok(false) => None,
            Err(_) => Some(-1),
        }
    }
}

impl Drop for TermScreen {
    fn drop(&mut self) {
        self.leave();
    }
}

impl TextScreen for TermScreen {
    fn getmaxyx(&self) -> (i32, i32) {
        terminal::size()
            .map(|(width, height)| (i32::from(height), i32::from(width)))
            .unwrap_or((24, 80))
    }

    fn erase(&mut self) {
        let _ = self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All));
        let _ = self.stdout.execute(MoveTo(0, 0));
    }

    fn addstr(&mut self, y: i32, x: i32, text: &str, attr: i32) {
        if y < 0 || x < 0 {
            return;
        }
        let _ = self.stdout.execute(MoveTo(x as u16, y as u16));
        apply_attr(&mut self.stdout, attr);
        let _ = self.stdout.execute(Print(text));
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
    }

    fn refresh(&mut self) {
        let _ = self.stdout.flush();
    }

    fn curs_set(&mut self, visibility: i32) {
        if visibility == 0 {
            let _ = self.stdout.execute(Hide);
        } else {
            let _ = self.stdout.execute(Show);
        }
    }

    fn move_cursor(&mut self, y: i32, x: i32) {
        if y < 0 || x < 0 {
            return;
        }
        let _ = self.stdout.execute(MoveTo(x as u16, y as u16));
    }
}

fn apply_attr(stdout: &mut io::Stdout, attr: i32) {
    let _ = stdout.execute(SetAttribute(Attribute::Reset));
    if attr & ATTR_BOLD != 0 {
        let _ = stdout.execute(SetAttribute(Attribute::Bold));
    }
    if attr & ATTR_REVERSE != 0 {
        let _ = stdout.execute(SetAttribute(Attribute::Reverse));
    }
    if attr & ATTR_ITALIC != 0 {
        let _ = stdout.execute(SetAttribute(Attribute::Italic));
    }
}

fn map_event(event: Event) -> Option<i32> {
    match event {
        Event::Key(key) => map_key(key.code, key.modifiers),
        _ => None,
    }
}

fn map_key(code: KeyCode, modifiers: KeyModifiers) -> Option<i32> {
    if modifiers.contains(KeyModifiers::CONTROL)
        && matches!(code, KeyCode::Char('c') | KeyCode::Char('C'))
    {
        return Some(KEY_INTERRUPT);
    }
    Some(match code {
        KeyCode::Up => KEY_UP,
        KeyCode::Down => KEY_DOWN,
        KeyCode::Left => KEY_LEFT,
        KeyCode::Right => KEY_RIGHT,
        KeyCode::Home => KEY_HOME,
        KeyCode::End => KEY_END,
        KeyCode::Enter => KEY_ENTER,
        KeyCode::Backspace => KEY_BACKSPACE,
        KeyCode::Esc => 27,
        KeyCode::Char(ch) => ch as i32,
        _ => return None,
    })
}
