**file**: docs/requirements/requirement-rust-tui.md
**Status**: Active (Version 1.2.8 – System-log is front row 4. Children are 41–43. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-tui`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

This file is the picture of rust3d’s text menu. The screen has two parts on one board: the menu, and a bottom input box. The first row of the menu region keeps the current path on the left and, when that row has room, a clock of the local time on the right. That clock is drawn again each second. It does not show the product name or the package version. That picture is the default TUI style.

The session is struct `Tui` in `src/menu.rs`. The frame is struct `MenuPainter` in `src/menu.rs`. Those homes are `requirement-rust-oop`. `fn main` stays in `src/cli.rs`. Struct `Cli` constructs `Tui` and passes this product’s name, version, and front board into that session. `cli.rs` does not keep a second frame painter. The product does not import another menu package.

Which verb runs stays on `requirement-rust-cli-interface`. Outline steps, the folder board, and the about facts stay on `requirement-domain-rust3d`. The encoder file is Retired. Status lines stay on `requirement-rust-cli-logging`.

Struct `Tui` and struct `MenuPainter` are on disk. The default path label is English `Path`. `MenuPainter` stores `路徑`. `requirement-rust-cli-language` selects the word. This file does not detect a language. A menu column is a display column. East Asian Wide and Fullwidth count as two. Ambiguous stays one. `TP-TUI-09` and `TP-TUI-10` have. `TP-TUI-01` through `TP-TUI-08` stay todo.

### 1.1 Human-facing

**In one sentence:** On a terminal, `rust3d` with no arguments shows a numbered menu and, along the bottom, a rounded box as wide as the screen, and you type on the only line inside that box.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | The person at the keyboard | `rust3d` |
| The other role | The session and the painter | struct `Tui` in `menu.rs`; struct `MenuPainter` in `menu.rs` |
| Not this file | How one image becomes an outline | The domain file |

| Includes | Excludes |
|----------|----------|
| The menu region above the box, and the rounded three-row frame | A `Choice:` line, `print` / `read`, or `input()` for that menu |
| The first row: Path and the folder on the left; a local clock on the right when the row has room, drawn again each second | The product name and the version on that first row. They stay on the status line |
| UTF-8 arc corners and bars, full screen width, typing on the only inner line | Frame glyphs copied into `cli.rs`, and a menu crate |
| The status line under the frame, and this product’s front rows (model, system-log, self-management, language, Exit) | Putting the numbered rows inside the frame. Row 2 stays omitted. System-log is row 4. Do not renumber 4, 6, 7, or 9 to close a gap |

| Surface | What you open | What for |
|---------|---------------|----------|
| `rust3d` on a terminal | Text screen | Menu above, input box on the bottom, status line on the last row |
| `src/cli.rs` | `fn main` and struct `Cli` | Builds the session. Does not paint the frame |
| `src/menu.rs` | struct `Tui` | Session loop |
| `src/menu.rs` | struct `MenuPainter` | Frame and columns |
| `rust3d` with no terminal | Help text | No screen and no wait. Exit 0 |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Start on a terminal with no words | You see model, system-log, self-management, language, and Exit. The program does not convert a folder yet and does not run cargo. The bottom of the screen is one rounded box, then a status line. | `rust3d` |
| Read the first row | The left side names the folder this process is in. The word before that colon is Path until a saved language selects another word. When the row has room, the right side is a clock of the local time, hours, minutes, and seconds. It moves once a second while this board is showing. The name and the version stay on the status line under the box. | `rust3d` |
| Choose model | The screen stays up. The next board is **1** current folder, then each immediate subfolder, then **0** back. Choosing a folder shows one waiting sentence, then runs the conversion and shows the lines on the result page. Esc or **0** returns to the front board and does not convert. | `1` then Enter, then a folder number |
| Choose view-log | The screen lists the log files and waits in the bottom box. It stays until you type a file number and Enter, or press Esc. A pause does not send you back to the system-log board. | `4`, then `41` |
| Start with no terminal | Help text and exit 0. The program does not draw the box and does not wait. | `rust3d` with no terminal |

## 2. Core Rules (Mandatory)

1. **Package writer.** The text-menu session **MUST** be struct `Tui` in `src/menu.rs`. The frame **MUST** be drawn by struct `MenuPainter` in `src/menu.rs` (`requirement-rust-oop`). rust3d **MUST** pass its product name, package version, and front board into that session. `cli.rs` **MUST NOT** contain the frame glyphs (`FRAME_TOP_LEFT` belongs to `MenuPainter`). The product **MUST NOT** import `py_tui` and **MUST NOT** declare `py-tui`. The menu **MUST NOT** be drawn with `print` / `read` / `input()` or a `Choice:` line. After **model** is chosen, the folder board stays on this same screen. A chosen folder's result page **MUST NOT** use `input()`. The typed verb `model` does not open this screen.
2. **Two regions.** On the front board the writer **MUST** draw a menu region and a bottom input box on the same screen. The menu region is every row above the box. Its first row **MUST** be the path line in rule 13. That row **MUST NOT** show the product name or the package version. The numbered rows for that board **MUST** follow the path line. Those rows **MUST NOT** be drawn inside the box. Each numbered row is a number, a verb, and an explain. On that board the number field is as wide as the longest number, and a shorter number is padded with spaces on the left. The verb field is as wide as the longest verb on that same board, and a shorter verb is padded with spaces immediately before the colon. The explain **MUST** be drawn after exactly one space. Those widths are display columns. East Asian Wide and Fullwidth count as two columns. Ambiguous stays one column. The next write on that row starts at the display column after the text already placed. A wide character that does not fit is dropped whole. This board uses its own widths. rust3d’s front board is the one in Implementation Notes: **1** model, **4** system-log, **6** self-management, **7** language, **9** Exit. Row **2** is omitted. **MUST NOT** renumber 4, 6, 7, or 9 to close the gap. The front board **MUST NOT** list hello, join, or list-videos. Row **4** **MUST** open the system-log board. That board **MUST** list **41** view-log, **42** clear-log, **43** log-folder, and **0** Back. Row **7** **MUST** open the language board. The codes and the words stay on `requirement-rust-cli-language`. `language` is not an argv verb. Row **6** **MUST** open the self-management board. That board **MUST** list **62** version, **63** about, **64** version-check, **65** self-update, **66** self-uninstall, **67** self-install, and **0** Back. The children of row 6 are claimed, so the front board **MUST** print row 6. `help` **MUST** stay off every numbered row. The front board **MUST NOT** list version, about, version-check, self-update, self-uninstall, self-install, view-log, clear-log, log-folder, or list-videos. **MUST NOT** use row **61**.
3. **Three-row frame plus status line.** The bottom input box **MUST** be exactly three terminal rows high: a top border, one inner row, and a bottom border. The status line **MUST** be the next row, and that row **MUST** be the last row of the screen. The frame **MUST** span the full width of the screen. The left border is the first column. The right border is the last column the screen writer can place. There **MUST** be no blank column before the left border or after the right border.
4. **UTF-8 frame.** Those three rows **MUST** be drawn with these UTF-8 box-drawing characters. Each one occupies one column. ASCII `+`, `-`, and `|` **MUST NOT** stand in for them. Sharp corners `┌` `┐` `└` `┘` **MUST NOT** stand in for the arcs.

| Row of the box | Characters, left to right |
|----------------|---------------------------|
| Top | `╭` (U+256D), then `─` (U+2500) repeated, then `╮` (U+256E) |
| Inner | `│` (U+2502), then the input field, then `│` (U+2502) |
| Bottom | `╰` (U+2570), then `─` (U+2500) repeated, then `╯` (U+256F) |

5. **Input inside the frame.** One space, the mark `> `, the typed characters, and the caret **MUST** sit on the only inner line, strictly between the two `│` characters. The mark, the typed characters, and the caret **MUST NOT** sit on the top border, the bottom border, or a menu row. When the box is focused, the caret **MUST** be the block `█` (U+2588) at the caret index on that inner line. When the list is focused, that block **MUST NOT** be drawn. Up and Down **MUST** walk the numbered rows and the box: Down on the last row enters the box, Up on the first row enters the box, Up from the box returns to the last row, and Down from the box returns to the first row. Enter **MUST** run the typed token, or the highlighted row when the box is empty. The box **MUST NOT** be a `Choice:` line.
6. **This board’s actions.** **model** (**1**) **MUST** open the folder board: **1** `current` (`pick:.`), then one row for each immediate non-hidden subfolder (`pick:` plus that name), then **0** back. Choosing a folder **MUST** run the same conversion as typed `model`. Before that conversion, when `requirement-domain-rust3d` says a process is about to start, the screen **MUST** paint a page titled `working` whose body is that one sentence, refresh, and **MUST NOT** wait for a key. The choice token is `current` when the pick is `.`, otherwise the child directory name. There is no download sentence. Opening this folder board does not paint it. Back, Esc, language, Exit, version, about, and the cargo rows do not paint it. The conversion lines **MUST** be the result page. Those lines may start with the sentence. The screen **MUST NOT** paint the working page a second time as the result. There is no second prompt. **0** and Esc **MUST** return to the front board and **MUST NOT** convert. The typed verb `rust3d model` **MUST NOT** draw this screen. **self-management** (**6**) **MUST** open that board and **MUST NOT** leave the program. On that board, **version** (**62**) **MUST** show the installed `VERSION` and **MUST NOT** call cargo. **about** (**63**) **MUST** show the result page. The page body is `requirement-rust-about`: identity, the host check, and the star box. The domain sentence stays on pillar D of `requirement-domain-rust3d`. That page **MUST NOT** name `py-tui`. When the page is longer than the screen, Up and Down scroll it. The footer says `Up/Down scrolls this page.` and `Press a key to return to the main menu.` **version-check** (**64**) **MUST** run `cargo index versions rust3d`. **self-update** (**65**) **MUST** run `cargo install --upgrade rust3d`. **self-uninstall** (**66**) **MUST** run `cargo uninstall -y rust3d`. Choosing that row is the confirmation. **self-install** (**67**) **MUST** run `cargo install rust3d`. Those cargo commands **MUST NOT** use `sudo` and **MUST NOT** use `curl`. **0** **MUST** return to the front board. **system-log** (**4**) **MUST** open that board and **MUST NOT** leave the program. **view-log** (**41**) **MUST** list the `.log` file names in the log folder and **MUST** show the chosen file on a result page. The person picks the number in the bottom box. That list **MUST** stay until the person chooses a number or presses Esc. The one-second clock wait from rule 13 **MUST NOT** close it. A pause longer than one second **MUST** keep the characters already typed. The result page **MUST** be the file name, a blank line, then the file text. An empty file **MUST** show `(empty)` in place of that text. Esc **MUST** return to the system-log board and **MUST NOT** show the file. When the folder has no `.log` file, the result page **MUST** be `No log file in <folder>.` **clear-log** (**42**) **MUST** list those same names, then ask `Clear <name>? (y/n)` in the bottom box. That list and that question **MUST** stay until the person answers or presses Esc. The one-second clock wait **MUST NOT** close them. Yes **MUST** empty that file and **MUST NOT** delete it. The result page **MUST** be `Cleared <name>.` No, Esc, or Enter **MUST** leave the file unchanged and **MUST NOT** show a result page. **log-folder** (**43**) **MUST** show `Log folder: <path>`, where `<path>` is the absolute path `logDir()` returns. When no logger is present, that page **MUST** be `No log folder.` This row **MUST NOT** type a folder ladder and **MUST NOT** create a directory. Rows 41, 42, and 43 are menu actions. They are not product verbs. Typing the short name on an open screen **MUST** run that row. Before the read and before the empty, the program **MUST** call `log_message` with the operation and the path, component `menu`, level INFO (`requirement-rust-cli-logging`). The read line **MUST** be `read log path=<path>`. The empty line **MUST** be `clear log path=<path>`. The call **MUST NOT** sit inside `isDebug()`. A chosen path **MUST** be a regular `.log` file whose parent, after resolve, is that log folder. **Exit** (**9**) **MUST** leave the program and return 0. It **MUST NOT** ask `Exit? (y/n)`. An unknown token **MUST** stay on this board and show an error line in the menu region above the box. It **MUST NOT** exit the process. A typed product verb (`requirement-rust-cli-interface`) **MUST** enter that action without a front-board pick. Choosing row 1 **MUST** open the folder board. `about` **MUST** open its result page. `rust3d model` **MUST NOT** draw this screen. `version` **MUST** print the installed version and **MUST NOT** call cargo and **MUST NOT** draw this screen. `version-check`, `self-update`, and `self-install` **MUST** run the cargo command above and **MUST NOT** open this screen. `self-uninstall` on the command line **MUST** require `--force` and **MUST NOT** open this screen. The command `help` **MUST NOT** draw this screen. Typing `help` on an open screen **MUST** show a result page and **MUST NOT** add a numbered help row. Empty argv on a terminal **MUST** show the front board and **MUST NOT** run self-install or self-update and **MUST NOT** convert a folder by itself.
7. **Status line and errors.** The status line under the frame **MUST** show the product name, the version from `VERSION`, the board title, and the key hint `Up/Down  •  Enter`, separated by `│`. It **MUST NOT** replace one of the three frame rows. An error line, when shown, **MUST** sit in the menu region above the box.
8. **Result page.** After about, after a model build, after a log action, or after another result page, the result page may omit the menu rows, the box, and the status line until the next key returns to the previous board. When the body is longer than the room under the title, Up and Down scroll that body and the footer says `Up/Down scrolls this page.` A one-line result still closes on Down.
9. **Too small or no terminal.** If the screen cannot hold the path line from rule 13 (a path shortened to fit still counts), at least one numbered menu row, the three-row box, and the status line, or cannot hold both corners plus the mark and one cell between them, the program **MUST NOT** collapse the box into one unframed row and **MUST NOT** fall back to a `Choice:` line. It **MUST** fail closed: non-zero, an operator-readable line, and the next step `rust3d help`. If stdout is not a terminal and argv is empty, the program **MUST** print help and return 0 (`requirement-rust-cli-interface`). It **MUST NOT** draw this screen and **MUST NOT** wait. `hello`, `join`, and `list-videos` are not verbs. An unknown verb **MUST** exit non-zero.
10. **One writer.** The screen writer **MUST NOT** become a second logger. A run that is not a text-screen verb **MUST NOT** open this screen. The folder board **MUST** be drawn by struct `Tui` calling struct `MenuPainter`. It **MUST NOT** be a `print` / `input()` line that closes the screen.
11. Actor / role / subject / approver: **considered**. No dest machine. No approver. The table stays on `requirement-class-software-dev.md`. This file does **not** add an actor requirement.
12. Dest fence conditions: **considered — none**. Do not invent one.
13. **Path line.** The first row of the menu region, on the front board, on the self-management board, on the system-log board, and on the language board, **MUST** be the path line. The shape **MUST** be `<label>: <current path>`, with one space after the colon. `<current path>` **MUST** be the absolute current working directory at the moment that board is painted. The path **MUST NOT** be translated. The product name and the package version **MUST NOT** appear on this row. They stay on the status line (rule 7). The label is one translatable message. `requirement-rust-cli-language` selects the language and owns the word. English, the default, is `Path`. Traditional Chinese is `路徑`. This file does not detect the language, and it does not invent a second line shape. The path line is also the first row of the language board. When the path text is wider than the screen, the writer **MUST** keep the path label, the colon, and the space, and **MUST** shorten the path from the left so that field fits on one row. A wide character that does not fit is dropped whole. The screen **MUST** still open.

The same row **MUST** carry a clock on the right when the row has room. The clock **MUST** be the local time of this machine at the moment that row is drawn. The shape **MUST** be `HH:MM:SS`: hours, minutes, and seconds, each two digits, 24-hour, zero-padded, separated by colons. Eight characters. No date. No timezone name. No label before the clock. The clock **MUST NOT** be translated. At least two spaces **MUST** separate the path text from the clock. When both fields fit with those two spaces, the path field **MUST** stay at the left and the clock **MUST** end on the last column the writer can place. That fit, that width, and that last column are display columns. East Asian Wide and Fullwidth count as two. Ambiguous stays one. When both do not fit, the writer **MUST** keep the path field, including the shorten-from-the-left rule above, and **MUST NOT** draw the clock. When no width is known, the row **MUST** be the path field, then two spaces, then the clock. The writer **MUST NOT** read `USER`, `USERNAME`, or `getpass` for this row, and **MUST NOT** run `id`.

While the front board, the self-management board, the system-log board, or the language board is showing, the session **MUST** wait at most one second for the next key. When that wait ends and no key arrived, the session **MUST** draw this row again with the local time at that moment and **MUST** wait again. That wait **MUST NOT** leave the program, **MUST NOT** change the highlighted row, the typed buffer, or the board, and **MUST NOT** drop a key that arrived during the wait. The session **MUST NOT** start a thread to move the clock. A screen that cannot arm this wait keeps the key read it already had. A result page, a notice that waits for a key, the view-log file list, the clear-log file list, and the clear confirm keep the product title on row 0. They do not draw this clock, and they do not use this one-second wait. The folder board is not a result page. `paint` draws the path line on it. `wait_key` does not arm the one-second wait for layer `folders`, so that clock does not tick until the session returns to one of the four boards above. Before each key read on those questions, the session **MUST** clear that wait. A no-key from the one-second wait **MUST NOT** be read as Esc there. The question **MUST** stay until the person answers or presses Esc. A pause longer than one second **MUST** keep the characters already typed. The clock redraw **MUST NOT** be a log line. The clock wait is not Esc and is not row 9 Exit.

14. **Control-C is not row 9.** This file does not own Control-C. Row **9** Exit leaves with no confirm question and returns 0. A clock redraw **MUST NOT** leave the program.

Picture of this board. The box in the picture is 16 columns wide so the corners stay visible. On a real screen those 16 columns become the full width, and the menu stays above the frame. The block in the picture is the focused caret. `14:05:09` is a sample local time. `1.0.1` is the current package version and moves with `VERSION`. The explain on row 1 follows the menu language. This picture is English.

```text
Path: /tmp/clips                                              14:05:09

1. model          : build a 3D model from outline images in a chosen folder
4. system-log     : view, clear, and the log folder
6. self-management: version, about, and cargo lifecycle
7. language       : display language for this menu
9. Exit           : leave
╭──────────────╮
│ > █          │
╰──────────────╯
  rust3d 1.0.1  │  main menu  │  Up/Down  •  Enter
```

Folder board after row 1, when the current directory has subfolders `alpha` and `beta`:

```text
1. current : convert images in this folder
2. alpha   : convert images in this subfolder
3. beta    : convert images in this subfolder
0. Back    : return to the main menu
```

The same first row when `requirement-rust-cli-language` has selected Traditional Chinese:

```text
路徑: /tmp/clips                                              14:05:09
```

The spaces in those pictures stand for the gap that grows on a wider screen. The clock ends on the last column the writer can place. When the row cannot hold both fields, the picture of that row is the path field alone.

### Sample code

The front board, the self-management board, the system-log board, and the language board arm a one-second wait in `MenuSession.wait_key`. A question and a notice clear that wait before the key read. A no-key from the clock is not Esc. `TP-TUI-09` does not prove this wait. `TP-TUI-01` through `TP-TUI-08` stay todo.

```python
# Boards that draw the clock. A no-key redraws the clock. It is not Esc.
screen.timeout(1000)
key = screen.getch()

# Result pages, notices, and file lists. Clear the one-second wait first.
screen.timeout(-1)
key = screen.getch()
```

### 2.1 Implementation Notes (this project)

| Field | Value |
|-------|--------|
| **Command** | `rust3d` |
| **Open the screen** | `rust3d` or `cargo run` on a terminal, with no verb |
| **Direct verb** | `rust3d model` converts and does not draw this screen. `rust3d about` opens the result page. `help`, `version`, and the cargo verbs do not draw this screen |
| **Session** | Struct `Tui` in `src/menu.rs`. The live terminal is struct `TermScreen` in that same file. It opens the alternate screen with `crossterm` |
| **Painter** | Struct `MenuPainter` in `src/menu.rs`. Stores `Path` and `路徑`. `path_label` returns the word `requirement-rust-cli-language` selected. English is the default. `cli.rs` calls `paint` and does not draw the frame |
| **Caller** | Struct `Cli` in `src/cli.rs`. `fn main` stays in that file. Empty argv on a terminal opens struct `Tui` |
| **Front rows** | **1** `model` — build a 3D model from outline images in a chosen folder; **4** `system-log` — view, clear, and the log folder; **6** `self-management` — version, about, and cargo lifecycle; **7** `language` — display language for this menu; **9** `Exit` — leave. Row 2 is omitted |
| **System-log rows** | **41** `view-log`; **42** `clear-log`; **43** `log-folder`; **0** `Back`. These rows are not product verbs |
| **Language rows** | **71**–**83** and **0** Back. Codes, the leaf, and the words are `requirement-rust-cli-language` |
| **Self-management rows** | **62** `version`; **63** `about`; **64** `version-check` (`cargo index versions rust3d`); **65** `self-update` (`cargo install --upgrade rust3d`); **66** `self-uninstall` (`cargo uninstall -y rust3d`); **67** `self-install` (`cargo install rust3d`); **0** `Back`. `help` is typed, not numbered. No row 61 |
| **model** | Folder board: **1** current (`pick:.`), numbered immediate subfolders, **0** back. A pick paints the waiting page titled `working`, then converts that folder to `model.glb` and `viewer.html` in its `model` directory and shows the result page. Esc returns to the front board |
| **about** | Row **63**. Result page from `requirement-rust-about`, composed by struct `AboutPage`. The domain sentence stays on pillar D. Omits the frame. Does not name `py-tui`. Up and Down scroll when the page is longer than the screen |
| **Box height** | 3 terminal rows, then one status row |
| **Box width** | Full width the writer can place. Left border at column 0 |
| **Mark inside the frame** | one space, then `> ` |
| **Caret when the box is focused** | `█` U+2588 on the inner row |
| **Top line** | Left: `<label>: <absolute current working directory>`. Right, when the row has room: local time `HH:MM:SS`. Same first row on the front board, the self-management board, the system-log board, and the language board. Product name and version stay on the status line |
| **Path label** | English `Path` by default. Traditional Chinese `路徑` when the language requirement selects `zh-Hant`. The path value is not translated. This file does not detect the language |
| **Worked Traditional Chinese top line** | `路徑: /tmp/clips` then a gap, then `14:05:09` flush right |
| **Clock wait** | One second on those four boards. No thread. Questions and file lists do not use this wait. A no-key is not Esc and is not Exit |
| **Status line** | `  rust3d <version>  │  main menu  │  Up/Down  •  Enter` |
| **External menu package** | Not imported and not declared |
| **No terminal, empty argv** | Help text, return 0 |
| **No terminal, model** | Builds without this screen. A missing folder returns 1 |
| **Too small** | Non-zero. Next step `rust3d help` |
| **Privilege** | normal user. Cargo rows do not use sudo |
| **Proof** | `TP-TUI-09` and `TP-TUI-10` have. `TP-TUI-01` through `TP-TUI-08` stay todo |

16-column inner line, focused and empty: `│ > █          │`

Invocation samples for the rows this file owns:

```text
rust3d
7
71
```

```text
rust3d
6
62
```

```text
rust3d version
rust3d version-check
rust3d self-update
rust3d self-install
rust3d self-uninstall --force
```

`rust3d` on a terminal is the front board. `7` then `71` opens English on the language board. `6` then `62` is version and does not call cargo. The typed cargo lines do not open this screen.

### 2.2 Why This Requirement Exists (Direct CIAO Alignment)

- **CIAO Principle 2 – Intentional**: The menu and the box are two named regions. The session is `Tui`. The frame is `MenuPainter`. The first row keeps the folder on the left and the local clock on the right.
- **CIAO Principle 16 – Interactive**: A process with no terminal never draws the screen, so it cannot wait inside the box. Empty argv in that case prints help and returns 0.
- **CIAO Principle 1 – Caution**: A screen too small to hold the frame fails closed. Esc on the folder board does not convert.
- **CIAO Principle 5 – SSOT**: This file owns the picture. Verb names also live on `requirement-rust-cli-interface`. Outline facts live on `requirement-domain-rust3d`.
- **CIAO Principle 10 – Least privilege**: Cargo rows use `cargo` as the normal user.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** The frame is three rows plus the status line, or the screen does not open.
- **Intentional:** Typing happens on the only inner line. The front rows are model, system-log, self-management, language, and Exit.
- **Anti-fragile:** The right border uses the last column the writer can place, and the left border stays on column 0.
- **Over-protect:** Do not replace the frame with a single underlined row, and do not fork the painter into `cli.rs`.

## 4. Protection Rule (Sacred)

**Future AI assistants or maintainers MUST NOT:**

- Collapse the bottom input box to one row.
- Draw the frame with ASCII `+`, `-`, or `|`, or with sharp corners `┌` `┐` `└` `┘`.
- Put the typed text, the caret, or `> ` on a border line or on a menu row.
- Inset the box so a blank column remains on the left or the right.
- Draw the numbered menu rows inside the three-row frame.
- Draw this menu with `print` / `read` / `input()` or a `Choice:` line.
- Close the screen when **model** is chosen and continue with `input()`.
- Convert a folder before the person picks one on the folder board.
- Copy the frame glyphs into `cli.rs`.
- Declare `py-tui`, or import `py_tui`.
- Renumber the front board to fill the omitted row 2, or place this package on row 61.
- Hide row 6 while rows 62–67 are claimed.
- Run cargo from empty argv, or run cargo with `sudo` or `curl`.
- Treat the one-second clock wait as Esc or as row 9 Exit.
- Translate the folder path or the clock. The path word belongs to `requirement-rust-cli-language`. English is `Path`. Traditional Chinese is `路徑`.
- Count a menu column with `len()` when the text can contain an East Asian Wide or Fullwidth character. The next write starts at the display column after the cells already drawn. Wide and Fullwidth count as two. Ambiguous stays one. A wide character that does not fit is dropped whole.
- Detect a language in this file. `requirement-rust-cli-language` selects the language.
- Add `Exit? (y/n)` to row 9, or invent a Control-C protocol in this file.
- Mark `TP-TUI-*` have while `src/` still uses `input()`.
- Start a folder conversion without the domain waiting sentence, paint that working page again as the result, or paint a download sentence when the weights file is already a file.

**Violating this rule is a critical menu regression.**

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, the person runs `rust3d` as the normal user. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to open the menu or to run rows 64–67. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`. Cargo rows call `cargo` only.

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | Empty argv on a terminal opens the front board and does not convert a folder and does not run cargo |
| AC-2 | Front rows are 1 model, 4 system-log, 6 self-management, 7 language, 9 Exit. Row 2 is absent and the others are not renumbered |
| AC-3 | Self-management lists 62–67 and 0 Back. version does not call cargo. Cargo rows do not use sudo or curl |
| AC-4 | System-log lists 41, 42, 43, and 0 Back. Those rows are not product verbs |
| AC-5 | Join questions stay in the bottom box, current folder only, Esc returns to the front board |
| AC-6 | Empty argv with no terminal prints help and returns 0. Typed `model` converts and does not draw this screen |
| AC-7 | The frame uses the arc glyphs. `cli.rs` does not contain them. `py_tui` is not imported |
| AC-8 | The default path label is `Path`. The painter stores `路徑`. `requirement-rust-cli-language` selects the word. The path and the clock are not translated |
| AC-9 | A wide short stays whole. The colon starts at the display column after that short. A wide path label keeps the clock on that row when both fields fit |
| AC-10 | A folder pick paints the domain waiting sentence on a page titled `working` and refreshes before the conversion. Opening the folder board does not. The result page does not paint that working page again |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-cli-interface` | Typed verbs and empty-argv contract |
| `requirement-domain-rust3d` | Outline steps, the folder board, about facts, and the waiting sentences |
| `requirement-rust-cli-language` | Menu language, front row 7, and the path word |
| `requirement-rust-oop` | Struct homes for `Tui`, `MenuPainter`, `MenuSession`, `MenuLanguage` |
| `requirement-rust-cli-logging` | Quiet mirror. view-log and clear-log lines |
| `requirement-rust-error-handling` | Fail-closed sentences |
| `requirement-video-ffmpeg-pipeline` | Concat after the questions succeed |
| `requirement-rust-packaging` | Package name `rust3d` on the cargo rows |
| `requirement-class-software-dev` | Actor residual. No new actor file |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-TUI-01** | `tests/test_menu.rs` | todo | Empty argv on a tty opens the front board. It does not convert a folder and does not run cargo |
| **TP-TUI-02** | `tests/test_menu.rs` | todo | Empty argv off a tty prints help and returns 0 |
| **TP-TUI-03** | `tests/test_menu.rs` | todo | Join asks for two indexes and an output name in the bottom box. No folder prompt. Esc returns to the front board |
| **TP-TUI-04** | `tests/test_menu.rs` | todo | Row 6 lists 62–67 and 0 Back. version does not call cargo. Cargo rows use `cargo` and do not use sudo or curl |
| **TP-TUI-05** | `tests/test_menu.rs` | todo | Rows 41, 42, and 43. The clock wait does not close a question or a file list |
| **TP-TUI-06** | `tests/test_menu.rs` | todo | No `print` / `input()` / `Choice:` menu. No `py_tui`. Frame glyphs are absent from `cli.rs` |
| **TP-TUI-07** | `tests/test_model.rs` | have | Typed `model` does not open the text screen |
| **TP-TUI-08** | `tests/test_menu.rs` | todo | Path line and clock on the four boards. No thread. A no-key is not Esc and is not Exit |
| **TP-TUI-09** | `tests/test_menu.rs` | have | With no language file, the label is `Path`. `路徑` is stored. `LANG` does not switch the label. The path and the clock are not translated |
| **TP-TUI-10** | `tests/test_menu.rs` | have | Wide and Fullwidth count as two columns. The colon starts after that width. The four wide path labels keep the clock on the row |
| **TP-TUI-11** | `tests/test_model.rs` | have | A folder pick paints the waiting sentence before the session. Front row 1 does not |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Default text-screen style

**Definition:** **Default TUI style** is the portable **full-screen look** of a claimed cargo Rust CLI default interaction. One screen writer draws **two regions** on the front board and on the self-management board: a **menu region** and a **bottom input box** pinned to the bottom of the screen. The first row of the menu region is the **path line**: `<label>: <absolute current working directory>`, with one space after the colon. That path is not translated. `rust3d` and `the live product version` are not on that row. They stay on the **status line** under the box, as plain text with a space between the name and the version. When the row has room, a **local clock** `HH:MM:SS` is flush right: 24-hour, zero-padded, eight characters, no date, no timezone name, no label, and not translated. At least two spaces separate the path from the clock. A row that cannot hold both keeps the path (shortened from the left) and omits the clock. Numbered rows follow the path line. The box is a rounded UTF-8 frame **exactly three terminal rows** high (top border, one inner typing line, bottom border) plus that status line on the next row, and that status row is the last row of the screen. Each numbered row is **number**, **verb**, and **explain**, aligned with **that board’s own widths**. Ink on the verb and the explain stays default CLI main menu style (bold verb, italic explain), drawn by the screen writer. While those two boards are showing, the session waits at most one second for the next key. No key redraws the clock and stays on the board. A typed key is still delivered. No thread. The redraw is not a log line. A screen that cannot arm the wait keeps the key read it already had. A result page and an edit question keep the product title on row 0 in the parenthetical app-name-version-display form, do not draw the clock, and do not use that wait. This term owns the **frame**, the **path line**, the **clock**, and the **column alignment**. It does not own which numbers are legal, Exit, or empty-argv cases. It does not name a class.

**Human daily-life explanation:**

**Default TUI style** is the chalkboard that fills the window. The top line names the folder you are standing in, and a small clock of the local time sits on the right of that same line and moves once a second. The dishes sit in straight columns under that line, and a rounded writing tray is fixed to the bottom edge. The shop name and the edition sit on the thin strip under the tray, not on the top line. Shorter dish numbers get blank space on the left, and shorter dish names get blank space just before the colon, so the digits line up, the colons line up, and the note after the colon starts one space later.

**Daily-life example:** On the lunch board the top line reads `Path: the absolute working directory` and the clock `14:05:09` ends at the right edge. A second later the clock shows the new local time and the dishes have not moved. Dish `11` and dish `0` share a two-digit number column (`11` beside ` 0`). You write the number in the tray. You do not answer a line that says `Choice:`. A result slip and a question slip still put the shop name on their first line and do not show the clock.

### Text-screen wide column

**Definition:** **TUI wide column** is the rule that a text-menu writer measures a cell in **display columns**, not in code points. A character whose East Asian width is **Wide (W)** or **Fullwidth (F)** occupies **two** columns. Every other character, including **Ambiguous**, occupies **one** column. The verb pad, the column where the colon is written, and the path row all use that width. A later write on the same row starts at or after the display column of every cell already drawn. It does not start on the continuation cell of a wide character. A wide character that does not fit is dropped whole. This term does not choose which rows exist, which words are stored, or which integers are legal.

**Human daily-life explanation:**

A **tui-wide-column** is the shop rule for how wide each dish name is on the chalkboard. Some letters take one slot. A character that is Wide or Fullwidth takes two slots. The colon that starts the note is placed after those slots, so the note does not paint over the second half of a wide name. Counting the letters and ignoring the slots leaves the colon in the middle of the name, and the rest of the name disappears.

**Daily-life example:** The name `语言` is two letters and four slots. If the colon is placed two slots in, the second letter is erased and the board shows `语`. The same erasure hits `語言`, `언어`, and `終了`. Box-drawing corners and the block caret stay one slot each, because they are Ambiguous.

### Major text-screen entry

**Definition:** **Major TUI entry point** is the rule that the major entry of a claimed text screen is interactive mode with **no positional command token**. Flags that are not a command (`--debug`, `--quiet`, `--force` with no command, and peers of that kind) stay on that entry. `--json` with no command stays JSON help, not the text screen. Creating or keeping a text screen does not attach every CLI verb to that screen. A verb opens the screen only when a specialized requirement explicitly names that verb as using the text screen. This term does not choose the look of the board, the width of a glyph, or which tokens the dispatcher accepts.

**Human daily-life explanation:**

A **major-tui-entry-point** is the walk-up board at a shop. You see that board when you name no dish. Naming a dish sends you to a different counter. A light switch that is not a dish (`--debug` and the same kind of flag) still leaves you at the walk-up board. Asking for the paper list with no dish (`--json` alone) is the paper list, not the board.

**Daily-life example:** You walk in and name no dish. The **major-tui-entry-point** is that board. You walk in and say only "more light." You are still at the **major-tui-entry-point**. You name a dish. That order is not the **major-tui-entry-point**. It stays at the board only when the card already names that dish for the board. Every other named dish goes to the plain counter.

### Python CLI verb default route

**Definition:** **Python CLI verb default route** is the rule that, for a Python CLI, each routed verb's default route is the **terminal**: standard output, standard error, a line interface, and a scriptable run, with no text screen. A verb uses the text screen only when a specialized requirement explicitly names that verb as using the screen. The default exists so a command-line test can drive the verb without a screen. This term does not choose which tokens the dispatcher accepts. It does not move the no-command interactive entry off the text screen.

**Human daily-life explanation:**

A **python-cli-verb-default-route** is the shop rule for a named dish. Naming a dish sends the order to the plain counter, where a written slip can be checked without standing at the walk-up board. The board is still there when you name no dish. A dish stays at the board only when the card already says that dish is served there.

**Daily-life example:** You write one dish name on a slip. The **python-cli-verb-default-route** is the plain counter, so a checker can read the slip without the board. The card already says one named dish is served at the board. That dish stays at the board. Every other named dish stays on the **python-cli-verb-default-route**.

### App-name-version display

**Definition:** **App-name-version-display** is the operator-facing **identity token** printed when a CLI **names the running program** on a human banner: live Config `APP_NAME` immediately followed by parenthesized live Config `VERSION`, with **no space**. The name is **bold**; the version (edition string; test helpers may bind it as `APP_VERSION`) is *italic*. Markdown written form: `**rust3d**(*the live product version*)`. It is **not** README H1 app-name, **not** a filesystem path, **not** an invocation/`Next:` command line, and **not** JSON (those keep separate `app` / `version` keys). Live scalars stay Config; this term owns the **print form** and **when** that form is required.

**Human daily-life explanation:**

**App-name-version-display** is the nametag on the program when it introduces itself: name and edition together, so you do not have to run a second “what version?” question to know which copy is talking.

**Daily-life example:** A café nametag that prints the shop name in **bold** and the edition in *italics* — **Harbor Kettle**(*2024*) — not only “Harbor Kettle,” and not a street address or the order slip that still says “Harbor Kettle, table 3.”

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-04 | Active 1.0.0 | Text menu for rust3d. Front board is join, system-log, self-management, Exit. Ship unit still uses `input()` |
| 2026-10-04 | Active 1.1.0 | Path label words are English `Path` and Traditional Chinese `路徑`. English stays in force until a multi-language environment requirement is Active. `TP-TUI-09` has |
| 2026-10-04 | Active 1.1.1 | Purpose, sample, and implementation notes match the session on disk. `TP-TUI-01` through `TP-TUI-08` stay todo |
| 2026-10-04 | Active 1.2.0 | Front row 4 is language. System-log moves to row 3 and children 31–33. The language requirement selects the path word. `TP-TUI-01` through `TP-TUI-08` stay todo |
| 2026-10-04 | Active 1.2.1 | Menu columns are display columns. Wide and Fullwidth count as two. Ambiguous stays one. `TP-TUI-10` has. `./tests/run.sh`: 6 tests, OK |
| 2026-10-04 | Active 1.2.2 | Row 83 shows `requirement-rust-about`. A long result page scrolls. Product version in the sample is **1.0.5** |
| 2026-10-05 | Active 1.2.3 | A folder pick paints a page titled `working` with the domain waiting sentence before the conversion. Opening the folder board does not |
| 2026-10-05 | Active 1.2.4 | The sample status line shows package **1.0.1** |
| 2026-10-05 | Active 1.2.5 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.2.6 | The live menu is the full-screen session. `TermScreen` in `src/menu.rs` uses `crossterm`. The line prompt is gone. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.2.7 | Self-management is front row 6, children 62–67. Language is front row 7, children 71–83. Row 2 stays omitted. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.2.8 | System-log is front row 4. Children are 41 view-log, 42 clear-log, and 43 log-folder. Row 2 stays omitted. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
