**file**: docs/requirements/requirement-rust-readme.md
**Status**: Active (Version 1.0.9 – Menu pictures recaptured on 2026-10-14. Language is front row 7. Self-management is front row 6. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-readme`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

This file owns the product user document at the repository root: which sections it has, which pictures it embeds, and the rule that a sentence in that document matches the requirement that owns the behavior. Project nature is a program a person builds and runs from a terminal. The menu picture stays on `requirement-rust-tui`. The thirteen languages stay on `requirement-rust-cli-language`. The verbs stay on `requirement-rust-cli-interface`. The outline stays on `requirement-domain-rust3d`. The encoder file is Retired. The prerequisite sentences stay on `requirement-runtime-prerequisites`. The package name and the console script stay on `requirement-rust-packaging`. The version string is `VERSION`. This file does not restate those bodies.

### 1.1 Human-facing

**In one sentence:** You open the root user document to install the program, read how outline images become a glTF model, and see pictures of this product.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | The person reading how to run the program | Open `README.md` at the checkout root |
| The other role | The requirements that own the menu, the outline, and the package | A sentence here matches those files |
| Not this file | The concat itself, the language file, and the host check | Pipeline, language, and domain |

| Includes | Excludes |
|----------|----------|
| Section order, badges, the Advantages contrast, install sentences, and the picture catalog | A second menu, a second verb list, or a second encode order |
| Absolute `https` image destinations for captures of the running text menu | A generated drawing that invents row text. A relative `screenshots/` image destination in this package description |
| The ten related-project lines | A shell online install, root as the documented path, or a setup script that is not on disk |

| Surface | What you open | What for |
|---------|---------------|----------|
| `README.md` | Root user document | Install, usage, pictures |
| `screenshots/` | Picture folder beside that document | The links in the Screenshots section |
| `rust3d` | The program on a terminal | What those pictures show |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Read the install | The public path is cargo. The document does not ask for root. | `cargo install rust3d` |
| Check the pictures | Row 7 is the language list. The pictures show this product. Row 1 is model. The folder board is in the catalog. About stays in English. | `rust3d`, then `7`, or `1` |
| Check the version line | The badge matches the package string. | `rust3d version` |

## 2. Core Rules / Requirements (Mandatory)

The user document is how a person learns the program. It is not a second copy of product law. A claim in the document matches the requirement that owns that claim.

### 2.1 Where it lives

1. **MUST** keep the user document at the repository-root `README.md`. Product law stays under `docs/requirements/`. The layout rule that the file exists is `requirement-rust-project-structure`. This file owns the sections and the pictures.
2. Pictures **MUST** live in a `screenshots/` folder at that same root. The user document **MUST** link every PNG in that folder. This document is the package description. Each image destination **MUST** be an absolute `https` URL that names that PNG on the public source host. A relative `screenshots/<file>` destination fails on the package index: the index fetches that path on its own host, and the picture folder is not in the published archive. The same relative destination still opens on the source forge. A link to a sibling document, such as `LICENSE.md`, **MAY** stay relative. A PNG on disk and absent from `README.md` fails this file. The URL base for this product is in Implementation Notes. That URL does not render on the package index until the file is on the public branch named in the URL.
3. **MUST NOT** freeze a session login or a `/home/<login>/` path in this requirement or in a sentence that is law. A picture may show the directory where that capture was started. The sentence in the document names “the current directory.”

### 2.2 What the document contains

The document **MUST** keep the sections in §2.5, in that order. A new heading, a dropped heading, or a reordered heading is a revision of this file.

Each behavioral sentence **MUST** match its owner and **MUST NOT** restate that owner’s procedure:

| Claim in the document | Owner |
|-----------------------|--------|
| Folder conversion, `<folder>/model` | `requirement-domain-rust3d` |
| No encoder | `requirement-video-ffmpeg-pipeline` is Retired |
| Menu picture, path row, clock, row numbers | `requirement-rust-tui` |
| Thirteen languages and what stays English | `requirement-rust-cli-language` |
| Product verbs and cargo lifecycle commands | `requirement-rust-cli-interface` |
| Empty argv on a terminal opens the menu. Empty argv with no terminal prints help and returns 0. Typed `model` converts and does not draw the menu | `requirement-rust-cli-interface` |
| Package name, console script, cargo install | `requirement-rust-packaging` |
| Package string on the badge | `VERSION` in `src/lib.rs`, copied by `Cargo.toml` |
| `chrono>=0.4` and the image floors | `requirement-rust-dependency-management` |
| Pip image stack, no host encoder, no root auto-install | `requirement-runtime-prerequisites` and `requirement-rust-dependency-management` |

This product does not claim `--json`. `model` is a product verb. `hello` is not. The user document does not name `./setup.sh` or `./build.sh`.

The first-row sentence in the user document **MUST** name the current directory on the left and a local clock (`HH:MM:SS`) on the right when the row has room. It **MUST NOT** name `Current`, a login, `USER`, `USERNAME`, or `getpass` as that right-hand field. The picture of the row stays on `requirement-rust-tui`.

### 2.3 Pictures

1. A menu picture **MUST** be a capture of the running text menu. **MUST NOT** satisfy a menu row with a drawing that invents row text. Every PNG in `screenshots/` **MUST** be linked from the user document. A PNG that is not a menu capture **MUST** be labeled as that file and **MUST NOT** be described as a text-menu capture.
2. The Screenshots section **MUST** show every PNG in `screenshots/`, in the catalog order. The lead **MUST** say that the pictures are captures of rust3d and that row 1 is **model**.
3. A paragraph **MUST** quote the pixels. It **MUST** name rust3d when the status line or the title shows that name. It **MUST NOT** describe a menu picture as VideoJoin or as a join of two videos. The domain file owns `model.glb` and `viewer.html`.
4. The about picture **MUST** stay in English. Leaf shorts in every menu picture **MUST** stay the English verb. That split is `requirement-rust-cli-language`.
5. A live-menu capture satisfies this file when it shows the menu the program paints, including a double-width label that stays whole and a clock that stays on the path row.
6. The heading of each picture **MUST** be that file’s basename. The paragraph and the image alt **MUST** be the catalog cells for that file. Those words come from the basename and from what the picture shows: the words on the screen, the characters in the input box, or the scene. A language name alone, or a sentence that only says the file is a picture, fails this file. The paragraph **MUST NOT** freeze a session login or a `/home/<login>/` path. The path row is “the current directory.”

### 2.4 Install honesty

1. The public install path **MUST** be cargo. **MUST NOT** document a shell online install, `a root package command`, or `sudo curl | sh`.
2. **MUST NOT** name a checkout setup script that is not on disk. This tree has no `setup.sh`. The document **MUST NOT** name `setup.sh`.
3. The document does not show a wheel or sdist example. **MUST NOT** add one unless the basename is the file the build writes and the version token matches the package version.
4. The prerequisite sentences **MUST** stay aligned with `requirement-runtime-prerequisites`. This file **MUST NOT** copy that table as a second owner.
5. **MUST NOT** add `./build.sh` verbs to the user document. Those verbs are not `rust3d` verbs.

### 2.5 Implementation Notes (this project)

| Field | Value |
|-------|--------|
| **Document** | `README.md` |
| **Pictures** | `screenshots/` |
| **Package** | `rust3d` |
| **Console script** | `rust3d` |
| **Version** | `1.0.1` from `src/lib.rs` (`VERSION`). `Cargo.toml` copies that string |
| **Python** | `>=3.10`. The badge says 3.10+ |
| **License file** | `LICENSE.md` (MIT). The file is present. The document link stays relative |
| **Homes** | `https://github.com/cloudgen/rust3d` and `https://crates.io/crates/rust3d` |
| **Current job** | `rust3d model` writes `model.glb` and `viewer.html` in `<folder>/model` |
| **Older pictures** | None. Menu pictures are captures of this product. `join-*.png` are not in `screenshots/` |
| **Every PNG** | `README.md` links each PNG under `screenshots/`. `model-viewer.png` is linked and is not a menu capture |
| **Image URL** | `https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/<basename>`. The package index can fetch that file only after it is on that public branch |
| **Viewer link** | `https://cloudgen.github.io/rust3d/sample-images/model/viewer.html`. `model-viewer.png` links to this address. The image destination stays the Image URL row |

**Title:** `rust3d - A glTF model and an HTML viewer from outline images`.

**Badges:** a Version badge whose token is the package string; License MIT; CIAO linked to `https://github.com/cloudgen/ciao`; stars for `cloudgen/rust3d`; Python 3.10+; crates.io `rust3d` linked to `https://crates.io/crates/rust3d`.

**Opening:** build one glTF model and one HTML viewer from outline images in a folder. The files are `model.glb` and `viewer.html`. On a terminal with no arguments, the text menu opens. The menu does not build a model by itself and does not call cargo. The opening also shows `model-viewer.png` linked to the viewer address in Implementation Notes. The image destination is the Image URL row.

**Sections, in order.** Advantages is inserted after Features. Screenshots is inserted after Usage. The other headings keep the existing kit order.

| Heading | What this document must keep | Owner of the behavior |
|---------|------------------------------|------------------------|
| Features | Text menu: model, system-log, language, self-management, Exit. Path and a local clock on the first row. Row 1 lists the current folder, each subfolder, and back. system-log is **3**. self-management is **6**. language is **7**. Verbs `help`, `version`, `about`, `model`, `self-install`, `version-check`, `self-update`, `self-uninstall`. One binary, `rust3d`. `cargo run` starts it | Peers in §2.2. The path-row sentence is the clock rule in §2.2 |
| Advantages | The four parts and the comparison table below | This file for the contrast. Peers in §2.2 for the behavior |
| Quick Installation | `chrono>=0.4`, `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2`. `cargo install rust3d`. No external media tool. Checkout uses a venv and `cargo install -e .`. The fenced text menu (front, system-log, language, self-management) stays in this section and shows row 1 **model** and version **1.0.1** | `requirement-rust-dependency-management`, `requirement-runtime-prerequisites`, `requirement-rust-packaging`, `requirement-rust-tui` |
| Usage | Menu versus typed verbs. `model` converts a folder and does not draw the menu. Menu row 1 lists the current folder, each subfolder, and back. Default `model.glb` and `viewer.html` in that folder's `model` directory. No terminal: empty argv prints help and returns 0. `self-uninstall` needs `--force`. No sudo | `requirement-rust-cli-interface`, `requirement-domain-rust3d`, `requirement-rust-tui` |
| Screenshots | Lead: each heading is the file name, and the paragraph is what that picture shows. Package **1.0.1**. The paragraph and the image alt are the catalog below. The pictures are captures of this product, recaptured on 2026-10-14. Language is row **7**. Self-management is row **6**. Row 1 is **model**. `model-folder.png` is the folder board. `model-viewer.png` links to the viewer address in Implementation Notes | This file |
| Examples | `cd` to a folder of images, then `rust3d`, then `rust3d version`, then `rust3d model` | `requirement-domain-rust3d` |
| Platform Compatibility | Linux primary. macOS and Windows when rustc is present. The text menu needs a terminal | `requirement-runtime-prerequisites` |
| Related Projects | The ten lines below, in that order, each with one sentence | This file |
| Contributing | Keep product law in sync. The conversion lives in `src/model.rs`. Version strings stay together | `requirement-rust-coding-style`, `requirement-rust-packaging` |
| License | MIT, link `LICENSE.md`, and that file exists | `requirement-rust-packaging` |
| Last Update | Names package **1.0.1**, the public source, the model build, the `model` directory, the viewer capture, and that menu pictures are captures of this product | This file for the line. `VERSION` for the string |

**Related projects, in this order.** The first two are this program. Each line is one sentence. Do not add an install recipe.

| Order | Link | Sentence |
|------:|------|----------|
| 1 | `https://github.com/cloudgen/rust3d` | This program’s source |
| 2 | `https://crates.io/crates/rust3d` | This program on crates.io |
| 3 | `https://github.com/cloudgen/OutlineImage` | Writes a detailed outline image for each picture in a folder, and this program reads those images |
| 4 | `https://pypi.org/project/OutlineImage/` | That program on crates.io |
| 5 | `https://github.com/Wilgat/AnimeDlp` | Command-line downloader for anime video sites |
| 6 | `https://github.com/Wilgat/StatusLog` | Status logger this program depends on (`chrono>=0.4`) |
| 7 | `https://github.com/Wilgat/VideoSpeed` | Cuts, changes speed, and boomerangs a clip. That program is not this one |
| 8 | `https://github.com/cloudgen/ciao` | Caution, Intentional, Anti-fragile, Over-engineered |
| 9 | `https://github.com/cloudgen/ciao-lite` | Short agent contract |
| 10 | `https://github.com/cloudgen/safe-rm` | Guarded `rm` |

**Advantages, after Features and before Quick Installation.** Three parts. The path row in Features names the local clock. There is no FFmpeg comparison table.

1. **A folder of outlines in, one model out.** `rust3d model` converts images in the current directory and does not draw the menu. On the menu, row **1 model** asks for the current folder or a subfolder, then writes `model.glb` and `viewer.html` in that folder's `model` directory. There is no `--json`. With no arguments on a terminal, the text menu opens. With no terminal and no verb, the program prints help and returns 0.
2. **Built-in languages.** Row **7** lists thirteen languages: English, Simplified Chinese, Traditional Chinese, Spanish, Arabic, French, Portuguese, Russian, German, Japanese, Korean, Dutch, and Greek. The choice is saved for the next run.
3. **Lifecycle and diagnostics.** `version-check`, `self-update`, `self-install`, and `self-uninstall` are menu rows **64**–**67** and typed verbs. `self-uninstall` on the command line needs `--force`. They call cargo and do not use root. **system-log** (**3**) views a log, clears a log, and shows the log folder. **about** (**63**) stays in English. The document does not include an FFmpeg comparison table.

**Screenshot catalog, in this order.** Each image destination is the absolute `https` URL in Implementation Notes, with that file’s basename. The paragraph and the alt are the words `README.md` prints for that file. The pictures are captures of rust3d. Do not describe them as VideoJoin.

| Order | File | Paragraph | Alt |
|------:|------|-----------|-----|
| 1 | `language-menu.png` | Row **7** has opened the language list. **71 English** is highlighted, with the note "use English for this menu" The other rows are **72 简体中文**, **73 繁體中文**, **74 Español**, **75 العربية**, **76 Français**, **77 Português**, **78 Русский**, **79 Deutsch**, **80 日本語**, **81 한국어**, **82 Nederlands**, and **83 Ελληνικά**. Each note says to use that language for this menu. **0 Back** says "return to the main menu" The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `language`. | Language list, 71 English highlighted |
| 2 | `main-menu-en.png` | English main menu. There is no saved-language line above the box. The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "build a 3D model from outline images in a chosen folder" Then **3 system-log** "view, clear, and the log folder" **6 self-management** "version, about, and cargo lifecycle" **7 language** "display language for this menu" and **9 Exit** "leave" The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `main menu`. | English main menu |
| 3 | `main-menu-zh-hans.png` | Simplified Chinese main menu. The line above the box says `菜单语言是简体中文`. The path label is `路径`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "把所选文件夹中的轮廓图做成三维模型" **3** is `系统日志`, **6** is `自我管理`, **7** is `语言`, and **9** is `离开`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `主菜单`. | Simplified Chinese main menu |
| 4 | `main-menu-zh-hant.png` | Traditional Chinese main menu. The line above the box says `選單語言是繁體中文`. The path label is `路徑`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "把所選資料夾中的輪廓圖做成三維模型" **3** is `系統日誌`, **6** is `自我管理`, **7** is `語言`, and **9** is `離開`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `主選單`. | Traditional Chinese main menu |
| 5 | `main-menu-es.png` | Spanish main menu. The line above the box says `El idioma del menú es español`. The path label is `Ruta`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted: "construye un modelo 3D con los contornos de una carpeta elegida" **3** is `registro`, **6** is `autogestión`, **7** is `idioma`, and **9** is `Salir`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `menú principal`. | Spanish main menu |
| 6 | `main-menu-ar.png` | Arabic main menu. The line above the box says `لغة القائمة هي العربية`. The path label is `المسار`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The numbers stay on the left. Arabic words on each row are shaped and read right to left. **1 model** is highlighted and stays `model`: "يبني نموذجا ثلاثي الأبعاد من حدود المجلد المختار" **3** is `سجل النظام`, **6** is `إدارة ذاتية`, **7** is `لغة`, and **9** is `خروج`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `القائمة الرئيسية`. | Arabic main menu |
| 7 | `main-menu-fr.png` | French main menu. The line above the box says `La langue du menu est le français`. The path label is `Chemin`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "construit un modèle 3D à partir des contours d'un dossier choisi" **3** is `journal`, **6** is `autogestion`, **7** is `langue`, and **9** is `Quitter`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `menu principal`. | French main menu |
| 8 | `main-menu-pt.png` | Portuguese main menu. The line above the box says `O idioma do menu é português`. The path label is `Caminho`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "constrói um modelo 3D a partir dos contornos de uma pasta escolhida" **3** is `registo`, **6** is `autogestão`, **7** is `idioma`, and **9** is `Sair`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `menu principal`. | Portuguese main menu |
| 9 | `main-menu-ru.png` | Russian main menu. The line above the box says `Язык меню — русский`. The path label is `Путь`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "собрать трёхмерную модель из контуров выбранной папки" **3** is `системный журнал`, **6** is `самоуправление`, **7** is `язык`, and **9** is `Выход`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `главное меню`. | Russian main menu |
| 10 | `main-menu-de.png` | German main menu. The line above the box says `Die Menüsprache ist Deutsch`. The path label is `Pfad`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "aus Umrissen eines gewählten Ordners ein 3D-Modell bauen" **3** is `Systemprotokoll`, **6** is `Selbstverwaltung`, **7** is `Sprache`, and **9** is `Beenden`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `Hauptmenü`. | German main menu |
| 11 | `main-menu-ja.png` | Japanese main menu. The line above the box says `メニューの言語は日本語`. The path label is `パス`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "選んだフォルダの輪郭画像から3Dモデルを作る" **3** is `システムログ`, **6** is `自己管理`, **7** is `言語`, and **9** is `終了`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `メインメニュー`. | Japanese main menu |
| 12 | `main-menu-ko.png` | Korean main menu. The line above the box says `메뉴 언어는 한국어`. The path label is `경로`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "고른 폴더의 윤곽 이미지로 3D 모델을 만듭니다" **3** is `시스템 로그`, **6** is `자기관리`, **7** is `언어`, and **9** is `종료`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `주 메뉴`. | Korean main menu |
| 13 | `main-menu-nl.png` | Dutch main menu. The line above the box says `De menutaal is Nederlands`. The path label is `Pad`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "maak een 3D-model van contouren in een gekozen map" **3** is `systeemlog`, **6** is `zelfbeheer`, **7** is `taal`, and **9** is `Afsluiten`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `hoofdmenu`. | Dutch main menu |
| 14 | `main-menu-el.png` | Greek main menu. The line above the box says `Η γλώσσα του μενού είναι ελληνικά`. The path label is `Διαδρομή`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 model** is highlighted and stays `model`: "φτιάχνει ένα τρισδιάστατο μοντέλο από τα περιγράμματα ενός φακέλου" **3** is `αρχείο καταγραφής`, **6** is `αυτοδιαχείριση`, **7** is `γλώσσα`, and **9** is `Έξοδος`. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `κύριο μενού`. | Greek main menu |
| 15 | `model-folder.png` | The folder board for row **1**. The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. **1 current** is highlighted: "build a 3D model from outline images in this folder" **2 photos** says "build a 3D model from outline images in this subfolder" **0 Back** says "return to the main menu" The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `model`. | Folder board, current folder highlighted |
| 16 | `self-management.png` | Row **6** has opened self-management. **62 version** is highlighted: "show the installed version" Then **63 about** "version and this computer", **64 version-check** "compare this install with cargo", **65 self-update** "upgrade this package with cargo", **66 self-uninstall** "remove this package with cargo", **67 self-install** "install this package with cargo", and **0 Back** "return to the main menu" The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `self-management`. | Self-management, 62 version highlighted |
| 17 | `tui-about.png` | **about** (63) on the result page. The title is `rust3d (1.0.1) — result`. The page prints `rust3d 1.0.1`, `Domain: Build a glTF model and an HTML viewer from outline images in a folder`, `Runtime tools: none`, and `Entry points: rust3d`. The host check is headed `2026-10-14 20:26:52.404013 rust3d (v1.0.1)  [CHECK SYSTEM]:` and `Now checking your operation system!`. The lines are `Rustc: rustc 1.89.0 (29483883e 2025-08-04)`, `C Library: GNU libc`, `Operation System: Ubuntu 24.04.5 LTS`, `Architecture: x86_64`, `Current User: operator`, `Shell: /bin/bash`, `Rustc Executable: /tmp/rust3d-tools/rustc`, `cargo location: /tmp/rust3d-tools/cargo`, `rustup location: /tmp/rust3d-tools/rustup`, `Inside docker container: false`, `Target triple: x86_64-unknown-linux-gnu`, `Binary Type: x86_64-GNU libc`, `Location: /tmp/rust3d-bin/rust3d`, `PID: 249978`, `Cache folder used: /dev/shm/cache/cache-rust3d-operator-249978`, `Cache folder (preferred): /dev/shm/cache/cache-rust3d-operator-249978`, `Cache folder (1st fallback): /tmp/cache/cache-rust3d-operator-249978`, `Cache folder (2nd fallback): /tmp/rust3d-home/.cache/cache-rust3d-249978`, `Persistence storage: /tmp/rust3d-home/.local/rust3d`, and `TTY / Interactive: yes`. The star box says `rust3d (1.0.1) by Wilgat Wong on 2026-10-11`, `You are using an UNINSTALLED version, location:`, `/tmp/rust3d-bin/rust3d`, `Basic Usage:`, `rust3d model`, and `Please visit our homepage:` with `"https://github.com/cloudgen/rust3d"`. The footer says `Press a key to return to the main menu.` There is no input box and no clock. The page stays in English. | About host check, English |
| 18 | `system-log.png` | Row **3** has opened system-log. **31 view-log** is highlighted: "list a log file and show it" Then **32 clear-log** "empty one log file", **33 log-folder** "show the log folder", and **0 Back** "return to the main menu" The path label is `Path`. The path is `/tmp/clips`. The clock is `20:26:52` on the right of that row. The input box shows `>` and no typed text. The status line says `rust3d 1.0.1` and `system-log`. | System log, 31 view-log highlighted |
| 19 | `model-viewer.png` | A capture on 2026-10-14 of the converted model in `sample-images/model/viewer.html`. The page background is near black. The mesh is light gray and built from stacked outline slices: a wide upper mass, an opening through the middle, and a lower mass with a flat slab. The line at the bottom left reads "rust3d — drag to orbit, wheel to zoom. model.glb is the glTF file." This is not a text-menu capture. The picture links to that viewer. | Converted model in the HTML viewer |

`TP-DOC-01` asserts the Version badge token, that every PNG is linked with the absolute `https` URL above, that `model-viewer.png` links to the viewer address in Implementation Notes, that no markdown image uses a relative `screenshots/` destination, that the section headings are present, that the document does not name `setup.sh`, that the ten related-project URLs are in order, that the path row names the local clock, and that `model-viewer.png` is not described as a text-menu capture. `TP-DOC-03` asserts that each screenshot paragraph and alt equals the catalog cell, and it does not claim a pixel-by-pixel proof. `TP-DOC-03` stays **todo**. Map: `docs/reviews/test-plan.md`.

### 2.6 Why This Requirement Exists (Direct CIAO Alignment)

- **CIAO Principle 2 – Intentional** (https://github.com/cloudgen/ciao): The sections and every PNG in `screenshots/` are a named catalog. A later edit cannot drop the language list, leave a PNG unlinked, drop the viewer link on `model-viewer.png`, or swap in a drawing for a menu row.
- **CIAO Principle 5 – SSOT** (https://github.com/cloudgen/ciao): This file owns the document. Behavior stays on the peer that already owns it.
- **CIAO Principle 16 – Interactive** (https://github.com/cloudgen/ciao): The pictures show a text menu, including row **7**, the folder board, self-management, about, and system-log.
- **CIAO Principle 21 – Dual policies** (https://github.com/cloudgen/ciao): The portable rule is “the document matches its owner.” The rust3d headings and file names live in §2.5.
- **CIAO Principle 4 – Over-protect** (https://github.com/cloudgen/ciao): Do not publish a setup script that is not on disk, or a success line the program did not print.

## Under command line for normal user only

The documented install is for this login. **This requirement:** the user document **MUST NOT** tell the reader to use `sudo`, `a root package command`, `sudo curl | sh`, or a dedicated system account. Git Bash and Windows cmd **MUST NOT** be told to invoke Termux `pkg`. The public install path is cargo. A picture does not change who may run a verb.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** An install sentence that names a missing script is a defect.
- **Intentional:** An Advantages section, nineteen pictures, one folder board, one viewer capture, one section order. Each picture paragraph is the catalog cell for that file.
- **Anti-fragile:** A new PNG in `screenshots/` becomes a required link in the same change. A non-menu file stays labeled as itself.
- **Over-protect (Principle 20):** Do not let the user document become a second menu, a second verb list, or a second encode procedure.

## 4. Protection Rule (Sacred)

**Future AI assistants or maintainers MUST NOT**:

- Restate encode order, verb lists, menu numbers, language codes, or the prerequisite table in this file as a second procedure.
- Turn the Advantages section into an FFmpeg filter recipe. The concat graph may be named only as something the person does not write.
- Claim that intermediate files never use the system temporary directory.
- Replace a capture with a drawing that invents row text.
- Put VideoJoin, or a join of two videos, back into `screenshots/` or into a picture paragraph.
- Leave any PNG in `screenshots/` out of `README.md`.
- Use a relative `screenshots/` destination for a picture in this package description. The package index cannot fetch that path. A sibling-document link such as `LICENSE.md` may stay relative.
- Describe `screenshots/model-viewer.png` as a text-menu capture.
- Caption a picture with only a language name, or with a sentence that does not say what the picture shows.
- Describe the path row’s right side as `Current` or a login.
- Name `./setup.sh` while that file is absent.
- Add `./build.sh` verbs, a shell online install, `a root package command`, or `sudo curl | sh` to the user document.
- Add `--json`, a length percent, a boomerang, or a folder prompt.
- Freeze a session login or a `/home/<login>/` path in this file.
- Mark `TP-DOC-03` have before the suite asserts that each catalog paragraph and alt equals the cell in §2.5.
- Treat `./build.sh` verbs as `rust3d` verbs.

## 5. Definition of done

1. `README.md` has the sections in §2.5, in that order, including Advantages after Features and Screenshots after Usage.
2. The Version badge token matches the package string.
3. The Screenshots section links every PNG in `screenshots/`, in catalog order, including `model-viewer.png`. Each image destination is an absolute `https` URL. Each heading is the basename. Each paragraph and each image alt are the catalog cells for that file.
4. The path-row sentence names the local clock.
5. The document does not name `setup.sh`, `a root package command`, or a shell online install.
6. Related Projects lists the ten URLs in §2.5, in that order.
7. `TP-DOC-01` is **have** in `tests/product.rs` (`./tests/run.sh`, 7 tests, OK). `TP-DOC-03` stays **todo** until the suite asserts that each paragraph and alt equals the catalog cell.

### Design-time verification

| TP family / ID | Suite | Status |
|----------------|-------|--------|
| **TP-DOC-01** Version badge matches the package string. Every PNG uses the absolute `https` URL. `model-viewer.png` links to the viewer address. No relative `screenshots/` image. Section headings. No `setup.sh`. Ten related-project URLs in order. The path row names the local clock. `model-viewer.png` is not a text-menu capture | `tests/product.rs` | have |
| **TP-DOC-03** Each screenshot paragraph and alt equals the catalog cell in §2.5 | `tests/product.rs` | todo |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `docs/requirements/index.md` | Registry |
| `requirement-rust-project-structure` | Root `README.md` exists. Sections and pictures are this file |
| `requirement-domain-rust3d` | Domain rows inside the user document |
| `requirement-video-ffmpeg-pipeline` | Stream copy, then re-encode. This file does not restate that procedure |
| `requirement-runtime-prerequisites` | Pip image stack. No host encoder. No root auto-install |
| `requirement-rust-dependency-management` | The four cargo floors in Quick Installation |
| `requirement-rust-packaging` | Package name, console script, cargo install |
| `requirement-rust-tui` | Menu picture and the clock |
| `requirement-rust-about` | The about picture. This file does not own the page |
| `requirement-rust-cli-language` | Row **7** and the thirteen codes. About stays English |
| `requirement-rust-cli-interface` | Product verbs the document lists. `model` is one of them |
| `requirement-rust-coding-style` | `std::fs::rename` for the published file |
| `requirement-class-software-dev` | Class residual |
| `README.md` | The user document |
| `screenshots/` | The captures |
| `docs/reviews/test-plan.md` | `TP-DOC-01` and `TP-DOC-03` |

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### crates.io-compatible README format

**Definition:** **crates.io-compatible README format** is the **shape of the product README** that a package index can render as the **package description**: markdown the index accepts, image destinations that are **absolute `https` URLs** (the picture rule remains crates.io HTML image rendering), a badge row that is already absolute `https`, a `cargo install` name that matches `Cargo.toml` `package.name`, and a version badge that matches project target version. A **relative** `screenshots/` image destination is **not** this format. A link to a **sibling document** may stay relative. This term is the **whole description shape**. It is not the live version probe and not the architecture class.

**Human daily-life explanation:**

**crates.io-compatible README format** is the leaflet written so the warehouse window can reprint it. Photos use a full street address. The install name on the leaflet is the same name as on the catalog card. The edition sticker matches the edition you confirmed. A note that only says “photo in the drawer” is a leaflet the warehouse cannot reprint. A link to the next page in the same booklet can stay a short note.

**Daily-life example:** The leaflet says `cargo install pancake-timer`, the edition sticker says `1.2.0`, and each photo is an `https` address. A photo written as `screenshots/menu.png` still opens in your own shop and shows a blank frame in the warehouse window.

### crates.io HTML image rendering

**Definition:** **crates.io HTML image rendering** is the rule for a **markdown image** inside a `README.md` that is also the **package description** on a package index. The image **destination** is an **absolute `https` URL** that names the picture on a host both the source forge and the package index can fetch. A **relative** destination such as `screenshots/<file>` is resolved by the index as a path on the index host. The picture is not inside the published archive, so that fetch fails and the page shows a broken image. The same relative destination still resolves on the source forge, because the forge resolves it against the repository tree. A link to a **sibling document** (a license file, another markdown page) may stay relative. This term does not choose the caption words and does not require the picture bytes to be uploaded inside the archive.

**Human daily-life explanation:**

**crates.io HTML image rendering** is the shop-window rule for a photo in the leaflet that the warehouse also reprints. The warehouse does not keep your photo drawer. A note that says “photo in the drawer” works in your own shop and shows a blank frame in the warehouse. A full street address that anyone can open works in both windows.

**Daily-life example:** A picture written as `screenshots/language-menu.png` opens on the source forge and fails on the package index. A picture written as `https://raw.githubusercontent.com/cloudgen/rust3d/main/screenshots/language-menu.png` is an address both windows can fetch. The badge row already uses absolute `https` and is a different sticker.

### README banner

**Definition:** A **README banner** (also **badge**, **shield**, **status badge**) is a small **markdown image** (often from shields.io or GitHub’s badge endpoints) placed in the product root **`README.md` banner row** under the H1. Banners summarize version, license, philosophy, stack, popularity, CI, or other at-a-glance facts. They are **not** a substitute for full sections (Quick Installation, License prose, Features).

**Human daily-life explanation:**

A **readme-banner** is a small status sticker in the README row: version, license, stack, and so on. It is not a substitute for full install or license sections.

**Daily-life example:** The tiny icons on a café awning (open, cards accepted) vs the full menu and house rules inside.

### Project target version

**Definition:** **Project target version** is the **user-confirmed product version string** (typically `MAJOR.MINOR.PATCH`, e.g. `1.0.0` or `1.7.0`) that agents **must collect** from the user (or confirm from an existing product root **`README.md`** parse). After it is written into product root **`README.md`**, that file is the **single source of truth (SSOT)** for this field—displayed **once** in the product README **header kit** as the shields.io **Version** badge. It is **not** hardcoded into skills, requirements, templates, or terminology files.

**Human daily-life explanation:**

**Project target version** is the user-confirmed version string shown once as the README Version badge. Skills do not own it.

**Daily-life example:** The edition number printed on a cookbook’s cover; the inner chapters should match, but the cover is what you asked to print.

### App name

**Definition:** **App name** is the **user-facing product / application name** shown in the product root **`README.md` header** as the first half of the H1 (`# rust3d - the one-line pitch`). Agents **must collect** it from the user (or confirm an existing README parse). After it is written into **`README.md`**, that file is the **single source of truth (SSOT)** for this display identity field for docs and agent reads—not skills, not requirements, not templates, and not terminology files.

**Human daily-life explanation:**

**App name** is the short name painted on the product’s front sign (the left half of the README title). After it is written there, that sign is the one place agents trust for the display name — not a skill sheet and not a reusable blank.

**Daily-life example:** The café’s window says “Harbor Kettle.” That painted name is the **App name**. The back-room machine id may match it, but guests read the window.

### Project short description

**Definition:** **Project short description** is the **one-line product pitch** shown in the product root **`README.md` header** as the second half of the H1 after ` - ` (`# rust3d - the one-line pitch`). Agents **must collect** it from the user (or confirm an existing README parse). After it is written into **`README.md`**, that file is the **single source of truth (SSOT)** for this field—not skills, not requirements, not templates, and not terminology files.

**Human daily-life explanation:**

**Project short description** is the one-line pitch after the product name in the README H1. After it is written there, README owns it.

**Daily-life example:** The six-word slogan under a café’s painted name on the awning, not the full menu inside.

### Proper project version bump

**Definition:** A **proper project version bump** is a **user-ordered semver increase** of the product ship version that is **strictly newer** than the previous local ship version, applied in the **same change** on every in-scope surface. When `Cargo.toml` exists, `package.name` is the **app and package name** and `package.version` **equals** the import-package `VERSION`, the ship-unit version, the README Version badge (project target version), and the current CHANGELOG heading. A bump is **not** a side effect of editing a requirement, a skill, or a glossary file. Agents **MUST NOT** invent a second version literal beside the one the user ordered.

**Human daily-life explanation:**

A **proper project version bump** is printing one newer edition number on every copy of the book in the same sitting, because someone asked for a new edition. Editing a workshop note does not print a new edition by itself. When the catalog card exists, the name on the card is the product name and the number on the card is the same number as the title page.

**Daily-life example:** The kitchen card says name `pancake-timer` and edition `1.2.0`. The title page, the cover sticker, and the change list all say `1.2.0`. The next ordered edition is `1.2.1` on all of those surfaces together. A note in the workshop drawer does not become `1.2.1` on its own.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-04 | Active 1.0.0 | User-document sections, Advantages, eight related projects, and twenty-two pictures. Image destinations are absolute `https`. `TP-DOC-01` have. `TP-DOC-03` todo |
| 2026-10-04 | Active 1.0.1 | About and self-management pictures match the new page. Other menu pictures still show **1.0.4**. Package string is **1.0.5** |
| 2026-10-05 | Active 1.0.2 | Quick Installation names the four cargo floors. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.0.3 | Public source is `cloudgen/rust3d`. `model-viewer.png` is the converted viewer and links to `sample-images/model/viewer.html` with an absolute `https` address. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.0.4 | The viewer link is the GitHub Pages address that serves `sample-images/model/viewer.html`. The picture address stays on `raw.githubusercontent.com`. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.0.5 | `video.png` is removed from the catalog and from `screenshots/`. Related Projects adds OutlineImage on GitHub and on crates.io. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.0.6 | Product version **1.0.1**. Menu pictures are captures of this version. The status line says `rust3d 1.0.1`. The clock is `20:26:52`. The about stamp is `2026-10-05 20:26:52.404013` |
| 2026-10-05 | Active 1.0.7 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.0.8 | Language is front row 7, children 71–83. Self-management is front row 6, children 62–67. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.0.9 | Menu pictures were recaptured. The clock is `20:26:52`. The about stamp is `2026-10-14 20:26:52.404013`. Language is row 7. Self-management is row 6. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
