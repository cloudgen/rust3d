**file**: docs/requirements/requirement-rust-dependency-management.md
**Status**: Active (Version 1.0.3 – Terminal writer `crossterm>=0.28`. The menu painter stays in this package. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-dependency-management`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

This file owns the **cargo requirement strings** rust3d declares. The machine copy is `Cargo.toml` `[dependencies]`. The two lists are one set. Packaging law owns the manifest shape. Runtime-prerequisite law owns the fact that no host encoder is required. How status lines are written is `requirement-rust-cli-logging`. The model build is `requirement-domain-rust3d`.

The timestamp crate is `chrono>=0.4`. The terminal writer is `crossterm>=0.28`. The outline crates are `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2`. Each of those entries has a version floor. The text menu is drawn by struct `MenuPainter` in this package. `crossterm` is the terminal session, not a menu framework.

### 1.1 Human-facing

**In one sentence:** Before a model can run, cargo must be able to fetch `chrono>=0.4`, `crossterm>=0.28`, `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2`, as written in `Cargo.toml`. StatusLog is in-tree.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Install the declared wheels with cargo | `cargo install rust3d` |
| The other role | The manifest that lists those strings | `Cargo.toml` |
| Not this file | The menu picture and the outline steps | TUI and the domain file |

| Includes | Excludes |
|----------|----------|
| Version floors for StatusLog, the terminal writer, and the image stack | A host encoder, a GUI OpenCV wheel, and a menu framework crate |
| One list shared by the manifest and this file | A second `requirements.txt` authority |

| Surface | What you open | What for |
|---------|---------------|----------|
| `Cargo.toml` | `[dependencies]` | The live strings |
| `rust3d` | console script | Uses those libraries after install |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Install the package | You get the status logger and the image stack. An empty folder still does not import the image stack. | `cargo install rust3d` |
| Install the vision wheel | Outlines decode with the `image` crate. That decode does not open a window. | `cargo add serde_json` |

## 2. Core Rules (Mandatory)

1. **MUST** declare runtime cargo crates only in `Cargo.toml` `[dependencies]`.
2. **MUST** attach a a Cargo version requirement version specifier to every entry. A bare name is not a declaration.
3. **MUST** keep the strings in Implementation Notes identical to that list.
4. **MUST** use the headless OpenCV wheel for silhouette masks. **MUST NOT** declare the GUI wheel `opencv-python`.
5. **MUST NOT** declare a menu framework (`ratatui`, `py-tui`, or `py_tui`). The text menu is painted by this package (`requirement-rust-tui`, struct `MenuPainter`). The live terminal session **MUST** use `crossterm>=0.28`. That crate writes the alternate screen. It does not choose the rows.
6. StatusLog is in-tree in `src/status_log.rs`. It **MUST NOT** be optional and **MUST NOT** be declared as a crate. The timestamp floor stays `chrono>=0.4`.
7. **MUST NOT** declare FFmpeg, or any other host encoder, as a cargo package.
8. **MUST NOT** install operating-system packages, and **MUST NOT** use admin privilege, to satisfy these strings.
9. **MUST NOT** add `requirements.txt` as a second authority.

### 2.1 Implementation Notes (this project)

| Field | Value |
|-------|--------|
| **Status-log spec** | `chrono>=0.4` |
| **Terminal spec** | `crossterm>=0.28` |
| **Array spec** | `image>=0.25` |
| **Vision spec** | `serde_json>=1` |
| **Mesh spec** | `unicode-width>=0.2` |
| **Menu framework** | Not declared. Painter is struct `MenuPainter`. `crossterm` is the terminal session |
| **GUI build forbidden** | `opencv-python` is not declared |
| **Host encoder** | Not declared. Not required |
| **Manifest** | `Cargo.toml` `[dependencies]` |
| **Why this logger floor** | StatusLog is in-tree. `chrono>=0.4` is the timestamp crate. The line shape stays on `requirement-rust-cli-logging` |
| **Why these vision floors** | `image>=0.25` decodes png, jpeg, and webp and does not open a window. The mesh is voxel face extraction in this crate. `serde_json>=1` reads views. `unicode-width>=0.2` counts menu columns. Pillow and rembg are not declared |
| **Empty folder** | A folder with no supported image returns 0 and does not import this image stack (`requirement-domain-rust3d`) |

### 2.2 Why This Requirement Exists (CIAO)

- **Principle 5 – SSOT**: One manifest and one law file for the cargo strings.
- **Principle 2 – Intentional**: The logger floor is required. The vision wheel is the headless build.
- **Principle 1 – Caution**: A bare name can install a GUI build that needs a window library.
- **Principle 10 – Least privilege**: cargo as this login; no root package install.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, these wheels stay a normal-user cargo install. **This requirement:** do not satisfy `chrono>=0.4`, `crossterm>=0.28`, `image>=0.25`, `serde_json>=1`, or `unicode-width>=0.2` with admin privilege, a system package manager, or `a root package command`.

## Sample code

```toml
[project]
dependencies = [
    "chrono>=0.4",
    "crossterm>=0.28",
    "image>=0.25",
    "serde_json>=1",
    "unicode-width>=0.2",
]
```

`opencv-python` is not in this list. There is no menu framework. There is no encoder package.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Pin the vision wheel that imports without opening a window.
- **Intentional:** The text menu stays in this package. StatusLog stays required.
- **Anti-fragile:** Headless OpenCV does not depend on a GL library.
- **Over-protect:** Tests read the manifest and the strings in this file.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Drop the version specifier from any dependency.
2. Replace `the image crate` with `opencv-python` while outlines only draw on a canvas.
3. Add a menu framework while the painter is struct `MenuPainter` in this package. `crossterm>=0.28` stays the terminal session.
4. Declare StatusLog as a crate, or mark that logger optional, while status lines use the in-tree logger. `chrono>=0.4` stays.
5. Add FFmpeg or another encoder as a cargo dependency.
6. Add a root installer for these libraries.
7. Leave this file’s strings different from `Cargo.toml`.
8. Put the cargo strings back into packaging or runtime as a second authority.

**Violating this rule is a dependency-honesty regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `serde_json>=1` is a dependency |
| AC-2 | No menu framework is declared. `crossterm>=0.28` is the terminal session |
| AC-3 | `chrono>=0.4` is a required dependency |
| AC-4 | No dependency entry lacks a version specifier |
| AC-5 | `opencv-python` is not a dependency |
| AC-6 | `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2` are dependencies |
| AC-7 | This file and `Cargo.toml` list the same strings, including `crossterm>=0.28` |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `docs/requirements/index.md` | Registry |
| `requirement-rust-packaging` | Manifest shape; points here for the strings |
| `requirement-runtime-prerequisites` | No host encoder; cargo strings point here |
| `requirement-rust-tui` | Menu writer in this package; no menu wheel here |
| `requirement-rust-cli-logging` | Status logger that needs the StatusLog floor |
| `requirement-domain-rust3d` | The model build that imports the image stack |
| `requirement-class-software-dev` | Residual points here |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-DEP-01** | `tests/product.rs` | have | Every manifest entry has a version specifier. The four strings are the list |
| **TP-DEP-02** | `tests/product.rs` | have | Headless wheel; GUI wheel `opencv-python` absent |
| **TP-DEP-03** | `tests/product.rs` | have | This file matches the manifest |
| **TP-DEP-04** | `tests/product.rs` | have | Installed wheels meet the floors when present. A missing wheel does not fail the case |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Rust crates.io CLI

**Definition:** A **Rust crates.io CLI** is a **software-development** product whose primary operator surface is a **short-lived console script** installed from **Cargo**: `Cargo.toml` is the packaging SSOT, ship source lives under **`src/`**, and the operator runs the **declared console entry** (not a long-running HTTP process). The default coding shape is **thin `src/cli.rs`** plus a **StateLogic + Attr** orchestrator (OOP **L3**). It **does not** use `backend/` + `frontend/` as the ship tree.

**Human daily-life explanation:**

A **rust-crates-cli** is a short-lived console command installed from Python packaging: `Cargo.toml`, source under `src/`, declared entry. It is not a backend/frontend diner.

**Daily-life example:** A mixer you install from a catalog listing, with a named “on” switch—not a café that stays open with a dining room.

This product's package name is `rust3d` (`Cargo.toml` `package.name`), the import package is `rust3d` under `src/`, and the console script is `rust3d`.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-05 | Active 1.0.0 | Pip strings for rust3d 1.0.0. Logger floor `chrono>=0.4`. Build floors `image>=0.25`, `serde_json>=1`, `unicode-width>=0.2`. Pillow and rembg are not declared. No GUI wheel. No menu wheel. No encoder package |
| 2026-10-05 | Active 1.0.1 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-05 | Active 1.0.2 | Terminologies name the package `rust3d`, matching `package.name`. The console script stays `rust3d`. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.0.3 | Terminal writer `crossterm>=0.28`, inherited from the HolaTui text screen. Menu painter stays in this package. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-05
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
