# rust3d - A glTF model and an HTML viewer from outline images

![Version](https://img.shields.io/badge/Version-1.0.1-blue?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)
[![CIAO](https://img.shields.io/badge/Philosophy-CIAO%20(Caution%20%E2%80%A2%20Intentional%20%E2%80%A2%20Anti--fragile%20%E2%80%A2%20Over--engineered)-purple.svg)](https://github.com/cloudgen/ciao)
[![Stars](https://img.shields.io/github/stars/cloudgen/rust3d?style=flat-square)](https://github.com/cloudgen/rust3d)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?style=flat-square)]()
[![crates.io](https://img.shields.io/crates/v/rust3d?style=flat-square)](https://crates.io/crates/rust3d)

rust3d builds one glTF model and one HTML viewer from outline images in a folder. The files are `model.glb` and `viewer.html`. Supported inputs are webp, png, jpg, and jpeg.

On a terminal, starting it with no arguments opens a text menu. The menu does not convert images by itself, and it does not call cargo.

The sample conversion is [`sample-images/model/viewer.html`](https://cloudgen.github.io/rust3d/sample-images/model/viewer.html). The picture is that page. The picture address is absolute, so the package index and GitHub both show it.

[![Converted model in the HTML viewer](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/model-viewer.png)](https://cloudgen.github.io/rust3d/sample-images/model/viewer.html)

## Features

- Text menu on a terminal: **model**, **system-log**, **self-management**, **language**, and **Exit**
- The first row shows the current folder and a local clock (`HH:MM:SS`) when the row has room
- Numbered rows line up. A rounded input box sits along the bottom, as wide as the terminal, with the name and version on the status line under the box
- **model** (1) lists **1** current folder, each subfolder, and **0** back. The chosen folder becomes one 3D model
- The typed verb `model` builds that model in the terminal and does not draw the menu. With no folder, it uses the current directory
- **system-log** (3) opens view-log, clear-log, and log-folder. **self-management** (6) opens version, about, and the cargo lifecycle rows. **language** (7) picks the menu language
- One binary, `rust3d`. `cargo run` starts it
- Typed verbs: `help`, `version`, `about`, `model`, `self-install`, `version-check`, `self-update`, `self-uninstall`

## Advantages

1. **A folder of outlines in, one model out.** `rust3d model` builds a model from outline images in the current directory and does not draw the menu. On the menu, row **1 model** asks for the current folder or a subfolder, then writes `model.glb` and `viewer.html` in that folder's `model` directory. With no arguments on a terminal, the text menu opens and waits. With no terminal and no verb, the program prints help and returns 0. There is no `--json`.
2. **Built-in languages.** Row **7** lists thirteen languages: English, Simplified Chinese, Traditional Chinese, Spanish, Arabic, French, Portuguese, Russian, German, Japanese, Korean, Dutch, and Greek. The choice is saved for the next run. The note on row 1 follows the menu language.
3. **Lifecycle and diagnostics.** `version-check`, `self-update`, `self-install`, and `self-uninstall` are menu rows **64**–**67** and typed verbs. `self-uninstall` on the command line needs `--force`. They call cargo and do not use root. **system-log** (**3**) views a log, clears a log, and shows the log folder. **about** (**63**) stays in English.

## Quick Installation

**Rust dependencies:** `chrono>=0.4`, `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2` (required). Cargo installs them with rust3d. The inputs are outline images already drawn. No model download and no external media tool are required.

### crates.io (registry)

```bash
cargo install rust3d
```

This installs the binary **`rust3d`**. Packaging name SSOT is `Cargo.toml` `package.name` = `rust3d`. This tree is **1.0.1**. The crates.io badge above shows the live index.

### Local install (checkout)

From a checkout (editable / unreleased work):

```bash
git clone https://github.com/cloudgen/rust3d.git
cd rust3d
cargo build --release
cargo install --path .
```

### Main menu

After install, on a terminal, `rust3d` with no arguments prints this screen. The verb is bold and the note after the colon is italic. `9` leaves. `0` on a submenu goes back. After a command finishes, the main menu is shown again. A number that is not on the list prints an error and lets you choose again.

Picture of the English menu. Current directory `/tmp/clips`. The rows are the text the menu paints. The frame is the 80-column box.

```text
$ rust3d
Path: /tmp/clips                                                       12:06:45

1. **model**          : *build a 3D model from outline images in a chosen folder*
3. **system-log**     : *view, clear, and the log folder*
6. **self-management**: *version, about, and cargo lifecycle*
7. **language**       : *display language for this menu*
9. **Exit**           : *leave*




╭─────────────────────────────────────────────────────────────────────────────╮
│ >                                                                           │
╰─────────────────────────────────────────────────────────────────────────────╯
  rust3d 1.0.1  │  main menu  │  Up/Down  •  Enter
```

**system-log** (3):

```text
Path: /tmp/clips                                                       12:06:45

31. **view-log**  : *list a log file and show it*
32. **clear-log** : *empty one log file*
33. **log-folder**: *show the log folder*
 0. **Back**      : *return to the main menu*




╭─────────────────────────────────────────────────────────────────────────────╮
│ >                                                                           │
╰─────────────────────────────────────────────────────────────────────────────╯
  rust3d 1.0.1  │  system-log  │  Up/Down  •  Enter
```

**self-management** (6):

```text
Path: /tmp/clips                                                       14:05:09

62. **version**       : *show the installed version*
63. **about**         : *version and this computer*
64. **version-check** : *compare this install with cargo*
65. **self-update**   : *upgrade this package with cargo*
66. **self-uninstall**: *remove this package with cargo*
67. **self-install**  : *install this package with cargo*
 0. **Back**          : *return to the main menu*

╭─────────────────────────────────────────────────────────────────────────────╮
│ >                                                                           │
╰─────────────────────────────────────────────────────────────────────────────╯
  rust3d 1.0.1  │  self-management  │  Up/Down  •  Enter
```

**language** (7). The short on each language row is that language’s own name. Numbers 70 and 84–89 are not printed. `0` goes back and does not save.

```text
Path: /tmp/clips                                                       12:06:46

71. **English**   : *use English for this menu*
72. **简体中文**      : *use 简体中文 for this menu*
73. **繁體中文**      : *use 繁體中文 for this menu*
74. **Español**   : *use Español for this menu*
75. **العربية**   : *use العربية for this menu*
76. **Français**  : *use Français for this menu*
77. **Português** : *use Português for this menu*
78. **Русский**   : *use Русский for this menu*
79. **Deutsch**   : *use Deutsch for this menu*
80. **日本語**       : *use 日本語 for this menu*
81. **한국어**       : *use 한국어 for this menu*
82. **Nederlands**: *use Nederlands for this menu*
83. **Ελληνικά**  : *use Ελληνικά for this menu*
 0. **Back**      : *return to the main menu*

╭─────────────────────────────────────────────────────────────────────────────╮
│ >                                                                           │
╰─────────────────────────────────────────────────────────────────────────────╯
  rust3d 1.0.1  │  language  │  Up/Down  •  Enter
```

Choose a number, or type the command name in the box. The block caret appears in that box while it is focused.

## Usage

```bash
rust3d
# or
cargo run
```

On a terminal, that opens the main menu above. It does not build a model yet, and it does not call cargo. With no terminal, the same command prints help and returns 0.

**model** (1) opens a folder list: **1** current folder, then each subfolder, and **0** back. The chosen folder is built into `model.glb` and `viewer.html`. By default those files go in that folder's `model` directory. A `views.json` in the folder, or `--views`, names each outline with azimuth and elevation in degrees. `--grid` sets the voxel count on each axis (16 to 160, default 96). Any key on the result page returns to the main menu. The typed verb builds the model in the terminal and does not draw the menu:

```bash
rust3d model
rust3d model photos --views views.json --grid 96
rust3d model --output photos/built
```

```bash
rust3d help
rust3d version
rust3d about
rust3d model
rust3d version-check
rust3d self-update
rust3d self-install
rust3d self-uninstall --force
```

`version` prints `rust3d 1.0.1` and does not call cargo. `about` shows one English page: the product identity, a host check of this computer, and a star box. It does not call cargo. `model` uses the current directory when no folder is given and does not draw the menu. `help` prints usage.

`version-check` runs `cargo search rust3d --limit 1`. `self-update` runs `cargo install rust3d --force`. `self-install` runs `cargo install rust3d`. `self-uninstall` runs `cargo uninstall rust3d` and needs `--force` on the command line. Those cargo verbs do not use sudo. Empty arguments do not install or update.

Exit codes: non-zero for an unknown verb, a missing folder, a bad views file, an empty visual hull, a cargo failure, or a menu that cannot open. Menu **Exit** (9) returns 0. `model` returns 0 when the folder exists and the model is written. An empty folder returns 0 and does not decode images.

Checkout run:

```bash
cargo run
# interactive session; a terminal opens the text menu
```

## Screenshots

Each heading is the file name. The paragraph is what that picture shows: the words on the screen, the characters in the input box, or the scene. Package **1.0.1**. These picture files were recaptured on 2026-10-14. They are captures of rust3d. Language is row **7**. Self-management is row **6**. Row 1 is **model**. The folder board is `model-folder.png`. `model-viewer.png` is the converted model in `sample-images/model/viewer.html`, and that picture links to the viewer. The package index can fetch a picture only after that file is on the public `main` branch. Every picture address is an absolute `https` URL.

### `language-menu.png`

Row **7** has opened the language list. **71 English** is highlighted, with the note "use English for this menu" The other rows are **72 简体中文**, **73 繁體中文**, **74 Español**, **75 العربية**, **76 Français**, **77 Português**, **78 Русский**, **79 Deutsch**, **80 日本語**, **81 한국어**, **82 Nederlands**, and **83 Ελληνικά**. Each note says to use that language for this menu. **0 Back** says "return to the main menu" The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `language`.

![Language list, 71 English highlighted](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/language-menu.png)

### `main-menu-en.png`

English main menu. There is no saved-language line above the box. The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "build a 3D model from outline images in a chosen folder" Then **3 system-log** "view, clear, and the log folder" **6 self-management** "version, about, and cargo lifecycle" **7 language** "display language for this menu" and **9 Exit** "leave" The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `main menu`.

![English main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-en.png)

### `main-menu-zh-hans.png`

Simplified Chinese main menu. The line above the box says `菜单语言是简体中文`. The path label is `路径`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "把所选文件夹中的轮廓图做成三维模型" **3** is `系统日志`, **6** is `自我管理`, **7** is `语言`, and **9** is `离开`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `主菜单`.

![Simplified Chinese main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-zh-hans.png)

### `main-menu-zh-hant.png`

Traditional Chinese main menu. The line above the box says `選單語言是繁體中文`. The path label is `路徑`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "把所選資料夾中的輪廓圖做成三維模型" **3** is `系統日誌`, **6** is `自我管理`, **7** is `語言`, and **9** is `離開`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `主選單`.

![Traditional Chinese main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-zh-hant.png)

### `main-menu-es.png`

Spanish main menu. The line above the box says `El idioma del menú es español`. The path label is `Ruta`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "construye un modelo 3D con los contornos de una carpeta elegida" **3** is `registro`, **6** is `autogestión`, **7** is `idioma`, and **9** is `Salir`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `menú principal`.

![Spanish main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-es.png)

### `main-menu-ar.png`

Arabic main menu. The line above the box says `لغة القائمة هي العربية`. The path label is `المسار`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The numbers stay on the left. Arabic words on each row are shaped and read right to left. **1 model** is highlighted and stays `model`: "يبني نموذجا ثلاثي الأبعاد من حدود المجلد المختار" **3** is `سجل النظام`, **6** is `إدارة ذاتية`, **7** is `لغة`, and **9** is `خروج`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `القائمة الرئيسية`.

![Arabic main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-ar.png)

### `main-menu-fr.png`

French main menu. The line above the box says `La langue du menu est le français`. The path label is `Chemin`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "construit un modèle 3D à partir des contours d'un dossier choisi" **3** is `journal`, **6** is `autogestion`, **7** is `langue`, and **9** is `Quitter`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `menu principal`.

![French main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-fr.png)

### `main-menu-pt.png`

Portuguese main menu. The line above the box says `O idioma do menu é português`. The path label is `Caminho`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "constrói um modelo 3D a partir dos contornos de uma pasta escolhida" **3** is `registo`, **6** is `autogestão`, **7** is `idioma`, and **9** is `Sair`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `menu principal`.

![Portuguese main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-pt.png)

### `main-menu-ru.png`

Russian main menu. The line above the box says `Язык меню — русский`. The path label is `Путь`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "собрать трёхмерную модель из контуров выбранной папки" **3** is `системный журнал`, **6** is `самоуправление`, **7** is `язык`, and **9** is `Выход`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `главное меню`.

![Russian main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-ru.png)

### `main-menu-de.png`

German main menu. The line above the box says `Die Menüsprache ist Deutsch`. The path label is `Pfad`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "aus Umrissen eines gewählten Ordners ein 3D-Modell bauen" **3** is `Systemprotokoll`, **6** is `Selbstverwaltung`, **7** is `Sprache`, and **9** is `Beenden`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `Hauptmenü`.

![German main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-de.png)

### `main-menu-ja.png`

Japanese main menu. The line above the box says `メニューの言語は日本語`. The path label is `パス`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "選んだフォルダの輪郭画像から3Dモデルを作る" **3** is `システムログ`, **6** is `自己管理`, **7** is `言語`, and **9** is `終了`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `メインメニュー`.

![Japanese main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-ja.png)

### `main-menu-ko.png`

Korean main menu. The line above the box says `메뉴 언어는 한국어`. The path label is `경로`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "고른 폴더의 윤곽 이미지로 3D 모델을 만듭니다" **3** is `시스템 로그`, **6** is `자기관리`, **7** is `언어`, and **9** is `종료`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `주 메뉴`.

![Korean main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-ko.png)

### `main-menu-nl.png`

Dutch main menu. The line above the box says `De menutaal is Nederlands`. The path label is `Pad`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "maak een 3D-model van contouren in een gekozen map" **3** is `systeemlog`, **6** is `zelfbeheer`, **7** is `taal`, and **9** is `Afsluiten`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `hoofdmenu`.

![Dutch main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-nl.png)

### `main-menu-el.png`

Greek main menu. The line above the box says `Η γλώσσα του μενού είναι ελληνικά`. The path label is `Διαδρομή`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "φτιάχνει ένα τρισδιάστατο μοντέλο από τα περιγράμματα ενός φακέλου" **3** is `αρχείο καταγραφής`, **6** is `αυτοδιαχείριση`, **7** is `γλώσσα`, and **9** is `Έξοδος`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `κύριο μενού`.

![Greek main menu](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/main-menu-el.png)

### `model-folder.png`

The folder board for row **1**. The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 current** is highlighted: "build a 3D model from outline images in this folder" **2 photos** says "build a 3D model from outline images in this subfolder" **0 Back** says "return to the main menu" The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `model`.

![Folder board, current folder highlighted](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/model-folder.png)

### `self-management.png`

Row **6** has opened self-management. **62 version** is highlighted: "show the installed version" Then **63 about** "version and this computer", **64 version-check** "compare this install with cargo", **65 self-update** "upgrade this package with cargo", **66 self-uninstall** "remove this package with cargo", **67 self-install** "install this package with cargo", and **0 Back** "return to the main menu" The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `self-management`.

![Self-management, 62 version highlighted](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/self-management.png)

### `tui-about.png`

**about** (63) on the result page. The title is `rust3d (1.0.1) — result`. The page prints `rust3d 1.0.1`, `Domain: Build a glTF model and an HTML viewer from outline images in a folder`, `Runtime tools: none`, and `Entry points: rust3d`. The host check is headed `2026-10-14 20:26:52.404013 rust3d (v1.0.1)  [CHECK SYSTEM]:` and `Now checking your operation system!`. The lines are `Rustc: rustc 1.89.0 (29483883e 2025-08-04)`, `C Library: GNU libc`, `Operation System: Ubuntu 24.04.5 LTS`, `Architecture: x86_64`, `Current User: operator`, `Shell: /bin/bash`, `Rustc Executable: /tmp/rust3d-tools/rustc`, `cargo location: /tmp/rust3d-tools/cargo`, `rustup location: /tmp/rust3d-tools/rustup`, `Inside docker container: false`, `Target triple: x86_64-unknown-linux-gnu`, `Binary Type: x86_64-GNU libc`, `Location: /tmp/rust3d-bin/rust3d`, `PID: 249978`, `Cache folder used: /dev/shm/cache/cache-rust3d-operator-249978`, `Cache folder (preferred): /dev/shm/cache/cache-rust3d-operator-249978`, `Cache folder (1st fallback): /tmp/cache/cache-rust3d-operator-249978`, `Cache folder (2nd fallback): /tmp/rust3d-home/.cache/cache-rust3d-249978`, `Persistence storage: /tmp/rust3d-home/.local/rust3d`, and `TTY / Interactive: yes`. The star box says `rust3d (1.0.1) by Wilgat Wong on 2026-10-11`, `You are using an UNINSTALLED version, location:`, `/tmp/rust3d-bin/rust3d`, `Basic Usage:`, `rust3d model`, and `Please visit our homepage:` with `"https://github.com/cloudgen/rust3d"`. The footer says `Press a key to return to the main menu.` There is no input box and no clock. The page stays in English.

![About host check, English](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/tui-about.png)

### `system-log.png`

Row **3** has opened system-log. **31 view-log** is highlighted: "list a log file and show it" Then **32 clear-log** "empty one log file", **33 log-folder** "show the log folder", and **0 Back** "return to the main menu" The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `system-log`.

![System log, 31 view-log highlighted](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/system-log.png)

### `model-viewer.png`

A capture on 2026-10-14 of the converted model in `sample-images/model/viewer.html`. The page background is near black. The mesh is light gray and built from stacked outline slices: a wide upper mass, an opening through the middle, and a lower mass with a flat slab. The line at the bottom left reads "rust3d — drag to orbit, wheel to zoom. model.glb is the glTF file." This is not a text-menu capture. The picture links to that viewer.

[![Converted model in the HTML viewer](https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/model-viewer.png)](https://cloudgen.github.io/rust3d/sample-images/model/viewer.html)

## Examples

```bash
cd /path/to/folder/with/outline-images
rust3d
```

```bash
rust3d version
```

`version` prints `rust3d 1.0.1` and does not call cargo. A run that is not the text menu can also print status lines above that. The text menu keeps those lines off the screen.

```bash
rust3d model --views views.json --grid 96
```

That builds `model/model.glb` and `model/viewer.html` from outline images in the current directory and does not draw the menu. On a terminal, row **1** asks for the current folder or a subfolder, then does the same build. `cargo run -- model` is the same build from a checkout. A views file lists each outline basename with azimuth and elevation in degrees. Without one, the outlines are spaced evenly around the object at elevation 0.

## Platform Compatibility

| Platform | Status |
|----------|--------|
| Linux | Primary; tested development path |
| macOS | Supported when rustc and cargo are on PATH |
| Windows | Supported when rustc and cargo are on PATH |
| Architectures | Any with rustc |

The text menu needs a terminal. With no terminal and no verb, the program prints help and returns 0. It does not wait.

## Related Projects

- [rust3d on GitHub](https://github.com/cloudgen/rust3d) — this program’s source
- [rust3d on PyPI](https://crates.io/crates/rust3d) — this program on crates.io
- [OutlineImage on GitHub](https://github.com/cloudgen/OutlineImage) — writes a detailed outline image for each picture in a folder, and this program reads those images
- [OutlineImage on PyPI](https://pypi.org/project/OutlineImage/) — that program on PyPI
- [AnimeDlp](https://github.com/Wilgat/AnimeDlp) — command-line downloader for anime video sites
- [ChronicleLogger](https://github.com/Wilgat/ChronicleLogger) — status-line shape this program's in-tree log follows
- [VideoSpeed](https://github.com/Wilgat/VideoSpeed) — cuts, changes speed, and boomerangs a clip. That program is not this one
- [CIAO](https://github.com/cloudgen/ciao) — Caution, Intentional, Anti-fragile, Over-engineered
- [CIAO-Lite](https://github.com/cloudgen/ciao-lite) — short agent contract
- [safe-rm](https://github.com/cloudgen/safe-rm) — guarded `rm`

## Contributing

1. Keep product law under `docs/requirements/` in sync when behavior changes.
2. Prefer small, CIAO-safe changes. The model build lives in `src/model.rs`. `rust3d model` and `cargo run -- model` call that module.
3. Version dual SSOT: bump **`Cargo.toml`** `package.version` and **`src/lib.rs`** `VERSION` together.
4. Open issues and pull requests on GitHub.

## License

MIT — see [`LICENSE.md`](./LICENSE.md). Also declared in `Cargo.toml`.

## Last Update

2026-10-14 — **1.0.1** working tree: the package name is **rust3d** and the console script is `rust3d`. The public source is `https://github.com/cloudgen/rust3d`. `model` builds `model.glb` and `viewer.html` from outline images in a folder. The default directory is that folder's `model`. Menu row 1 lists **1** current folder, each subfolder, and **0** back, then runs that build. `screenshots/model-viewer.png` is a capture of `sample-images/model/viewer.html`, and that picture links to the viewer. Picture addresses are absolute `https` URLs. Menu pictures are captures of this product at **1.0.1**, including the folder board. Those pictures were recaptured on 2026-10-14. The path-row clock is `20:26:52`. Language is row **7** and self-management is row **6**. The status line says `rust3d 1.0.1`. Version badge matches `Cargo.toml` and `VERSION`. `chrono>=0.4`, `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2` are required.
