**file**: docs/requirements/requirement-rust-about.md
**Status**: Active (Version 1.0.5 – About is menu row 63 under self-management row 6. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-about`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

This file is the about page for rust3d. Menu action **about** returns one text page: the product identity lines, a host check, and a star-bordered box. This file names every line and the read that fills it.

The screen that shows the page stays on `requirement-rust-tui`. The domain sentence stays on `requirement-domain-rust3d`. The version string stays the single `VERSION` on `requirement-rust-packaging`. `--version` stays a one-line human string on `requirement-rust-cli-interface`. This product does not claim `--json`. This file does not open the menu, convert a folder, or write a JSON object.

Pyenv and conda path reads for this page stay in this file. This product does not add `requirement-rust-pyenv` or `requirement-rust-conda`.

### 1.1 Human-facing

**In one sentence:** On the text menu, choose **about** and the page names this build, this computer, and where the program is running from.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Person reading the about page | Menu row **63 about**, under **6** self-management |
| The other role | The text-menu result page | `requirement-rust-tui` draws it and scrolls it |
| Not this file | The outline conversion, the cargo rows, and a JSON object | Domain, CLI, and the absent JSON requirement |

| Includes | Excludes |
|----------|----------|
| Identity lines, the host-check lines, and the star box | A second about layout, a remote version lookup |
| How each value is read on this machine | Running `python2` or `python3` to ask their version |
| Install kind: global, local, or uninstalled | An install command while this product has no download URL |
| Pyenv and conda path reads used by the location lines | A separate pyenv or conda requirement |

| Surface | What you open | What for |
|---------|---------------|----------|
| `rust3d` | text menu on a terminal | Choose **6**, then **63**, or type `about` |
| `rust3d about` | the same page, no front-board pick | No folder prompt and no file prompt |
| `src/about.rs` | struct `AboutPage` | The page text. It calls `CheckSystem` for the host check |
| `src/about.rs` | struct `CheckSystem` | Host-check reads, including pyenv and conda paths |
| `tests/product.rs` | suite | The lines and the reads |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Open about | The menu stays in the result page. The page starts with the product name and the domain sentence, then the host check, then the star box. Up and Down scroll when the page is longer than the screen. Any other key returns to the menu. | `rust3d`, then `6`, then `63` |
| Read the install sentence | A checkout under `src/rust3d` with `Cargo.toml` above it says UNINSTALLED. A copy under your home directory says LOCAL INSTALLED. A copy under `/usr`, `/opt`, or `/bin` says GLOBAL INSTALLED. | Same page |
| Read the toolchain lines | The host check names `rustc --version`, the cargo path, and the rustup path. A missing command leaves the value empty. The page does not name pyenv or conda. | `rust3d about` |
| Read the run lines | After `Location`, the check names this process id, the cache folder in use and its three candidates, the persistence directory, and whether stdout is a terminal. | Same page |

## 2. Core Rules (Mandatory)

1. **One page.** Menu action **about** and the verb `rust3d about` **MUST** show one result page whose body is this file. The verb **MUST NOT** ask for a folder or a file. The page **MUST** be three blocks in this order: identity, host check, star box. A blank line **MUST** separate the blocks. The page **MUST NOT** be `--version`. Without a terminal, that page **MUST** be the whole of standard output. This product does not claim `--json`. The page **MUST NOT** gain a JSON stream.
2. **Identity block.** The first four lines **MUST** be, in order: `{APP_NAME} {version}`, `Domain: Build a glTF model and an HTML viewer from outline images in a folder`, `Runtime tools: none`, `Entry points: rust3d`. `{version}` **MUST** be the package version string `VERSION`. The domain sentence **MUST** stay the sentence on `requirement-domain-rust3d`. The runtime-tools line is static. It **MUST NOT** name FFmpeg. The page **MUST NOT** name `py-tui`. It **MUST NOT** print an author email. A non-empty download URL on the star box **MAY** print `cargo install rust3d`. The product caller passes an empty URL, so the live page omits that line.
3. **Host-check header.** The check **MUST** start with `{local stamp} {APP_NAME} (v{version})  [CHECK SYSTEM]:` and the next line **MUST** be two spaces then `Now checking your operation system!`. The stamp **MUST** be local time `YYYY-MM-DD HH:MM:SS.ffffff`. There are two spaces before `[CHECK SYSTEM]:`.
4. **Host-check fields.** After the header, the page **MUST** print these labels in this order, each starting with four spaces, then the label, a colon, a space, and the value. A missing command **MUST** leave the line in place. The check **MUST NOT** spawn `python2`, `python3`, `conda`, `pyenv`, or `id`. It **MUST NOT** print pyenv, conda, or a Cython string.

| Label | Read |
|-------|------|
| `Rustc` | `rustc --version`, or `not found` when the command fails. |
| `C Library` | `GNU libc` when `/lib/x86_64-linux-gnu/libc.so.6` or `/lib64/libc.so.6` exists. Otherwise `unknown`. |
| `Operation System` | `PRETTY_NAME` from `/etc/os-release` when that line is present. Otherwise the Rust OS name. |
| `Architecture` | The Rust architecture name. |
| `Current User` | Environment `USER`, else empty. |
| `Shell` | Environment `SHELL`, else empty. |
| `Rustc Executable` | `which rustc`, else empty. |
| `cargo location` | `which cargo`, else empty. |
| `rustup location` | `which rustup`, else empty. |
| `Inside docker container` | `true` when `/.dockerenv` exists, otherwise `false`. No other container probe. |
| `Target triple` | The `host:` line of `rustc -vV`. Otherwise `{arch}-unknown-{os}`. |
| `Binary Type` | `{arch}-{libc}` when both are non-empty. `{arch}-` when libc is empty. Empty when arch is empty. |
| `Location` | The program path from rule 8. The same path **MUST** be the indented path inside the star box. |
| `PID` | `std::process::id()` as decimal text. |
| `Cache folder used` | Rule 17. The page **MUST NOT** create the cache directories. |
| `Cache folder (preferred)` | Rule 17. |
| `Cache folder (1st fallback)` | Rule 17. |
| `Cache folder (2nd fallback)` | Rule 17. The leaf has no username. |
| `Persistence storage` | Rule 17. Durable data. Not the user bin. Not the cache. The language leaf lives under this directory. This read **MUST NOT** create it. |
| `TTY / Interactive` | `yes` when stdout is a terminal, otherwise `no`. This line does not measure stdin and does not choose the menu. |

5. **C Library.** The label is `GNU libc` when a known libc file exists, otherwise `unknown`.
6. **Architecture.** Use the architecture constant from the Rust standard library.
7. **Binary type.** `{architecture}-{c library}`.
8. **Program path.** Use the current executable path. The same path **MUST** be the indented path inside the star box when the caller does not pass another location.
9. **Install kind.** Classify that path in this order.
   - The path contains `/src/` or `/target/` and a parent within eight levels has `Cargo.toml`: **uninstalled**.
   - Under the home directory, and the path contains `/.cargo/` or the basename is the console name or `rust3d`: **local**.
   - Starts with `/usr/`, `/opt/`, or `/bin/`: **global**.
   - Otherwise: **uninstalled**.
10. **Star box sentences.** **global** **MUST** say `You are using the GLOBAL INSTALLED version, location:`. **local** **MUST** say `You are using the LOCAL INSTALLED version, location:`. **uninstalled** **MUST** say `You are using an UNINSTALLED version, location:`.
11. **Star box lines.** Inside the border, in order: the title `{APP_NAME} ({version}) by {author} on {date}`; a blank line; the install sentence; four spaces and the program path; a blank line; `Basic Usage:`; four spaces and the usage line; a blank line; `Please visit our homepage: `; four spaces, a double quote, the homepage, and a double quote; then the trailing blanks from rule 12. `{version}` **MUST** be the package version string. **MUST NOT** split it into `MAJOR`, `MINOR`, and `PATCH` integers. Author, date, homepage, and usage are the constants in Implementation Notes.
12. **Border.** Take the lines of rule 11. When the download URL is empty and the homepage is non-empty, print the first twelve of those source lines (through the second blank after the homepage). When both are empty, print the first eleven. When the download URL is non-empty, the source list uses `Installation command:` and `    cargo install rust3d` in place of the extra blanks, and the print stops before the final blank. Pad every printed line on the right to the longest printed line. The top and bottom borders are that width plus four asterisks. Each body row is `* `, the padded line, ` *`. One all-space body row sits under the top border and one sits above the bottom border.
13. **No install channel on this page.** The product download URL **MUST** stay empty while no Active install requirement names a channel. The live about page **MUST NOT** contain `curl -fsSL`. The box **MAY** render that install line only when a caller passes a non-empty URL. This product’s about caller **MUST NOT** pass one. Cargo install stays on rows 64–67. It is not a line of this page.
14. **No privilege and no network.** Building the page **MUST NOT** call `sudo`, wrap `apt` or `dnf`, create an account, open a socket, or recommend `a root package command` or `sudo curl | sh`. Git Bash and Windows cmd **MUST NOT** invoke Termux `pkg`.
15. **No Python toolchain probe.** The host check **MUST NOT** read pyenv, conda, `PYENV_ROOT`, `CONDA_PREFIX`, or a Cython SOABI. Those reads belonged to the covered Python page.
16. **Rust toolchain locations.** `rustc`, `cargo`, and `rustup` are located with a PATH search. A missing command leaves an empty value. The page does not install them.
17. **Run lines.** The username segment is the `Current User` value. The pid segment is the `PID` value. `{APP_NAME}` is `rust3d`.
    - **Home.** Environment `HOME` when it is set and non-blank. Otherwise there is no home.
    - **Preferred leaf.** When the username is non-empty, `cache-{APP_NAME}-{username}-{pid}`. When it is empty, `cache-{APP_NAME}-{pid}`.
    - **Preferred path.** `/dev/shm/cache/` plus the preferred leaf.
    - **1st fallback.** `/tmp/cache/` plus the preferred leaf.
    - **2nd fallback.** `{home}/.cache/cache-{APP_NAME}-{pid}` when home exists. No username in that leaf. Empty when home does not exist. The line stays.
    - **Used.** The preferred path when `/dev/shm` is a directory. Otherwise the 1st fallback when `/tmp` is a directory. Otherwise the 2nd fallback. Building the page **MUST NOT** call `os.mkdir` or `os.makedirs`.
    - **Persistence storage.** `{home}/.local/{APP_NAME}` when home exists. Empty when it does not. The line stays.
18. **No virtualenv booleans.** The page **MUST NOT** print venv, pyenv, or conda flags. Debug mode stays on `requirement-rust-cli-logging`.
19. Actor / role / subject / approver: **considered**. No dest machine. No approver. The table stays on `requirement-class-software-dev.md`.
20. Dest fence conditions: **considered — none**. Do not invent one.
21. The host-check methods named in `requirement-rust-oop` **MUST** be methods of struct `CheckSystem` in `src/about.rs`. This file still owns the line text. Struct `AboutPage` in `src/about.rs` is the about composer. It **MUST** call `CheckSystem` for the check block and for `self_location`. `Cli` writes `CheckSystem(...)` and then `AboutPage(...)`. `AboutPage` **MUST NOT** construct `CheckSystem`.

### 2.1 Sample shape

Values in angle brackets are the live read. They are not a frozen login or home path.

```text
rust3d 1.0.1
Domain: Build a glTF model and an HTML viewer from outline images in a folder
Runtime tools: none
Entry points: rust3d

2026-10-11 11:16:23.700590 rust3d (v1.0.1)  [CHECK SYSTEM]:
  Now checking your operation system!
    Rustc: rustc 1.89.0 (example)
    C Library: GNU libc
    Operation System: <pretty name>
    Architecture: <arch>
    Current User: <login>
    Shell: /bin/bash
    Rustc Executable: <rustc path>
    cargo location: <cargo path>
    rustup location: <rustup path>
    Inside docker container: false
    Target triple: <host triple>
    Binary Type: <arch>-GNU libc
    Location: <program path>
    PID: <pid>
    Cache folder used: /dev/shm/cache/cache-rust3d-<login>-<pid>
    Cache folder (preferred): /dev/shm/cache/cache-rust3d-<login>-<pid>
    Cache folder (1st fallback): /tmp/cache/cache-rust3d-<login>-<pid>
    Cache folder (2nd fallback): <home>/.cache/cache-rust3d-<pid>
    Persistence storage: <home>/.local/rust3d
    TTY / Interactive: <yes or no>

***************************************************
*                                                 *
* rust3d (1.0.1) by Wilgat Wong on 2026-10-11  *
*                                                 *
* You are using an UNINSTALLED version, location: *
*     <program path>                              *
*                                                 *
* Basic Usage:                                    *
*     rust3d model                          *
*                                                 *
* Please visit our homepage:                      *
*     "https://github.com/cloudgen/rust3d"     *
*                                                 *
*                                                 *
*                                                 *
***************************************************
```

The version string in the sample is the package version at the time of writing. A later package version **MUST** appear instead. The stamp **MUST** be the clock at the moment the page is built. `<login>`, `<pid>`, `<home>`, and `<yes or no>` are the live reads. They are not a frozen login or home path. When `/dev/shm` is not a directory, `Cache folder used` is the 1st fallback or the 2nd fallback under rule 17. The sample shows the case where used equals preferred.

Invocation: `rust3d` on a terminal, then `6`, then `63`. The same page is `rust3d about` with no terminal.

### 2.2 Implementation Notes (this project)

| Item | Value |
|------|--------|
| **Page builder** | Struct `AboutPage` in `src/about.rs` (`framework_about`) |
| **Host check** | Struct `CheckSystem` in `src/about.rs` |
| **Pyenv paths** | Rule 16. Not a separate requirement |
| **Conda paths** | Rule 15. Not a separate requirement |
| **Star box** | `AboutPage.about_box_lines` calls `CheckSystem` for `self_location` |
| **Menu hook** | Row 63 and `rust3d about` use `AboutPage.framework_about` |
| **APP_NAME** | `rust3d` |
| **Console script** | `rust3d` |
| **Author** | `Wilgat Wong` |
| **Homepage** | `https://github.com/cloudgen/rust3d` |
| **About date** | `2026-10-11` (`LAST_UPDATE`). This is the box date, not the host-check stamp |
| **Usage line** | `rust3d model` (`Cli.BASIC_USAGE`) |
| **Download URL** | empty (`Cli.DOWNLOAD_URL`) |
| **Docker marker** | `/.dockerenv` only |
| **PID** | `std::process::id()` |
| **Cache folders** | Preferred `/dev/shm/cache/…`, 1st `/tmp/cache/…`, 2nd `{home}/.cache/…` with no username. Used follows rule 17. The page does not create them |
| **Persistence storage** | `{home}/.local/rust3d` |
| **TTY / Interactive** | `yes` or `no` from whether stdout is a terminal |
| **JSON** | Not claimed. No `--json` stream |
| **Privilege** | normal user privilege |
| **Product version** | `1.0.1` |

### 2.3 Why This Requirement Exists (Direct CIAO Alignment)

- **CIAO Principle 5 – SSOT** (https://github.com/cloudgen/ciao): one page owns the about lines and the reads that fill them.
- **CIAO Principle 2 – Intentional** (https://github.com/cloudgen/ciao): each label names one read. The install kind is three named sentences.
- **CIAO Principle 1 – Caution** (https://github.com/cloudgen/ciao): missing tools stay blank. The page does not run interpreters or open the network.
- **CIAO Principle 16 – Interactive** (https://github.com/cloudgen/ciao): a person opens the page from the menu. `--version` stays the short line.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, **admin privilege** and **dedicated system user privilege** stay unused. **This requirement:** building the about page **MUST NOT** call `sudo`, wrap `apt` / `dnf`, create a dedicated account, or recommend `a root package command` or `sudo curl \| sh`. Git Bash and Windows cmd **MUST NOT** invoke Termux `pkg`. The page only reads the local interpreter, environment, and files.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** A missing `rustc`, `cargo`, or `rustup` is an empty location, not an error. The cache lines name a private leaf. The page does not search a shared cache-purpose folder.
- **Intentional:** Field order and the three install sentences are named.
- **Anti-fragile:** A source checkout, a home install, and a `/usr` install each get a sentence from the same path rules.
- **Over-protect (Principle 20):** Do not add a second about page, and do not print a `curl | sh` line while the download URL is empty.

## 4. Protection Rule (Sacred)

**Future AI assistants or maintainers MUST NOT:**

- Drop a host-check label or change its order.
- Replace `shutil.which` with a subprocess that runs `python2`, `python3`, `conda`, or `pyenv`.
- Report `{root}/libexec/pyenv` as `pyenv location` when `{root}/bin/pyenv` exists.
- Report `{root}/condabin/conda` as `conda location` when `{root}/bin/conda` exists.
- Let an active conda prefix borrow a base `python2` or `python3`.
- Probe containers by anything other than `/.dockerenv`.
- Show GLOBAL, LOCAL, or UNINSTALLED for a path the rules in this file classify differently.
- Put `curl -fsSL` on the live about page while the download URL is empty.
- Put `cargo install` or a live `ffmpeg` PATH probe on this page.
- Turn this page into `--version` or into a JSON object.
- Add `--json` for this page.
- Split the package version into `MAJOR`, `MINOR`, and `PATCH` for the star-box title.
- Emit the `[CHECK SYSTEM]:` block through StatusLog.
- Print `in_venv`, `in_pyenv`, or `in_conda`, or use those booleans to choose a location line.
- Name `py-tui` on the page.
- Freeze a Unix login or a `/home/<login>/…` path into this requirement.
- Drop `PID`, the four cache-folder lines, `Persistence storage`, or `TTY / Interactive`, or print them before `Location`.
- Create the cache directories or the persistence directory while building this page.
- Report `/dev/shm`, the process temp directory, or the home `.cache` directory as the cache folder. The four cache lines name the private leaves in rule 17.
- Treat a label-only check of `Cache folder used` as proof the leaf survived. The proof must include `cache/cache-rust3d-{login}-{pid}` on the preferred and 1st paths, and `cache/cache-rust3d-{pid}` on the 2nd path.
- Run `id` to fill `Current User` or a cache leaf.
- Construct `CheckSystem` inside `AboutPage`.

## 5. Design-time verification

| ID | Suite | Status |
|----|-------|--------|
| TP-ABOUT-01 | `tests/product.rs` | have |
| TP-ABOUT-04 | `tests/product.rs` | have |
| TP-ABOUT-06 | `tests/product.rs` | have |
| TP-ABOUT-13 | `tests/product.rs` | have |
| TP-ABOUT-14 | `tests/product.rs` | have |
| TP-ABOUT-05 | `tests/product.rs` | n/a |
| TP-ABOUT-09 | `tests/product.rs` | n/a |
| TP-ABOUT-10 | `tests/product.rs` | n/a |
| TP-ABOUT-11 | `tests/product.rs` | n/a |
| TP-ABOUT-12 | `tests/product.rs` | n/a |
| TP-ABOUT-16 | `tests/product.rs` | n/a |
| TP-ABOUT-02 | `tests/product.rs` | todo |
| TP-ABOUT-03 | `tests/product.rs` | todo |
| TP-ABOUT-07 | `tests/product.rs` | todo |
| TP-ABOUT-08 | `tests/product.rs` | todo |

There is no TP-ABOUT-15. This product does not claim `--json`.

TP-ABOUT-01 asserts the identity lines, the rustc and cargo labels, a rectangular star box, and that the live page has no pyenv, conda, Cython, or `Python `. TP-ABOUT-04 asserts an explicit download URL renders `cargo install rust3d`, and that an empty URL omits that line. TP-ABOUT-06 asserts a checkout path is uninstalled and `/usr/bin/rust3d` is global. TP-ABOUT-13 asserts the four cache lines and persistence, and that building the page does not create those directories. TP-ABOUT-14 asserts that `Cache folder used` is the preferred leaf when `/dev/shm` is a directory, and that the 2nd leaf is `cache-rust3d-{pid}` with no username. TP-ABOUT-05, TP-ABOUT-09, TP-ABOUT-10, TP-ABOUT-11, TP-ABOUT-12, and TP-ABOUT-16 are n/a: rules 15, 16, and 18 forbid a Python, pyenv, or conda probe. TP-ABOUT-02, TP-ABOUT-03, TP-ABOUT-07, and TP-ABOUT-08 stay todo.

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`.

## 6. Related artifacts

| Artifact | Role |
|----------|------|
| `docs/requirements/index.md` | Registry |
| `docs/requirements/requirement-domain-rust3d.md` | Domain sentence on the identity block |
| `docs/requirements/requirement-rust-tui.md` | Result page; scroll; row 63 |
| `docs/requirements/requirement-rust-oop.md` | Struct `CheckSystem` owns the host-check reads. Struct `AboutPage` owns the composer |
| `docs/requirements/requirement-rust-cli-logging.md` | `in_venv`, `in_pyenv`, and `in_conda` read the one logger. They are not printed |
| `docs/requirements/requirement-rust-cli-interface.md` | `--version` stays the short human line. The verb `about` is named there |
| `docs/requirements/requirement-rust-cli-language.md` | The about page stays English. The leaf short `about` stays English |
| `docs/requirements/requirement-rust-packaging.md` | The version string in the title and the stamp |
| `docs/requirements/requirement-runtime-prerequisites.md` | Image stack is cargo crates. This page does not probe it. Runtime tools stay `none` |
| `docs/requirements/requirement-class-software-dev.md` | Approver none; no dest fence |
| `src/about.rs` | Struct `AboutPage`. `framework_about` |
| `src/about.rs` | Struct `CheckSystem` |
| `Cargo.toml` | Homepage URL and the version string |

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### App-name-version display

**Definition:** **App-name-version-display** is the operator-facing **identity token** printed when a CLI **names the running program** on a human banner: live Config `APP_NAME` immediately followed by parenthesized live Config `VERSION`, with **no space**. The name is **bold**; the version (edition string; test helpers may bind it as `APP_VERSION`) is *italic*. Markdown written form: `**rust3d**(*the live product version*)`. It is **not** README H1 app-name, **not** a filesystem path, **not** an invocation/`Next:` command line, and **not** JSON (those keep separate `app` / `version` keys). Live scalars stay Config; this term owns the **print form** and **when** that form is required.

**Human daily-life explanation:**

**App-name-version-display** is the nametag on the program when it introduces itself: name and edition together, so you do not have to run a second “what version?” question to know which copy is talking.

**Daily-life example:** A café nametag that prints the shop name in **bold** and the edition in *italics* — **Harbor Kettle**(*2024*) — not only “Harbor Kettle,” and not a street address or the order slip that still says “Harbor Kettle, table 3.”

### CLI about

**Definition:** **CLI about** is the operator page of the well-known operational verb `about`. It prints **live** facts for this run: which program this is, what this computer is, and where the program is running from. The page is three blocks in this order: identity, host check, install location. A missing host-check value stays a blank on a line that remains. The check reads the environment and path existence. It does **not** spawn `python2`, `python3`, `conda`, or `pyenv`, and it does **not** freeze a login, a home path, or a display id into the page. When the well-known menu claims the row, the number is **83** under self-management. The shell picture is CLI about shell example. The Python picture is CLI about Python example. A specialized product requirement owns extra identity lines and the live reads.

**Human daily-life explanation:**

**CLI about** is the card the tool shows when you ask “what are you, and which machine are you on?” It names the program, the computer, and the drawer it is running from. It does not install itself, and it does not go look those facts up by starting other programs.

**Daily-life example:** You ask the kitchen timer “about.” It says which model it is, that it is on the counter, and which drawer it came from. It does not start the oven to answer.

### Proper project version bump

**Definition:** A **proper project version bump** is a **user-ordered semver increase** of the product ship version that is **strictly newer** than the previous local ship version, applied in the **same change** on every in-scope surface. When `Cargo.toml` exists, `package.name` is the **app and package name** and `package.version` **equals** the import-package `VERSION`, the ship-unit version, the README Version badge (project target version), and the current CHANGELOG heading. A bump is **not** a side effect of editing a requirement, a skill, or a glossary file. Agents **MUST NOT** invent a second version literal beside the one the user ordered.

**Human daily-life explanation:**

A **proper project version bump** is printing one newer edition number on every copy of the book in the same sitting, because someone asked for a new edition. Editing a workshop note does not print a new edition by itself. When the catalog card exists, the name on the card is the product name and the number on the card is the same number as the title page.

**Daily-life example:** The kitchen card says name `pancake-timer` and edition `1.2.0`. The title page, the cover sticker, and the change list all say `1.2.0`. The next ordered edition is `1.2.1` on all of those surfaces together. A note in the workshop drawer does not become `1.2.1` on its own.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-04 | Active 1.0.0 | About page: identity, host check, star box. Pyenv and conda reads stay here. No `--json`. Product version **1.0.5**. `./tests/run.sh` — 23 tests, OK. `TP-ABOUT-01` through `TP-ABOUT-14` and `TP-ABOUT-16` are have. There is no `TP-ABOUT-15` |
| 2026-10-05 | Active 1.0.1 | Homepage is `https://github.com/cloudgen/rust3d` |
| 2026-10-05 | Active 1.0.2 | Product version in the sample and the notes is **1.0.1** |
| 2026-10-05 | Active 1.0.3 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-11 | Active 1.0.4 | Host-check sample matches the Rust labels. Cache lines stay the private leaves. `TP-ABOUT-13` and `TP-ABOUT-14` assert the leaf suffix. Python toolchain probes are n/a |
| 2026-10-14 | Active 1.0.5 | About is menu row 63 under self-management row 6. Cargo rows on the menu are 64–67. Product version stays **1.0.1** |
**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
