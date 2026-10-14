**file**: docs/requirements/requirement-rust-cli-interface.md
**Status**: Active (Version 1.1.8 – Menu row 7 owns language. Self-management children are 62–67. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-cli-interface`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Define the official command-line entry points, empty-argv behavior, and typed verbs for the rust3d Python package.

The text-menu picture is `requirement-rust-tui`. Outline steps and formats are `requirement-domain-rust3d`. The about page is `requirement-rust-about`. The domain sentence stays on the domain file. The logger construct is `requirement-rust-cli-logging`. Struct homes are `requirement-rust-oop`. The encoder file is Retired.

`fn main` writes `StatusLog::new` and then constructs `Cli`. The argument parser runs after that construct and after the debug block. The allowed end state is this file.

### 1.1 Human-facing

**In one sentence:** `rust3d` with no words opens the numbered menu on a terminal, and with no terminal it prints help and stops.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Person at the keyboard or in a script | `rust3d` or `rust3d model` |
| The other role | The menu picture and the outline facts | `requirement-rust-tui`, `requirement-domain-rust3d` |
| Not this file | How the frame is drawn, and how one image becomes an outline | TUI and the domain file |

| Includes | Excludes |
|----------|----------|
| Console script, module entry, typed verbs, empty-argv menu or help | A shell installer on empty argv |
| `model`, an optional folder, and `--views`, `--grid`, and `--output`, also owned by the domain file | A numbered row for `help` |
| Cargo verbs `version-check`, `self-update`, `self-install`, `self-uninstall`, also owned by the text menu | `sudo` cargo, or a downloaded script piped into a shell |

| Surface | What you open | What for |
|---------|---------------|----------|
| `rust3d` on a terminal | Front board | Menu. Does not convert a folder |
| `rust3d` with no terminal | Help text | Exit 0 |
| `rust3d model` | The terminal | Convert one folder. No menu and no prompt |
| `rust3d help` | Help text | Verb list. Does not draw the menu |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Start on a terminal | The front board opens. Pip does not run. | `rust3d` |
| Start in a script | Help text, exit 0. | `rust3d` |
| Convert a folder | `model.glb` and `viewer.html` in that folder's `model` directory. No menu. | `rust3d model` |
| Check or change the install | Pip runs only when you ask. Empty argv does not. | `rust3d version-check` |

## 2. Core Rules (Mandatory)

### 2.1 Entry points

1. **MUST** expose a console script entry named `rust3d` pointing at `rust3d.cli:main` (declared in packaging SSOT).
2. **MUST** support module execution: `cargo run`.
3. **MUST** keep `fn main` in `src/cli.rs` as the single runtime entry. `main` writes `StatusLog::new` before it constructs `Cli` (`requirement-rust-cli-logging`, `requirement-rust-oop`).
4. **MUST NOT** require root or sudo to run the CLI.

### 2.2 Empty argv

5. On a terminal, bare invocation (`rust3d` with no args, or `cargo run` with no args) **MUST** open the front board in `requirement-rust-tui`. It **MUST NOT** convert a folder. It **MUST NOT** run cargo.
6. When stdout is not a terminal, bare invocation **MUST** print help and return 0. It **MUST NOT** draw the text screen and **MUST NOT** wait.
7. **MUST NOT** default empty argv to a shell-channel install or to self-update.

### 2.3 Typed verbs

8. `ArgumentParser.parse_args` **MUST** run after `StatusLog::new` and after the debug block.
9. The typed verbs are `help`, `version`, `about`, `model`, `self-install`, `version-check`, `self-update`, and `self-uninstall`. Each one is operational. **MUST NOT** add a verb whose only purpose is a test. **MUST NOT** add `hello`, `join`, or `list-videos`. `language` is not a typed verb. Menu row 7 owns it (`requirement-rust-cli-language`). **MUST NOT** add `language` to `PRODUCT_VERBS`.
10. A folder operand and `--views`, `--grid`, and `--output` are legal only on `model`. On any other verb they **MUST** be an error. `model` **MUST NOT** scan subfolders for images.
11. On a terminal, `model` **MUST NOT** draw the text screen and **MUST NOT** prompt. `about` **MUST** open the about page.
12. With no terminal, `about` **MUST** print the about page and return 0. It does not draw the text screen. `model` **MUST** convert and return the code from `requirement-domain-rust3d`. It does not draw the text screen.
13. `help` and `model` **MUST NOT** draw the text screen. `help` returns 0. `model` with no folder converts the current directory. The default format is png. Menu row 1 opens the folder board and does not run this verb by itself.
14. `version` **MUST** print `VERSION` and **MUST NOT** call cargo and **MUST NOT** draw the text screen.
15. `version-check`, `self-update`, `self-install`, and `self-uninstall` **MUST NOT** open the text screen. The commands are:

| Verb | Command |
|------|---------|
| `version-check` | `cargo index versions rust3d` |
| `self-update` | `cargo install --upgrade rust3d` |
| `self-install` | `cargo install rust3d` |
| `self-uninstall` | `cargo uninstall -y rust3d` only when `--force` is also present |

`self-uninstall` without `--force` **MUST** print the next step and return non-zero. The package stays installed. These commands **MUST NOT** use `sudo` and **MUST NOT** use `curl`. Empty argv **MUST NOT** run `self-install` or `self-update`.

Every typed verb is named again by its topic owner. Help text is not that second mention.

| Verb | Second mention |
|------|----------------|
| `help`, `model`, `version` | This file for `help`. `model` is also pillar A of `requirement-domain-rust3d`. `version` is also row 62 on `requirement-rust-tui` |
| `about` | `requirement-rust-about` and row 63. The domain sentence stays on pillar D of `requirement-domain-rust3d` |
| `self-install`, `version-check`, `self-update`, `self-uninstall` | Rows 64–67 on `requirement-rust-tui` |

`language` is not in this table. The second mention of that ban is `requirement-rust-cli-language`.

Complete invocation samples:

```text
rust3d
rust3d help
rust3d model
rust3d model photos --views views.json --grid 96
rust3d version
rust3d about
rust3d self-install
rust3d version-check
rust3d self-update
rust3d self-uninstall --force
cargo run
```

### 2.4 Model build

These rules are the `model` verb. The domain file owns the image steps, the folder board, and the output name. They are not what empty argv does.

16. **MUST** accept an optional folder. When it is omitted, the folder is the current directory.
17. **MUST** accept `--views`, `--grid`, and `--output` as the domain file defines them. **MUST NOT** accept `--format`.
18. **MUST NOT** prompt. **MUST NOT** draw the text screen.
19. A missing folder, a bad grid, or a bad views file **MUST** return 1 with a next-step line.
20. A folder with no supported image **MUST** return 0 and **MUST NOT** import the image stack.
21. One failed image **MUST** be reported. The remaining images still run. Any failure makes the exit code 1.
22. From a checkout, `cargo run -- model` **MUST** call the same conversion. There is no `convert` binary.
23. When `model` is about to convert images, this caller **MUST** print the waiting sentence from `requirement-domain-rust3d` and flush before the blocking call. The choice token is `model` for the verb. Returned lines still start with the sentences that were shown. This caller **MUST** print only the remainder, so each sentence appears once. There is no download sentence. `help`, `version`, `about`, and the cargo verbs **MUST NOT** announce. The domain file owns the words.

Menu row 1 asks which folder. That question is `requirement-rust-tui`. Esc there returns to the front board and does not convert.

### 2.5 Flags

24. The parser accepts the verbs in rule 9. `help` is also available as `--help`.
25. This product does not claim `--json`. **MUST NOT** add it in this file.
26. **MUST NOT** add `--debug`. Debug status is the environment gate on `requirement-rust-cli-logging`.
27. **MUST NOT** add a second pair of input paths. The one folder operand belongs to `model` only.

### 2.6 Output behavior

28. Outline progress and failures **MUST** be visible to the operator. On the text screen they are the result page. Off the text screen they are console lines.
29. Durable status **MUST** go through StatusLog once `main` has constructed it (`requirement-rust-cli-logging`). The console failure sentence remains required.
30. **MUST NOT** mix a machine-only JSON mode into the session. This product does not claim `--json`.

### 2.7 No encoder

31. The front board **MUST** be allowed to open when `ffmpeg` is missing. This product does not look for it.
32. `model` **MUST NOT** spawn an encoder. A missing image library fails only when the folder has a supported image (`requirement-runtime-prerequisites`).
33. The about page runtime-tools line is `none` (`requirement-rust-about`). It does not probe `PATH`.

### 2.8 Implementation Notes (this project)

| Item | Value |
|------|--------|
| **Console script** | `rust3d = rust3d.cli:main` |
| **Binary entry** | `src/bin/rust3d.rs` → `cli::main()` |
| **CLI module** | `src/cli.rs` defines `Cli` and `fn main` in the allowed end state |
| **Empty argv, terminal** | Front board. No conversion. No cargo |
| **Empty argv, no terminal** | Help text. Return 0 |
| **Parser** | After StatusLog and the debug block |
| **Quiet / JSON** | Text screen sets `is_quiet=True` on the logger construct. No `--json` |
| **Privilege** | user-level only |
| **Model** | `Cli._verb_model` calls `convert_folder`. It prints the waiting sentence and flushes, then prints only the remainder. It does not draw the menu |
| **Product version** | `1.0.1` |
| **User docs** | Root `README.md` Usage matches this contract |

### 2.9 Why This Requirement Exists (CIAO)

- **Principle 2 – Intentional**: Entry, empty argv, and each typed verb are explicit.
- **Principle 16 – Interactive awareness**: A terminal opens the menu. No terminal prints help and returns 0.
- **Principle 5 – SSOT**: Verb names live here and on the topic owner. The picture lives on the TUI requirement.
- **Principle 10 – Least privilege**: Pip verbs do not use sudo.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, the person runs `rust3d` as the normal user. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to start the program or to run the cargo verbs. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** A missing folder does not write an outline. An empty folder does not load the model.
- **Intentional:** Empty argv is the menu on a terminal and help off a terminal.
- **Anti-fragile:** Module entry and the console script both call `main`.
- **Over-protect:** Empty argv does not install, update, or convert a folder.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Change empty argv into a shell-channel install.
2. Make empty argv convert a folder or run cargo.
3. Make empty argv off a terminal exit non-zero. That path prints help and returns 0.
4. Remove the console script or the module entry without a packaging and docs update.
5. Require root to run `model`.
6. Add `hello`, `join`, or `list-videos` without a domain-law change.
7. Add `--json`, `--debug`, or `--format` in this file.
8. Treat help text as the second mention of a verb.
9. Add `language` as an argv verb. Row 7 is the menu language.
10. Start `model` image work without the domain waiting sentence, print that sentence a second time after it was already shown, or announce a download when the weights file is already a file.

**Violating this rule is a critical CLI regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `rust3d` entry declared in packaging |
| AC-2 | `cargo run` reaches `main` |
| AC-3 | Empty argv on a terminal opens the front board and does not convert and does not run cargo |
| AC-4 | Empty argv with no terminal prints help and returns 0 |
| AC-5 | `model` does not draw the menu. A folder operand on another verb is an error |
| AC-6 | An empty folder returns 0. A missing folder returns 1 |
| AC-7 | Pip verbs match the command table and do not use sudo or curl |
| AC-8 | `model` prints the domain waiting sentence and flushes before the work, then prints only the remainder. `version` does not announce |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-tui` | Menu picture and numbered rows |
| `requirement-rust-cli-language` | `language` is not an argv verb. Menu row 7 |
| `requirement-domain-rust3d` | Outline steps, the folder board, the about domain sentence, and the waiting sentences |
| `requirement-rust-about` | The about page |
| `requirement-rust-cli-logging` | Construct before the parser |
| `requirement-rust-oop` | `Cli` and `fn main` |
| `requirement-video-ffmpeg-pipeline` | Retired. This file does not call an encoder |
| `requirement-rust-packaging` | Console script name |
| `requirement-rust-error-handling` | Fail messaging |
| `requirement-runtime-prerequisites` | Image stack. No encoder check |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-CLI-01** | `tests/test_cli.rs` | todo | Empty argv on a tty opens the front board |
| **TP-CLI-02** | `tests/test_model.rs` | have | A folder operand on `version` is an error |
| **TP-CLI-03** | `tests/test_model.rs` | have | An empty directory returns 0 and does not import the image stack |
| **TP-CLI-04** | `tests/test_cli.rs` | optional | Empty argv off a tty prints help and returns 0 |
| **TP-TUI-07** | `tests/test_model.rs` | have | Typed `model` does not open the text screen |
| **TP-CLI-05** | `tests/test_model.rs` | have | `model` flushes the waiting sentence once before the session. `version` does not announce |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### CLI

**Definition:** A **CLI** (command-line interface) is a product whose **primary operator surface** is a **short-lived argv program**: the login user runs a command with arguments, it does the work, and it **exits**. The word is **language-agnostic**. Type 0 / Type 1 / Type 2 privilege layers, a CLI dispatcher, and CLI routed-verbs live **inside** a CLI. It is **not** a long-running HTTP server and **not** a GUI.

**Human daily-life explanation:**

A **CLI** is a program people run with a short command line: it does the work and **exits**. Privilege layers, the waiter, and live verbs live **inside** it. It is not a website that stays up and not a windowed app.

**Daily-life example:** You type a one-liner, the tool hangs the picture, then it is gone. That come-work-leave program is a **CLI**.

### Python CLI verb default route

**Definition:** **Python CLI verb default route** is the rule that, for a Python CLI, each routed verb's default route is the **terminal**: standard output, standard error, a line interface, and a scriptable run, with no text screen. A verb uses the text screen only when a specialized requirement explicitly names that verb as using the screen. The default exists so a command-line test can drive the verb without a screen. This term does not choose which tokens the dispatcher accepts. It does not move the no-command interactive entry off the text screen.

**Human daily-life explanation:**

A **python-cli-verb-default-route** is the shop rule for a named dish. Naming a dish sends the order to the plain counter, where a written slip can be checked without standing at the walk-up board. The board is still there when you name no dish. A dish stays at the board only when the card already says that dish is served there.

**Daily-life example:** You write one dish name on a slip. The **python-cli-verb-default-route** is the plain counter, so a checker can read the slip without the board. The card already says one named dish is served at the board. That dish stays at the board. Every other named dish stays on the **python-cli-verb-default-route**.

### Major text-screen entry

**Definition:** **Major TUI entry point** is the rule that the major entry of a claimed text screen is interactive mode with **no positional command token**. Flags that are not a command (`--debug`, `--quiet`, `--force` with no command, and peers of that kind) stay on that entry. `--json` with no command stays JSON help, not the text screen. Creating or keeping a text screen does not attach every CLI verb to that screen. A verb opens the screen only when a specialized requirement explicitly names that verb as using the text screen. This term does not choose the look of the board, the width of a glyph, or which tokens the dispatcher accepts.

**Human daily-life explanation:**

A **major-tui-entry-point** is the walk-up board at a shop. You see that board when you name no dish. Naming a dish sends you to a different counter. A light switch that is not a dish (`--debug` and the same kind of flag) still leaves you at the walk-up board. Asking for the paper list with no dish (`--json` alone) is the paper list, not the board.

**Daily-life example:** You walk in and name no dish. The **major-tui-entry-point** is that board. You walk in and say only "more light." You are still at the **major-tui-entry-point**. You name a dish. That order is not the **major-tui-entry-point**. It stays at the board only when the card already names that dish for the board. Every other named dish goes to the plain counter.

### Command line for normal user only

**Definition:** A **command line for normal user only** is a POSIX-like shell environment whose **privilege ceiling is Type 0** (**normal user privilege**). Typical instances: **Termux**, **Git Bash**, and **Windows cmd** (Windows Command Prompt / Windows command-line). There is no usable root/`sudo` host mutate and no dedicated system account for this login. When the ship unit **detects** this class (or detects Termux, Git Bash, or Windows cmd), it **MUST NOT implement or enable** **admin privilege** (Type 1) or **dedicated system user privilege** (Type 2): no in-tool `sudo`, no `apt`/`dnf` wrap, no `/etc` dest, no `useradd`, no LPU switch.

**Formal name (people-facing):** Command line for normal user only  
**Workshop word (keep):** command-line-for-normal-user-only

**Human daily-life explanation:**

A **command line for normal user only** is a keyboard that only has **your** keys. Termux on a phone, Git Bash on Windows, and Windows cmd / Command Prompt are this kind of room: you can tidy your own drawer; you cannot borrow the building site key or put on a dedicated-operator badge.

**Daily-life example:** On Termux you run `pkg` as yourself. On Git Bash you run `git` as yourself. On Windows cmd you type as yourself. None of those rooms should grow a `sudo apt` or a hidden system-user switch.

### Rust crates.io CLI

**Definition:** A **Rust crates.io CLI** is a **software-development** product whose primary operator surface is a **short-lived console script** installed from **Cargo**: `Cargo.toml` is the packaging SSOT, ship source lives under **`src/`**, and the operator runs the **declared console entry** (not a long-running HTTP process). The default coding shape is **thin `src/cli.rs`** plus a **StateLogic + Attr** orchestrator (OOP **L3**). It **does not** use `backend/` + `frontend/` as the ship tree.

**Human daily-life explanation:**

A **rust-crates-cli** is a short-lived console command installed from Python packaging: `Cargo.toml`, source under `src/`, declared entry. It is not a backend/frontend diner.

**Daily-life example:** A mixer you install from a catalog listing, with a named “on” switch—not a café that stays open with a dining room.

This product's package name is `rust3d` (`Cargo.toml` `package.name`), the import package is `rust3d` under `src/`, and the console script is `rust3d`.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-08-09 | Active 1.0.0 | Initial Python CLI interface law for rust3d |
| 2026-10-04 | Active 1.1.0 | Empty argv opens the text menu on a terminal and prints help off a terminal. Typed verbs. Join questions are the `join` verb |
| 2026-10-04 | Active 1.1.1 | `language` is not an argv verb. Menu row 4 owns it |
| 2026-10-04 | Active 1.1.2 | Verb `about` is `requirement-rust-about`. Rule 33 no longer claims a live ffmpeg probe. Product version **1.0.5** |
| 2026-10-05 | Active 1.1.3 | `model` and `cargo run --bin convert` print the domain waiting sentence and flush before the work, then print only the remainder |
| 2026-10-05 | Active 1.1.4 | Product version **1.0.1** |
| 2026-10-05 | Active 1.1.5 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-05 | Active 1.1.6 | Terminologies name the package `rust3d`, matching `package.name`. The console script stays `rust3d`. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.1.7 | Checkout menu is `cargo run`. Checkout conversion is `cargo run -- model`. There is no `convert` binary. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.1.8 | Menu row 7 owns language. Version is row 62. About is row 63. Cargo rows are 64–67. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
