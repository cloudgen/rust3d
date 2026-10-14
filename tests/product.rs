//! Product proof for rust3d. Tests that touch the process environment or the
//! working directory take rust3d::PROCESS_LOCK.

use rust3d::about::AboutPage;
use rust3d::cli::{self, Cli};
use rust3d::language::MenuLanguage;
use rust3d::menu::{
    MenuModel, MenuPainter, MenuSession, TextScreen, Tui, PATH_LABEL_EN, PATH_LABEL_ZH_HANT,
};
use rust3d::model::{self, convert_folder, convert_main, list_images, selected_choice_line};
use rust3d::self_manage::SelfManage;
use rust3d::status_log::StatusLog;
use rust3d::system_log::SystemLog;
use rust3d::{APP_NAME, VERSION};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

struct ProcessGuard {
    env: Vec<(String, Option<String>)>,
    dir: PathBuf,
}

impl ProcessGuard {
    fn new(keys: &[&str]) -> Self {
        let env = keys
            .iter()
            .map(|key| (key.to_string(), std::env::var(key).ok()))
            .collect();
        Self {
            env,
            dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
}

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.dir);
        for (key, value) in self.env.drain(..) {
            match value {
                Some(text) => unsafe { std::env::set_var(key, text) },
                None => unsafe { std::env::remove_var(key) },
            }
        }
        cli::force_tty(None);
        model::reset_decode_watch();
    }
}

fn lock() -> std::sync::MutexGuard<'static, ()> {
    rust3d::PROCESS_LOCK
        .lock()
        .unwrap_or_else(|err| err.into_inner())
}

fn temp_dir(prefix: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "{prefix}{}{}",
        std::process::id(),
        AtomicSeq::next()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

struct AtomicSeq;
static SEQ: AtomicUsize = AtomicUsize::new(0);
impl AtomicSeq {
    fn next() -> usize {
        SEQ.fetch_add(1, Ordering::SeqCst)
    }
}

fn set_env(key: &str, value: &str) {
    unsafe { std::env::set_var(key, value) }
}

fn unset_env(key: &str) {
    unsafe { std::env::remove_var(key) }
}

fn run(app: &mut Cli, argv: &[&str]) -> (i32, String, String) {
    let mut out = Vec::new();
    let mut err = Vec::new();
    let owned: Vec<String> = argv.iter().map(|item| item.to_string()).collect();
    let code = app.run(&owned, &mut out, &mut err);
    (
        code,
        String::from_utf8_lossy(&out).to_string(),
        String::from_utf8_lossy(&err).to_string(),
    )
}

fn commit(model: &mut MenuModel, painter: &MenuPainter, token: &str) -> Option<String> {
    model.buffer = token.to_string();
    model.cursor = token.chars().count();
    model.focus = "input".to_string();
    model.apply_key(painter, 10, 24)
}

static DECODE_WATCH: AtomicUsize = AtomicUsize::new(0);

fn watch_decode() {
    DECODE_WATCH.fetch_add(1, Ordering::SeqCst);
}

fn write_disk(path: &Path, rx: f64, ry: f64) {
    let mut image = image::GrayImage::new(80, 80);
    for y in 0..80 {
        for x in 0..80 {
            let dx = (x as f64 - 39.5) / rx;
            let dy = (y as f64 - 39.5) / ry;
            if dx * dx + dy * dy <= 1.0 {
                image.put_pixel(x, y, image::Luma([255]));
            }
        }
    }
    image.save(path).unwrap();
}

struct ColumnScreen {
    size: (i32, i32),
    calls: Vec<(i32, i32, String)>,
}

impl ColumnScreen {
    fn new(size: (i32, i32)) -> Self {
        Self {
            size,
            calls: Vec::new(),
        }
    }
}

impl TextScreen for ColumnScreen {
    fn getmaxyx(&self) -> (i32, i32) {
        self.size
    }
    fn erase(&mut self) {
        self.calls.clear();
    }
    fn addstr(&mut self, y: i32, x: i32, text: &str, _attr: i32) {
        self.calls.push((y, x, text.to_string()));
    }
}

fn char_width(ch: char) -> usize {
    let text = ch.to_string();
    MenuPainter::display_width(&text)
}

fn occupy(calls: &[(i32, i32, String)]) {
    let mut rows: std::collections::BTreeMap<i32, std::collections::BTreeMap<i32, char>> =
        std::collections::BTreeMap::new();
    for (y, x, text) in calls {
        let row = rows.entry(*y).or_default();
        let mut col = *x;
        for ch in text.chars() {
            let width = char_width(ch) as i32;
            for offset in 0..width {
                let pos = col + offset;
                assert!(
                    !row.contains_key(&pos),
                    "row {y} column {pos} already holds {:?}",
                    row.get(&pos)
                );
                row.insert(pos, if offset == 0 { ch } else { '\0' });
            }
            col += width;
        }
    }
}

fn gap_before(line: &str, clock: &str) -> usize {
    let head = &line[..line.len() - clock.len()];
    head.len() - head.trim_end().len()
}

#[test]
fn model_cli_menu_and_glb() {
    let _lock = lock();
    let _guard = ProcessGuard::new(&["RUST3D_LANG", "HOME", "LANG", "LC_ALL"]);
    unset_env("RUST3D_LANG");

    assert!(cli::PRODUCT_VERBS.contains(&"model"));
    for gone in ["outline", "hello", "join", "list-videos"] {
        assert!(!cli::PRODUCT_VERBS.contains(&gone), "{gone}");
    }
    cli::force_tty(Some(true));
    assert!(Cli::opens_text_screen(&[]));
    assert!(!Cli::opens_text_screen(&["model".into()]));
    assert!(!Cli::opens_text_screen(&["model".into(), "photos".into()]));
    assert!(Cli::opens_text_screen(&["about".into()]));
    assert!(!Cli::opens_text_screen(&["join".into()]));
    cli::force_tty(Some(false));
    assert!(!Cli::opens_text_screen(&[]));

    let empty = temp_dir("rust3d-empty-");
    let previous = std::env::current_dir().unwrap();
    std::env::set_current_dir(&empty).unwrap();
    let mut app = Cli::new(None);
    let (code, out, _) = run(&mut app, &["model"]);
    std::env::set_current_dir(&previous).unwrap();
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("No supported images found in"), "{out}");
    assert!(
        out.contains(&empty.canonicalize().unwrap().display().to_string()),
        "{out}"
    );
    assert!(!out.contains("has been selected"), "{out}");

    let (code, out, err) = run(
        &mut app,
        &["model", "/tmp/rust3d-missing-folder-does-not-exist"],
    );
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("is not a folder"), "{out}");
    assert!(!out.contains("has been selected"), "{out}");

    let (code, _, err) = run(&mut app, &["version", "photos"]);
    assert_eq!(code, 1);
    assert!(err.contains("A folder is only for model"), "{err}");

    for verb in ["join", "list-videos", "hello", "outline"] {
        let (code, _, err) = run(&mut app, &[verb]);
        assert_eq!(code, 1, "{verb} {err}");
        assert!(err.contains(&format!("Unknown verb '{verb}'")), "{err}");
    }
    let (code, help, _) = run(&mut app, &["help"]);
    assert_eq!(code, 0);
    assert!(help.contains("model"));
    assert!(help.contains("model.glb"));
    assert!(help.contains("cargo search rust3d"));
    for banned in ["list-videos", "FFmpeg", "ffmpeg", "concatenate"] {
        assert!(!help.contains(banned), "{banned} in {help}");
    }

    cli::force_tty(Some(false));
    let (code, out, _) = run(&mut Cli::new(None), &[]);
    assert_eq!(code, 0);
    assert!(out.contains("model"));
    assert!(!out.contains("has been selected"));

    cli::force_tty(Some(true));
    let menu_home = temp_dir("rust3d-menu-");
    set_env("HOME", menu_home.to_str().unwrap());
    let painter = MenuPainter::new(Some(menu_home.to_str().unwrap()), None);
    let mut model = MenuModel::new(None);
    let mut screen = ColumnScreen::new((24, 80));
    painter.paint(&mut screen, &mut model, APP_NAME, VERSION);
    let menu_text = screen
        .calls
        .iter()
        .map(|item| item.2.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(menu_text.contains("model"), "{menu_text}");
    assert!(menu_text.contains('╭'), "{menu_text}");
    assert!(
        menu_text.contains("Exit") || menu_text.contains("leave"),
        "{menu_text}"
    );
    assert!(!menu_text.contains("has been selected"), "{menu_text}");
    assert!(!menu_text.contains("Choice:"), "{menu_text}");
    assert_eq!(commit(&mut model, &painter, "9").as_deref(), Some("exit"));
    cli::force_tty(None);

    assert_eq!(
        selected_choice_line("model", "Building the 3D model"),
        "model has been selected. Building the 3D model takes time to finish."
    );

    model::reset_decode_watch();
    DECODE_WATCH.store(0, Ordering::SeqCst);
    model::set_decode_hook(watch_decode);
    let quiet = temp_dir("rust3d-quiet-");
    let (code, lines) = convert_folder(
        quiet.to_str().unwrap(),
        None,
        None,
        None,
        "model",
        None::<&mut fn(&str)>,
    );
    assert_eq!(code, 0);
    assert!(lines.join("\n").contains("No supported images found in"));
    assert_eq!(DECODE_WATCH.load(Ordering::SeqCst), 0);
    assert_eq!(model::decode_hits(), 0);

    let listed = temp_dir("rust3d-list-");
    fs::write(listed.join("a.png"), b"").unwrap();
    fs::create_dir(listed.join("nested")).unwrap();
    fs::write(listed.join("nested").join("b.png"), b"").unwrap();
    fs::write(listed.join("note.txt"), "x").unwrap();
    let names: Vec<_> = list_images(&listed)
        .iter()
        .map(|item| item.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert_eq!(names, vec!["a.png".to_string()]);

    let conv = temp_dir("rust3d-conv-");
    let code = convert_main(&[
        "--input-dir".into(),
        conv.display().to_string(),
        "--grid".into(),
        "24".into(),
    ]);
    assert_eq!(code, 0);

    let built = temp_dir("rust3d-build-");
    write_disk(&built.join("front.png"), 24.0, 18.0);
    write_disk(&built.join("side.png"), 12.0, 18.0);
    let views = built.join("views.json");
    fs::write(
        &views,
        r#"{"views":[{"file":"front.png","azimuth":0,"elevation":0},{"file":"side.png","azimuth":90,"elevation":0}]}"#,
    )
    .unwrap();
    model::reset_decode_watch();
    DECODE_WATCH.store(0, Ordering::SeqCst);
    model::set_decode_hook(watch_decode);
    let (code, lines) = convert_folder(
        built.to_str().unwrap(),
        None,
        Some(views.to_str().unwrap()),
        Some(24),
        "model",
        None::<&mut fn(&str)>,
    );
    let text = lines.join("\n");
    assert_eq!(code, 0, "{text}");
    assert!(
        text.starts_with("model has been selected. Building the 3D model takes time to finish.")
    );
    assert!(DECODE_WATCH.load(Ordering::SeqCst) >= 1);
    let glb = fs::read(built.join("model").join("model.glb")).unwrap();
    assert!(glb.starts_with(b"glTF"));
    let page = fs::read_to_string(built.join("model").join("viewer.html")).unwrap();
    assert!(page.contains("<canvas"));
    assert!(!page.contains("script src="));
    assert!(text.contains("Model saved in:"));
    assert!(text.contains("Saved model -> model.glb"));
    assert!(text.contains("Saved viewer -> viewer.html"));

    let cli_folder = temp_dir("rust3d-cli-");
    write_disk(&cli_folder.join("front.png"), 24.0, 18.0);
    std::env::set_current_dir(&cli_folder).unwrap();
    model::reset_decode_watch();
    DECODE_WATCH.store(0, Ordering::SeqCst);
    model::set_decode_hook(watch_decode);
    let (code, out, err) = run(&mut Cli::new(None), &["model", "--grid", "24"]);
    std::env::set_current_dir(&previous).unwrap();
    assert_eq!(code, 0, "{out}{err}");
    let sentence = "model has been selected. Building the 3D model takes time to finish.";
    assert!(out.contains(sentence), "{out}");
    assert!(DECODE_WATCH.load(Ordering::SeqCst) >= 1);
    assert!(out.find(sentence).unwrap() < out.find("Saved model").unwrap());

    let (code, lines) = convert_folder(
        built.to_str().unwrap(),
        None,
        None,
        Some(4),
        "model",
        None::<&mut fn(&str)>,
    );
    assert_eq!(code, 1);
    assert!(lines.join("\n").contains("grid must be an integer"));
}

#[test]
fn menu_language_and_columns() {
    let _lock = lock();
    let _guard = ProcessGuard::new(&[
        "HOME",
        "RUST3D_LANG",
        "LANG",
        "LANGUAGE",
        "LC_ALL",
        "LC_MESSAGES",
    ]);
    let home = temp_dir("rust3d-home-");
    let folder = temp_dir("rust3d-clips-");
    fs::create_dir(folder.join("beta")).unwrap();
    fs::create_dir(folder.join("alpha")).unwrap();
    fs::create_dir(folder.join(".hidden")).unwrap();
    set_env("HOME", home.to_str().unwrap());
    unset_env("RUST3D_LANG");
    set_env("LANG", "zh_TW.UTF-8");
    set_env("LANGUAGE", "zh_TW:zh");
    set_env("LC_ALL", "zh_TW.UTF-8");
    set_env("LC_MESSAGES", "zh_TW.UTF-8");
    std::env::set_current_dir(&folder).unwrap();

    assert_eq!(PATH_LABEL_EN, "Path");
    assert_eq!(PATH_LABEL_ZH_HANT, "路徑");
    let mut painter = MenuPainter::new(Some(home.to_str().unwrap()), None);
    assert_eq!(painter.language.code(), "en");
    assert_eq!(painter.path_label(), "Path");
    let front = painter.display_rows("front");
    let numbers: Vec<_> = front.iter().map(|row| row.number).collect();
    assert_eq!(numbers, vec![1, 4, 6, 7, 9]);
    assert_eq!(front[0].short, "model");
    assert_eq!(front[0].kind, "model");
    assert_eq!(front[1].kind, "system-log");
    assert_eq!(front[2].short, "self-management");
    assert_eq!(front[2].kind, "self-management");
    assert_eq!(front[3].short, "language");
    assert_eq!(front[3].kind, "language");
    assert!(!numbers.contains(&2));
    let log_rows = painter.display_rows("log");
    assert_eq!(
        log_rows.iter().map(|row| row.number).collect::<Vec<_>>(),
        vec![41, 42, 43, 0]
    );
    assert_eq!(
        log_rows
            .iter()
            .take(3)
            .map(|row| row.short.as_str())
            .collect::<Vec<_>>(),
        vec!["view-log", "clear-log", "log-folder"]
    );
    let self_rows = painter.display_rows("self");
    assert_eq!(
        self_rows.iter().map(|row| row.number).collect::<Vec<_>>(),
        vec![62, 63, 64, 65, 66, 67, 0]
    );
    let lang_rows = painter.display_rows("lang");
    let printed: Vec<_> = lang_rows.iter().map(|row| row.number).collect();
    assert_eq!(&printed[..13], &(71..84).collect::<Vec<_>>());
    assert_eq!(*printed.last().unwrap(), 0);
    assert!(!printed.contains(&70));
    assert!(!printed.contains(&84));
    assert!(!printed.contains(&89));
    assert_eq!(lang_rows[0].short, "English");
    assert_eq!(lang_rows[2].short, "繁體中文");
    assert_eq!(lang_rows[lang_rows.len() - 2].short, "Ελληνικά");

    let mut model = MenuModel::new(None);
    assert!(commit(&mut model, &painter, "1").is_none());
    assert_eq!(model.layer, "folders");
    let picked = painter.display_rows("folders");
    assert_eq!(picked[0].number, 1);
    assert_eq!(picked[0].short, "current");
    assert_eq!(picked[0].kind, "pick:.");
    assert_eq!(picked[1].short, "alpha");
    assert_eq!(picked[1].kind, "pick:alpha");
    assert_eq!(picked[2].short, "beta");
    assert!(picked.iter().all(|row| row.short != ".hidden"));
    model.layer = "front".into();

    let clock = "14:05:09".to_string();
    let clock_text = clock.clone();
    painter.clock_text = Box::new(move || clock_text.clone());
    let current = folder.canonicalize().unwrap().display().to_string();
    let logical = format!("Path: {current}  {clock}");
    assert_eq!(painter.path_line(None), logical);
    assert!(!logical.contains("路徑"));
    let narrow = MenuPainter::display_width("Path: ") + 4;
    let shortened = painter.path_line(Some(narrow));
    assert_eq!(
        shortened,
        format!("Path: {}", &current[current.len() - 4..])
    );
    assert!(!shortened.contains(&clock));

    let mut session = Tui::new(Some(home.to_str().unwrap()), None);
    let clock_tui = clock.clone();
    session.painter.clock_text = Box::new(move || clock_tui.clone());
    assert_eq!(session.front_lines()[0], logical);
    assert_eq!(session.self_lines()[0], logical);
    assert_eq!(session.log_lines()[0], logical);
    assert_eq!(session.language_lines()[0], logical);

    assert!(commit(&mut model, &painter, "7").is_none());
    assert_eq!(model.layer, "lang");
    assert!(commit(&mut model, &painter, "0").is_none());
    assert_eq!(model.layer, "front");
    let leaf = painter.language.leaf_path();
    assert!(!Path::new(&leaf).exists());
    assert!(commit(&mut model, &painter, "7").is_none());
    assert!(commit(&mut model, &painter, "70").is_none());
    assert_eq!(model.layer, "lang");
    assert!(model.error.contains("not on this list"));
    assert!(!Path::new(&leaf).exists());
    assert!(commit(&mut model, &painter, "84").is_none());
    assert!(!Path::new(&leaf).exists());
    let action = commit(&mut model, &painter, "73");
    assert_eq!(action.as_deref(), Some("set-zh-Hant"));
    let mut session = MenuSession::new(
        "rust3d",
        "1.0.1",
        painter,
        Box::new(|| "1.0.1".to_string()),
        Box::new(|| "about".to_string()),
        None,
    );
    session.model = model;
    assert_eq!(session.take_action(action), "stay");
    assert_eq!(session.model.layer, "front");
    assert_eq!(session.model.notice, "選單語言是繁體中文");
    assert_eq!(session.painter.path_label(), "路徑");
    assert_eq!(session.painter.language.code(), "zh-Hant");
    assert!(leaf.starts_with(home.to_str().unwrap()));
    assert_eq!(fs::read_to_string(&leaf).unwrap(), "zh-Hant\n");
    assert_eq!(
        fs::metadata(&leaf).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let parent = Path::new(&leaf).parent().unwrap();
    assert_eq!(
        fs::metadata(parent).unwrap().permissions().mode() & 0o777,
        0o700
    );
    let front = session.painter.display_rows("front");
    assert_eq!(front[2].short, "自我管理");
    assert_eq!(front[3].short, "語言");
    assert_eq!(front[0].short, "model");
    assert_eq!(front[0].kind, "model");
    assert_eq!(session.painter.display_rows("log")[0].short, "view-log");
    assert_eq!(session.painter.display_rows("lang")[0].short, "English");
    assert!(!cli::PRODUCT_VERBS.contains(&"language"));

    let mut loaded = MenuLanguage::new(Some(home.to_str().unwrap()), None);
    fs::write(&leaf, "nope\nzh-Hant\n").unwrap();
    loaded.load();
    assert_eq!(loaded.code(), "en");
    assert_eq!(loaded.path_label(), "Path");
    assert_eq!(fs::read_to_string(&leaf).unwrap(), "nope\nzh-Hant\n");
    set_env("RUST3D_LANG", "zh-Hant");
    let forced = MenuLanguage::new(Some(home.to_str().unwrap()), None);
    assert_eq!(forced.code(), "zh-Hant");
    assert_eq!(forced.path_label(), PATH_LABEL_ZH_HANT);
    assert_eq!(fs::read_to_string(&leaf).unwrap(), "nope\nzh-Hant\n");
    unset_env("RUST3D_LANG");
    fs::write(&leaf, "es\r\n").unwrap();
    loaded.load();
    assert_eq!(loaded.code(), "es");
    assert_eq!(loaded.path_label(), "Ruta");

    let mut empty_lang = MenuLanguage::new(Some(""), None);
    assert_eq!(empty_lang.save("es"), "Could not save the menu language");
    assert_eq!(empty_lang.code(), "en");
    session.painter.language.save("zh-Hant");
    let local = home.join(".local");
    fs::rename(&local, home.join(".local.kept")).unwrap();
    fs::write(&local, "x").unwrap();
    let message = session.painter.language.save("es");
    assert_eq!(session.painter.language.code(), "zh-Hant");
    assert_eq!(message, "無法儲存選單語言");

    assert_eq!(MenuPainter::display_width("╭─╮"), 3);
    assert_eq!(MenuPainter::display_width("█"), 1);
    assert_eq!(
        MenuPainter::display_width("路径"),
        MenuPainter::display_width("Path")
    );
    assert_eq!(MenuPainter::display_width("路徑"), 4);
    assert_eq!(MenuPainter::display_width("パス"), 4);
    assert_eq!(MenuPainter::display_width("경로"), 4);

    unset_env("RUST3D_LANG");
    let wide = 79usize;
    let mut english = MenuPainter::new(Some(home.to_str().unwrap()), None);
    let clock_en = clock.clone();
    english.clock_text = Box::new(move || clock_en.clone());
    let english_line = english.path_line(Some(wide));
    assert_eq!(MenuPainter::display_width(&english_line), wide);
    assert!(english_line.ends_with(&clock));
    let english_gap = gap_before(&english_line, &clock);
    for (label, code, first) in [
        ("路径", "zh-Hans", "路"),
        ("路徑", "zh-Hant", "路"),
        ("パス", "ja", "パ"),
        ("경로", "ko", "경"),
    ] {
        set_env("RUST3D_LANG", code);
        let mut labeled = MenuPainter::new(Some(home.to_str().unwrap()), None);
        let clock_row = clock.clone();
        labeled.clock_text = Box::new(move || clock_row.clone());
        assert_eq!(labeled.path_label(), label);
        let line = labeled.path_line(Some(wide));
        assert!(line.starts_with(&format!("{label}: ")), "{line}");
        assert!(line.ends_with(&clock), "{line}");
        assert_eq!(MenuPainter::display_width(&line), wide);
        assert_eq!(gap_before(&line, &clock), english_gap);
        assert_eq!(labeled.path_line(Some(1)), "");
        assert_eq!(labeled.path_line(Some(2)), first);
        assert_eq!(labeled.path_line(Some(3)), first);
        assert_eq!(labeled.path_line(Some(5)), format!("{label}:"));
    }

    let expected = [
        (
            "zh-Hans",
            vec![
                "1. model   : 把所选文件夹中的轮廓图做成三维模型",
                "4. 系统日志: 查看、清空，以及日志文件夹",
                "6. 自我管理: 版本、关于，以及 cargo 生命周期",
                "7. 语言    : 此菜单的显示语言",
                "9. 离开    : 离开",
            ],
        ),
        (
            "zh-Hant",
            vec![
                "1. model   : 把所選資料夾中的輪廓圖做成三維模型",
                "4. 系統日誌: 查看、清空，以及日誌資料夾",
                "6. 自我管理: 版本、關於，以及 cargo 生命週期",
                "7. 語言    : 這個選單的顯示語言",
                "9. 離開    : 離開",
            ],
        ),
        (
            "ko",
            vec![
                "1. model      : 고른 폴더의 윤곽 이미지로 3D 모델을 만듭니다",
                "4. 시스템 로그: 보기, 비우기, 로그 폴더",
                "6. 자기관리   : 버전, 정보, cargo 수명 주기",
                "7. 언어       : 이 메뉴의 표시 언어",
                "9. 종료       : 종료",
            ],
        ),
        (
            "ja",
            vec![
                "1. model       : 選んだフォルダの輪郭画像から3Dモデルを作る",
                "4. システムログ: 表示、消去、ログフォルダ",
                "6. 自己管理    : バージョン、概要、cargo のライフサイクル",
                "7. 言語        : このメニューの表示言語",
                "9. 終了        : 終了",
            ],
        ),
    ];
    for (code, rows) in expected {
        set_env("RUST3D_LANG", code);
        let mut board_painter = MenuPainter::new(Some(home.to_str().unwrap()), None);
        let clock_row = clock.clone();
        board_painter.clock_text = Box::new(move || clock_row.clone());
        assert_eq!(board_painter.language.code(), code);
        let board = board_painter.display_rows("front");
        assert_eq!(board_painter.format_rows(&board), rows);
        let mut model = MenuModel::new(None);
        let mut screen = ColumnScreen::new((24, 80));
        board_painter.paint(&mut screen, &mut model, APP_NAME, VERSION);
        occupy(&screen.calls);
        let placeable = screen.getmaxyx().1;
        let path_text = &screen.calls[0].2;
        assert_eq!(screen.calls[0].0, 0);
        assert_eq!(screen.calls[0].1, 0);
        assert!(path_text.starts_with(&format!("{}: ", board_painter.path_label())));
        assert!(path_text.ends_with(&clock));
        assert_eq!(MenuPainter::display_width(path_text), placeable as usize);
        let framed: Vec<_> = screen
            .calls
            .iter()
            .filter(|item| item.2.starts_with('╭'))
            .collect();
        assert_eq!(framed.len(), 1);
        assert_eq!(MenuPainter::display_width(&framed[0].2), placeable as usize);
    }
}

#[test]
fn about_status_log_and_cargo_lifecycle() {
    let _lock = lock();
    let _guard = ProcessGuard::new(&["DEBUG", "debug", "HOME", "USER", "RUST3D_LANG"]);
    unset_env("DEBUG");
    unset_env("debug");
    let home = temp_dir("rust3d-about-home-");
    let user = format!("cu{}", std::process::id());
    set_env("HOME", home.to_str().unwrap());
    set_env("USER", &user);
    let page = AboutPage::product(None);
    let body = page.framework_about();
    let pid = std::process::id();
    let preferred = format!("/dev/shm/cache/cache-rust3d-{user}-{pid}");
    let first = format!("/tmp/cache/cache-rust3d-{user}-{pid}");
    let second = format!("{}/.cache/cache-rust3d-{pid}", home.display());
    let persist = format!("{}/.local/rust3d", home.display());
    assert!(
        body.contains(&format!("    Cache folder (preferred): {preferred}")),
        "{body}"
    );
    assert!(
        body.contains(&format!("    Cache folder (1st fallback): {first}")),
        "{body}"
    );
    assert!(
        body.contains(&format!("    Cache folder (2nd fallback): {second}")),
        "{body}"
    );
    assert!(
        body.contains(&format!("    Persistence storage: {persist}")),
        "{body}"
    );
    assert!(!body.contains(".local/share/rust3d"), "{body}");
    let second_line = body
        .lines()
        .find(|line| line.contains("Cache folder (2nd fallback):"))
        .unwrap();
    assert!(!second_line.contains(&user), "{second_line}");
    if Path::new("/dev/shm").is_dir() {
        assert!(
            body.contains(&format!("    Cache folder used: {preferred}")),
            "{body}"
        );
    }
    assert!(!Path::new(&preferred).exists());
    assert!(!Path::new(&first).exists());
    assert!(!Path::new(&second).exists());
    assert!(!home.join(".local").join("rust3d").exists());
    let mut cache_at = body.find("Cache folder used:").unwrap();
    for label in [
        "Cache folder (preferred):",
        "Cache folder (1st fallback):",
        "Cache folder (2nd fallback):",
        "Persistence storage:",
    ] {
        let at = body.find(label).unwrap();
        assert!(cache_at < at, "{label}");
        cache_at = at;
    }
    assert!(body.contains("rust3d 1.0.1"), "{body}");
    assert!(body
        .contains("Domain: Build a glTF model and an HTML viewer from outline images in a folder"));
    assert!(body.contains("Runtime tools: none"));
    assert!(body.contains("Entry points: rust3d"));
    assert!(body.contains("Rustc:"));
    assert!(body.contains("cargo location:"));
    assert!(body.contains("rustup location:"));
    assert!(body.contains("Target triple:"));
    assert!(body.contains("Operation System:"));
    assert!(body.contains("[CHECK SYSTEM]:"));
    for banned in ["pyenv", "conda", "Cython", "Python "] {
        assert!(!body.contains(banned), "{banned}");
    }
    let lines = page.about_box_lines(Some("/tmp/rust3d-bin"), Some("uninstalled"), Some(""));
    let width = lines[0].chars().count();
    assert!(lines[0].starts_with('*'));
    assert!(lines.iter().all(|line| line.chars().count() == width));
    assert!(!lines.join("\n").contains("Installation command"));
    let with_url = page.about_box_lines(
        Some("/tmp/rust3d-bin"),
        Some("local"),
        Some("https://crates.io/crates/rust3d"),
    );
    assert!(with_url.join("\n").contains("cargo install rust3d"));
    let root = std::env::current_dir().unwrap();
    let source = root.join("src").join("lib.rs");
    assert_eq!(
        page.check.install_kind(source.to_str().unwrap()),
        "uninstalled"
    );
    assert_eq!(page.check.install_kind("/usr/bin/rust3d"), "global");

    let base = temp_dir("rust3d-log-");
    let log_dir = base.join("log");
    let quiet = StatusLog::new(
        "rust3d",
        true,
        base.to_str().unwrap(),
        log_dir.to_str().unwrap(),
    );
    assert_eq!(quiet.log_name(), "rust3d");
    assert!(!quiet.is_debug());
    assert!(quiet.is_quiet());
    quiet.log_message("hello", "info", "main");
    let daily = fs::read_dir(&log_dir)
        .unwrap()
        .filter_map(|item| item.ok())
        .map(|item| item.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .starts_with("rust3d-")
        })
        .unwrap();
    let stored = fs::read_to_string(&daily).unwrap();
    assert!(stored.contains("pid:"), "{stored}");
    assert!(stored.contains("[INFO] @main :] hello"), "{stored}");
    let camel = StatusLog::new(
        "FooBar",
        true,
        base.to_str().unwrap(),
        log_dir.to_str().unwrap(),
    );
    assert_eq!(camel.log_name(), "foo-bar");
    set_env("DEBUG", "1");
    let debug = StatusLog::new(
        "rust3d",
        false,
        base.to_str().unwrap(),
        log_dir.to_str().unwrap(),
    );
    assert!(debug.is_debug());
    assert_eq!(StatusLog::class_version(), "1.0.1");

    let owned_dir = temp_dir("rust3d-syslog-");
    let logger = std::rc::Rc::new(StatusLog::new(
        "rust3d",
        true,
        base.to_str().unwrap(),
        owned_dir.to_str().unwrap(),
    ));
    let logs = SystemLog::new(Some(logger));
    assert!(logs.folder_text().starts_with("Log folder:"));
    let owned = owned_dir.join("sample.log");
    fs::write(&owned, "line\n").unwrap();
    fs::write(owned_dir.join("skip.txt"), "no").unwrap();
    let listed = logs.log_files();
    assert!(listed
        .iter()
        .any(|path| path.file_name().and_then(|name| name.to_str()) == Some("sample.log")));
    assert!(listed
        .iter()
        .all(|path| path.extension().and_then(|ext| ext.to_str()) == Some("log")));
    assert_eq!(logs.read_log(&owned).as_deref(), Some("line\n"));
    assert!(logs.clear_log(&owned));
    assert_eq!(fs::read_to_string(&owned).unwrap(), "");
    assert!(owned.is_file());
    assert!(logs
        .read_log(owned_dir.join("skip.txt").as_path())
        .is_none());
    let none = SystemLog::new(None);
    assert_eq!(none.folder_text(), "No log folder.");

    fn stub(argv: &[String]) -> (i32, String, String) {
        assert_eq!(argv[0], "cargo");
        (0, "ok".into(), String::new())
    }
    let mut manage = SelfManage::new("rust3d", "1.0.1", None);
    assert_eq!(manage.local_version(), "rust3d 1.0.1");
    assert_eq!(
        manage.argv_for("version-check"),
        vec!["cargo", "search", "rust3d", "--limit", "1"]
    );
    assert_eq!(
        manage.argv_for("self-update"),
        vec!["cargo", "install", "rust3d", "--force"]
    );
    assert_eq!(
        manage.argv_for("self-install"),
        vec!["cargo", "install", "rust3d"]
    );
    assert_eq!(
        manage.argv_for("self-uninstall"),
        vec!["cargo", "uninstall", "rust3d"]
    );
    manage.set_runner(stub);
    let mut out = Vec::new();
    assert_eq!(manage.emit("version-check", &mut out), 0);
    let text = String::from_utf8(out).unwrap();
    assert!(text.contains("$ cargo search rust3d --limit 1"), "{text}");
    assert!(text.contains("rust3d 1.0.1"));

    let mut app = Cli::new(None);
    app.self_manage.set_runner(stub);
    let (code, out, _) = run(&mut app, &["version"]);
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "rust3d 1.0.1");
    let (code, _, err) = run(&mut app, &["self-uninstall"]);
    assert_eq!(code, 1);
    assert!(
        err.contains("self-uninstall removes this package with cargo."),
        "{err}"
    );
    let (code, out, _) = run(&mut app, &["self-install"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("$ cargo install rust3d"), "{out}");
    let (code, _, err) = run(&mut app, &["version", "--force"]);
    assert_eq!(code, 1);
    assert!(err.contains("--force is only for self-uninstall"), "{err}");
}

#[test]
fn readme_dependencies_and_version_ssot() {
    let cargo = fs::read_to_string("Cargo.toml").unwrap();
    let package = cargo.split("[dependencies]").next().unwrap();
    let version = package
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("version = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .unwrap();
    assert_eq!(version, VERSION);
    assert_eq!(VERSION, "1.0.1");
    assert!(package.contains("default-run = \"rust3d\""));
    let bins: Vec<_> = cargo
        .lines()
        .filter(|line| line.trim() == "[[bin]]")
        .collect();
    assert_eq!(bins.len(), 1, "{cargo}");
    assert!(cargo.contains("name = \"rust3d\""));
    assert!(!cargo.contains("name = \"convert\""));
    assert!(std::path::Path::new("src/bin/rust3d.rs").is_file());
    assert!(!std::path::Path::new("src/bin/convert.rs").exists());
    let packaging = fs::read_to_string("docs/requirements/requirement-rust-packaging.md").unwrap();
    assert!(
        packaging.contains("exactly one binary, `rust3d`"),
        "{packaging}"
    );
    assert!(
        packaging.contains("`default-run` **MUST** be `rust3d`"),
        "{packaging}"
    );
    assert!(
        packaging.contains("**MUST NOT** declare `convert`"),
        "{packaging}"
    );
    assert!(cargo.contains("chrono"));
    assert!(cargo.contains("image"));
    assert!(cargo.contains("serde_json"));
    assert!(cargo.contains("unicode-width"));
    assert!(!cargo.contains("opencv"));
    assert!(!cargo.contains("numpy"));
    assert!(!cargo.contains("ChronicleLogger"));

    let law =
        fs::read_to_string("docs/requirements/requirement-rust-dependency-management.md").unwrap();
    for spec in [
        "chrono>=0.4",
        "image>=0.25",
        "serde_json>=1",
        "unicode-width>=0.2",
    ] {
        assert!(law.contains(spec), "{spec}");
    }
    assert!(law.contains("`opencv-python` is not declared"), "{law}");
    assert!(!cargo.contains("requirements.txt"));

    let readme = fs::read_to_string("README.md").unwrap();
    assert!(!readme.contains("cargo run --bin"), "{readme}");
    assert!(!readme.contains("checkout binary"), "{readme}");
    assert!(readme.contains("One binary, `rust3d`. `cargo run` starts it"));
    let changelog = fs::read_to_string("CHANGELOG.md").unwrap();
    let current = changelog.split("## Historical").next().unwrap();
    assert!(!current.contains("--bin"), "{current}");
    assert!(current.contains("The only binary is `rust3d`"));
    assert!(readme.contains(&format!("Version-{version}-blue")));
    assert!(readme.contains("local clock"));
    assert!(readme.contains("HH:MM:SS"));
    assert!(!readme.contains("setup.sh"));
    assert!(!readme.contains("sudo pip"));
    assert!(!readme.contains("curl | sh"));
    let headings: Vec<_> = readme
        .lines()
        .filter_map(|line| {
            line.strip_prefix("## ")
                .filter(|rest| !rest.starts_with('#'))
        })
        .collect();
    assert_eq!(
        headings,
        vec![
            "Features",
            "Advantages",
            "Quick Installation",
            "Usage",
            "Screenshots",
            "Examples",
            "Platform Compatibility",
            "Related Projects",
            "Contributing",
            "License",
            "Last Update",
        ]
    );
    let image_base = "https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/";
    let viewer = "https://cloudgen.github.io/rust3d/sample-images/model/viewer.html";
    let mut names: Vec<_> = fs::read_dir("screenshots")
        .unwrap()
        .map(|item| item.unwrap().file_name().to_string_lossy().to_string())
        .filter(|name| name.ends_with(".png"))
        .collect();
    names.sort();
    assert!(!names.is_empty());
    for name in &names {
        assert!(readme.contains(&format!("{image_base}{name}")), "{name}");
    }
    assert!(readme.contains(viewer));
    assert!(readme.contains(&format!("]({image_base}model-viewer.png)]({viewer})")));
    assert!(!readme.contains("](screenshots/"));
    assert!(!readme.contains("](./screenshots/"));
    let related = readme
        .split("## Related Projects")
        .nth(1)
        .unwrap()
        .split("## Contributing")
        .next()
        .unwrap();
    let urls: Vec<_> = related
        .split("](")
        .skip(1)
        .filter_map(|part| part.split(')').next())
        .filter(|url| url.starts_with("https://"))
        .collect();
    assert_eq!(
        urls,
        vec![
            "https://github.com/cloudgen/rust3d",
            "https://crates.io/crates/rust3d",
            "https://github.com/cloudgen/OutlineImage",
            "https://pypi.org/project/OutlineImage/",
            "https://github.com/Wilgat/AnimeDlp",
            "https://github.com/Wilgat/ChronicleLogger",
            "https://github.com/Wilgat/VideoSpeed",
            "https://github.com/cloudgen/ciao",
            "https://github.com/cloudgen/ciao-lite",
            "https://github.com/cloudgen/safe-rm",
        ]
    );
    let viewer_section = readme.split("### `model-viewer.png`").nth(1).unwrap();
    let viewer_section = viewer_section.split("\n### ").next().unwrap();
    assert!(viewer_section.contains("not a text-menu capture"));
    assert!(!readme.contains("video.png"));
    assert!(!readme.contains("ThreeDimensionModeller"));
    assert!(!readme.contains("pyproject.toml"));
}
