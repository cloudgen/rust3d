**file**: docs/requirements/requirement-rust-oop.md
**Status**: Active (Version 1.0.7 – About is row 63. Cargo menu rows are 64–67. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-oop`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Define when rust3d uses a struct, and name the file that holds each struct. One struct lives in one module. `fn main` stays in `src/cli.rs`. Every struct in the map that logs accepts the one status logger in its constructor so that struct can log. The line text stays on `requirement-rust-cli-logging`.

The menu picture stays on `requirement-rust-tui`. The about page stays on `requirement-rust-about`. The domain sentence and the conversion functions stay on `requirement-domain-rust3d`. A publish-from-temp, when one exists, stays `std::fs::rename` on `requirement-rust-coding-style`. This file decides the class and the file. It does not invent a second picture, a second about page, or a second conversion.

The target is an ordinary multi-class split. A StateLogic + `Attr` rewrite stays unordered. This file does not claim that rewrite. The notes still record the older three-module gap. `CheckSystem` and `AboutPage` are on disk for the about page. `TP-OOP-*` stay todo.

### 1.1 Human-facing

**In one sentence:** Each struct lives in its own file, `main` stays in `cli.rs`, and a logging struct receives the logger in `::new`.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Maintainer adding a prompt, a log action, or an outline step | Put it on the class or the module that already owns that job |
| The other role | The menu picture, the about facts, and the conversion | Their own requirements |
| Not this file | A StateLogic rewrite of `main` | `requirement-rust-coding-style` leaves that rewrite unordered |

| Includes | Excludes |
|----------|----------|
| One class per file. The class is the only public surface of that module. Constants that `Cli` owns live on struct `Cli` | A second class in the same file, a module-level function beside the class, or a module-level assignment of `Cli`'s identity, verb lists, or suffixes |
| `fn main` in `src/cli.rs` | Moving `main` to another module |
| Ordinary classes named in the map below | One class per function. A StateLogic rewrite |
| The one logger passed into `::new`. The struct is constructed at the site that needs the object | A setter, a module global, a second logger built inside the struct, or a factory |

| Surface | What you open | What for |
|---------|---------------|----------|
| `src/cli.rs` | `fn main` and struct `Cli` | Entry. Builds the other objects and runs one job |
| `src/menu.rs` | struct `Tui` | Text-menu session |
| `src/model.rs` | `convert_folder` | The conversion for the verb `model` and menu row 1. Checkout is `cargo run -- model` |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Add a menu prompt | It is a method on `Tui`. The frame is `MenuPainter`. | Edit the class file this requirement names |
| Change the conversion | It is `convert_folder`. The verb `model` and menu row 1 use it. Checkout is `cargo run -- model`. | Edit `src/model.rs` |
| Start the program | The binary calls `rust3d::cli::main`. | `rust3d` |
| Name a constant the program uses | It is an attribute of the class that owns it. `Cli` owns identity and the verb lists. | `Cli.APP_NAME` |

## 2. Core Rules (Mandatory)

### 2.1 Use a class when the functions share one job

1. When a set of functions shares one job, the crate **MUST** make them methods of one struct and put that struct in a module under `src/`.
2. **MUST NOT** make a class whose only method is a single function that nothing else in the file shares.
3. The class **MUST** be the only public surface of its module. A module-level `def` beside that class is a second home. **MUST NOT** add one.
4. That module **MUST** define one class. One exception type raised by that class **MAY** share the file. `MenuScreenError` shares `menu.rs` with `Tui`.
5. Menu types share `src/menu.rs`. `AboutPage` and `CheckSystem` share `src/about.rs`. `SelfManage` is `src/self_manage.rs`. `MenuLanguage` is `src/language.rs`. `StatusLog` is `src/status_log.rs`. `SystemLog` is `src/system_log.rs`.
6. A full StateLogic + `Attr` rewrite of `main` stays unordered (`requirement-rust-coding-style`). This file does not order that rewrite. The classes in the map are ordinary classes. The allowed end state is this map. **MUST NOT** collapse the end state back to a single procedural module. `model.rs` is the conversion module. Its functions are the allowed module-level surface for that job, because the verb `model`, the menu, and the tests call them. Checkout is `cargo run -- model`.

### 2.2 Entry: `main` stays in `cli.rs`

7. `src/cli.rs` **MUST** define struct `Cli` and **MUST NOT** define another class.
8. `fn main` **MUST** stay in `src/cli.rs`. The signature stays `main(argv=None, basedir="", logdir="")`. `main` writes `StatusLog::new` (`requirement-rust-cli-logging`), builds `Cli`, and runs one job. Empty `basedir` and `logdir` leave folder resolution to the library.
9. The binary **MUST** stay `rust3d` from `src/bin/rust3d.rs`, and that file **MUST** call `rust3d::cli::main`.
10. `cli.rs` **MUST NOT** define another struct's methods. `Cli` owns the parser, dispatch, help, version, about, model, lifecycle, unknown-verb, and tty checks. `Cli` does not own the StatusLog construct. Cargo argument lists stay on `SelfManage`. The about body stays on `AboutPage`. The conversion stays on `model.rs`. The frame stays on `MenuPainter`.
11. `Cli` constructs the other classes and calls them. Each class receives its collaborators through its constructor.
11f. A constant or variable that struct `Cli` or `fn main` owns is an attribute of struct `Cli`, or a name assigned inside `main()`. It is not a module-level assignment in `cli.rs`. That set is `APP_NAME` (`rust3d`), `CONSOLE_NAME` (`rust3d`), `PRODUCT_VERBS`, `LIFECYCLE_VERBS`, `AUTHOR_NAME`, `AUTHOR_EMAIL`, `HOMEPAGE`, `LAST_UPDATE`, `DOWNLOAD_URL`, `BASIC_USAGE`, and `DOMAIN_SENTENCE`. Input suffixes stay on `model.rs`. The about page prints the author name, the homepage, the box date, and the usage line. It does not print the email. The email stays on `Cli`. `Cli` reads the package version from `VERSION` only and **MUST NOT** keep a second literal, including a fallback `"1.0.0"`. `Cli` **MUST NOT** keep `MAJOR`, `MINOR`, or `PATCH` integers. `Cli` passes the values other classes print. Those classes do not import the names from the `cli` module.
11g. Another class keeps its own constants. The frame glyphs, `MENU_ROWS`, `SELF_ROWS`, `LOG_ROWS`, `PATH_LABEL_EN`, and `PATH_LABEL_ZH_HANT` stay on `MenuPainter`. `Cli` does not own those names.

### 2.2a The logger arrives through `::new`

11a. Every struct in the map that logs **MUST** accept the one StatusLog as a parameter of `::new`.
11b. `::new` **MUST** store that object. A struct that constructs another struct in the map **MUST** pass that same object into that constructor.
11c. `fn main` writes `StatusLog::new` before `Cli` (`requirement-rust-cli-logging`) and **MUST** pass that object into `Cli::new`. The parameter **MAY** be absent so a caller that has no logger can omit it. `main` has the logger and **MUST NOT** omit it.
11d. **MUST NOT** construct a StatusLog inside a struct or from a method. **MUST NOT** attach the logger through a setter or a module global after `::new` returns. **MUST NOT** call a module function to write the instantiation line. `::new` logs it.
11e. The instantiation line, its level, and quiet stay on `requirement-rust-cli-logging`. This file owns only that the logger arrives through `::new`.
11h. The site that needs an object **MUST** write `ClassName(...)`. A factory is a function or a method whose job is to instantiate a class. This map does not include a factory. The class can be created without one. The ban on that factory also lives on `requirement-rust-coding-style`.

### 2.3 Text menu

12. `src/menu.rs` **MUST** define struct `Tui` and the exception `MenuScreenError`. No other class. `Tui` owns `open_text_menu`, `front_lines`, `self_lines`, `log_lines`, `language_lines`, `model_result`, `about_on_screen`, `help_on_screen`, `read_key`, and `notice`. `model_result` calls `convert_folder`.
13. Struct `MenuPainter` in `src/menu.rs` owns `paint`, `paint_prompt`, `path_label`, `path_line`, `clock_text`, `format_rows`, `display_rows`, `folder_rows`, `screen_can_hold`, `display_width`, `clip_columns`, and `clip_columns_right`. The frame glyphs, `MENU_ROWS`, `SELF_ROWS`, `LOG_ROWS`, `PATH_LABEL_EN`, and `PATH_LABEL_ZH_HANT` live on that class. `display_width`, `clip_columns`, and `clip_columns_right` are methods of that class. The module does not define them beside the class. Wide and Fullwidth count as two. Ambiguous stays one. The column rule stays on `requirement-rust-tui`. `FRAME_TOP_LEFT` **MUST NOT** appear in `cli.rs`. `path_label`, `clock_text`, and `path_line` draw the first row of the front board, of the self-management board, of the system-log board, and of the language board. The clock shape, shortening the path from the left, and when the clock is omitted stay on `requirement-rust-tui` rule 13. The selected word stays on `requirement-rust-cli-language`. `path_label` returns that word. It does not read `LANG` or `LC_ALL`, and it does not invent a second line. `MenuPainter::new` writes `MenuLanguage::new`.
14. Struct `MenuModel` in `src/menu.rs` owns `apply_key`, `edge_keys`, `highlighted_row`, and `buffer_text`.
15. Struct `MenuSession` in `src/menu.rs` owns `run`, `take_action`, `wait_key`, and `leave_to_front`. `wait_key` is the one-second clock wait from `requirement-rust-tui` rule 13, including the language board. It does not start a thread. `run` loops until row 9 Exit. A leaf action returns to the front board. A language pick saves through the shared `MenuLanguage` and returns to the front board. `MenuSession` dispatches model, system-log, language, self-management, and exit. Kind `model` switches to layer `folders`. Kind `pick:` runs the conversion. It does not paint the frame.
15a. Struct `MenuLanguage` in `src/language.rs` owns the thirteen codes, the language leaf, and the menu words. One struct. No second struct in that file. The logger arrives through `::new`. The codes and the leaf stay on `requirement-rust-cli-language`.
16. The menu picture stays on `requirement-rust-tui`. These classes implement that picture. They do not invent a second layout.
17. Until an implement order lands this split, `cli.rs` still holds the prompt session. After the split, `cli.rs` **MUST NOT** hold `Tui`, `MenuPainter`, `MenuModel`, or `MenuSession` methods.

### 2.4 About page

18. Struct `AboutPage` in `src/about.rs` owns `framework_about`, the star box, `install_kind`, and the install sentence. `framework_about` composes the page in `requirement-rust-about`. The domain sentence stays on `requirement-domain-rust3d`. The install sentence names global, local, or uninstalled. It **MUST NOT** name a downloaded-script installer. Struct `CheckSystem` in `src/about.rs` owns the host-check reads. It **MUST NOT** probe pyenv, conda, or Cython. `AboutPage` calls that object. It **MUST NOT** construct a second checker for the same page. Row 63 and the `about` verb call `framework_about`.
19. This product has no struct `Join` and no `src/join.py`. **MUST NOT** add an encoder struct. The model functions stay in `src/model.rs`: `list_subfolders`, `list_images`, `folder_for_pick`, `output_dir_for`, `outline_to_solid_mask`, `reconstruct_visual_hull`, `convert_folder`, and `convert_main`. The typed verb `model` calls `convert_main`. Checkout is `cargo run -- model`.

### 2.5 The other jobs

20. Struct `SystemLog` in `src/system_log.rs` owns `log_dir`, `log_files`, `read_log`, `clear_log`, and `folder_text`. The menu rows and the result-page words stay on `requirement-rust-tui`. `Tui` writes `SystemLog(...)` at the site that needs it and passes the same logger. `cli.rs` **MUST NOT** list the log files, read one, or empty one.
21. Struct `SelfManage` in `src/self_manage.rs` owns `local_version`, `argv_for`, `run_cargo`, and `emit`. `local_version` reads `VERSION` and does not call cargo. `argv_for` builds the cargo argument lists for `version-check`, `self-update`, `self-install`, and `self-uninstall`. The verb names and the command strings stay on `requirement-rust-cli-interface` and `requirement-rust-tui`. `cli.rs` **MUST NOT** build those cargo argument lists. The about body is not this struct.
22. Version display stays the single string `VERSION` in `src/lib.rs`. **MUST NOT** add a second integer triple.

### 2.6 Implementation Notes (this project)

| Class | File | Owns |
|-------|------|------|
| `Cli` | `src/cli.rs` | Parser, dispatch, verbs, tty checks. Identity constants and verb lists. `fn main` stays in this file |
| `Tui` | `src/menu.rs` | Text-menu session (rule 12). Shares the file with `MenuScreenError` |
| `MenuPainter` | `src/menu.rs` | Frame, glyphs, `MENU_ROWS`, `SELF_ROWS`, `LOG_ROWS`, `PATH_LABEL_EN`, `PATH_LABEL_ZH_HANT`, `paint`, `path_label`, `path_line`, `clock_text`, `display_width`, `clip_columns`, `clip_columns_right`. Writes `MenuLanguage::new` |
| `MenuLanguage` | `src/language.rs` | Thirteen codes, the language leaf, and the menu words |
| `MenuModel` | `src/menu.rs` | Keystroke state. Shares the painter’s language object |
| `MenuSession` | `src/menu.rs` | `run`, `take_action`, `wait_key`, `leave_to_front` |
| `SystemLog` | `src/system_log.rs` | Log folder, file list, read, empty |
| `SelfManage` | `src/self_manage.rs` | Local version and the cargo commands for rows 64–67 |
| `CheckSystem` | `src/about.rs` | Host-check reads for the about page. Does not draw the star box |
| `AboutPage` | `src/about.rs` | `framework_about`, the star box, and the install kind. Calls `CheckSystem` |
| conversion functions | `src/model.rs` | `list_subfolders`, `list_images`, `folder_for_pick`, `convert_folder`, `convert_main`. Not a class. The typed verb `model` calls `convert_main` |

| Item | Value |
|------|--------|
| **Binary** | `rust3d` |
| **Module entry** | `src/bin/rust3d.rs` calls `rust3d::cli::main` |
| **Running tree** | The class files in the map are on disk, plus `model.rs`. Identity constants live on struct `Cli`. There is no `VIDEO_SUFFIXES` and no second version literal |
| **Landed** | `MenuLanguage`, `AboutPage`, `CheckSystem`, and `model.rs` are on disk. `TP-OOP-01` through `TP-OOP-04` stay todo |
| **Logger** | A logging struct accepts the logger in `::new` and stores it. `fn main` writes `StatusLog::new` and then builds `Cli`. Collaborators are constructed at the site that needs them |
| **Not this split** | Menu rows, about line facts, the conversion steps, version string, the wording of the instantiation line |

### 2.7 Why This Requirement Exists (CIAO)

- **Principle 2 – Intentional**: Each job has one struct and one file. That struct receives the logger in `::new`.
- **Principle 5 – SSOT**: `cli.rs` does not keep a second copy of another class's methods. `main` has one home.
- **Principle 1 – Caution**: The picture, the about facts, and the conversion stay where their requirements already put them.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, each struct still receives the logger through `::new` at this login. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to construct a class or to pass the logger. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`.

## Sample code

StateLogic stays unordered. The site constructs the struct. This sample is the allowed shape. There is no `Join`.

```rust
fn main() {
    let logger = StatusLog::new("rust3d", quiet);
    let app = Cli::new(Some(logger));
    app.run();
}
```

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Do not add an encoder struct. The modules in the map are on disk.
- **Intentional:** One job, one struct, one file. There is no `Join`.
- **Anti-fragile:** `fn main` and the console script keep their addresses.
- **Over-protect:** No factory, no second logger, no StateLogic rewrite hidden inside this map.

## 4. Protection Rule (Sacred)

**Future AI assistants or maintainers MUST NOT:**

1. Move `fn main` out of `src/cli.rs`.
2. Put two classes in one module, except `MenuScreenError` beside `Tui`.
3. Leave a class that has only one method.
4. Construct StatusLog inside a class.
5. Add a factory whose job is to instantiate a class in this map.
6. Put `MENU_ROWS`, `SELF_ROWS`, `LOG_ROWS`, or frame glyphs on `Cli`.
7. Keep a second version literal on `Cli`.
8. Add an encoder class or a file-stage class for this product.
9. Order a StateLogic + `Attr` rewrite from this file.
10. Mark `TP-OOP-*` have before the proofs pass.
11. Delete the running `cli.rs` behavior in a requirement-only edit.

**Violating this rule is a critical structure regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | The map names `Cli`, `Tui`, `MenuPainter`, `MenuLanguage`, `MenuModel`, `MenuSession`, `SystemLog`, `SelfManage`, `CheckSystem`, and `AboutPage`. There is no `Join` |
| AC-2 | `fn main` stays in `cli.rs`. Binary `rust3d` calls `rust3d::cli::main` |
| AC-3 | A logging struct accepts the logger in `::new` |
| AC-4 | `Cli` owns the identity constants. `MenuPainter` owns the row tuples and the glyphs |
| AC-5 | `AboutPage` composes the about page. `CheckSystem` owns the host check. `model.rs` owns the conversion |
| AC-6 | The modules in rule 3 and rule 5 are on disk |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-cli-logging` | Construct, `instantiated` line, quiet |
| `requirement-rust-tui` | Picture the classes draw |
| `requirement-rust-cli-interface` | Verb names `Cli` dispatches |
| `requirement-rust-about` | About page line text. This file names `AboutPage` and `CheckSystem` |
| `requirement-domain-rust3d` | Domain sentence and the conversion |
| `requirement-video-ffmpeg-pipeline` | Concat order `Join.run` calls |
| `requirement-rust-coding-style` | `std::fs::rename`. StateLogic stays unordered. Factory ban |
| `requirement-rust-project-structure` | Running tree versus this map |
| `requirement-rust-packaging` | Console script and `VERSION` |
| `requirement-class-software-dev` | Residual pointer |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-OOP-01** | `tests/product.rs` | todo | The struct files in the map exist. `cli.rs` defines `Cli` and `fn main` |
| **TP-OOP-02** | `tests/product.rs` | todo | A logging struct takes the logger in `::new` and logs `instantiated` |
| **TP-OOP-03** | `tests/product.rs` | todo | `MenuPainter` owns the frame glyphs and the row tables |
| **TP-OOP-04** | `tests/product.rs` | todo | Display version is `VERSION` only. Input suffixes stay on `model.rs` |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Python OOP style levels

**Definition:** **Python OOP style levels** are a **portable ladder** (L0–L3) for how much object-oriented structure a **Python CLI / library product** uses—from pure procedural scripts to multi-class SRP designs to **framework-grade** orchestrator patterns (e.g. StateLogic+Attr). Levels are **architecture shape labels**, not project classes. They guide **revision design**, product **requirements**, and skill choice without forcing a rewrite of working L1 code for style alone.

**Human daily-life explanation:**

**Python OOP style levels** are a ladder from plain scripts (L0) up to framework-grade orchestrators (L3). They label kitchen organization, not which kind of shop this is.

**Daily-life example:** Cooking from a scribbled list vs a boxed mise-en-place system with labeled bowls—same dinner, different kitchen organization.

### Avoid a factory that only creates a class

**Definition:** **Avoid-factory-create-class** is the portable Python construct rule that an object is created by writing `ClassName(...)` at the site that needs that object. A **factory** is a function or a method whose job is to instantiate a class: a module function that returns `SomeClass(...)`, a method that returns `SomeClass(...)`, or `Cls.__new__(Cls)` followed by a method that builds the object. The class can be created without that factory. The factory does not add a way to create the object that the call site lacks. It hides the class name behind a second home.

**Human daily-life explanation:**

**Avoid-factory-create-class** means you pick up the thing itself. You do not hand the job to a clerk whose only task is to pick it up and hand it back. The class already knows how to become an object. Writing `ClassName(...)` is that step. A factory is the clerk.

**Daily-life example:** You want a kettle. You take a kettle off the shelf. You do not fill out a slip that a back-room clerk uses to fetch the same kettle and pass it through a window. That slip is a factory. The kettle was already on the shelf.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-04 | Active 1.0.0 | L2 struct map for rust3d. Running tree remains three modules. StateLogic unordered. Proofs todo |
| 2026-10-04 | Active 1.0.1 | `MenuPainter` owns `path_label`, `PATH_LABEL_EN`, and `PATH_LABEL_ZH_HANT`. Those methods do not detect a language. `TP-OOP-*` stay todo |
| 2026-10-04 | Active 1.0.2 | Struct `MenuLanguage` is the language home. `MenuPainter` writes `MenuLanguage::new`. `TP-OOP-*` stay todo |
| 2026-10-04 | Active 1.0.3 | `display_width`, `clip_columns`, and `clip_columns_right` are methods of `MenuPainter`. `TP-OOP-*` stay todo |
| 2026-10-04 | Active 1.0.4 | `CheckSystem` owns the host check. `AboutPage` owns `framework_about`. No second version triple. `TP-OOP-*` stay todo |
| 2026-10-05 | Active 1.0.5 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.0.6 | The conversion stays `convert_folder` and `convert_main`. There is no `convert` binary. Checkout is `cargo run -- model`. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.0.7 | About is row 63. Cargo menu rows are 64–67. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
