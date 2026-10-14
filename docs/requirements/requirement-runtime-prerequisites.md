**file**: docs/requirements/requirement-runtime-prerequisites.md
**Status**: Active (Version 1.2.4 – Terminologies name the package `rust3d`, matching `package.name`. Product version stays 1.0.1)
**Area**: runtime
**Key**: `requirement-runtime-prerequisites`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Declare the host and Rust runtime prerequisites required to run rust3d. This product does not implement a privileged installer. This file is the documentation and validation SSOT for what must already be present. The cargo strings are `requirement-rust-dependency-management`.

### 1.1 Human-facing

**In one sentence:** You need rustc, cargo, and the crates in `requirement-rust-dependency-management`. StatusLog is in-tree. No encoder is required.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Person about to build a model from a folder of outlines | cargo already fetched the image crate |
| The other role | cargo | Installs rust3d and fetches `chrono`, `image`, `serde_json`, and `unicode-width`. StatusLog is in-tree |
| Not this file | The menu picture and the outline steps | TUI and the domain file |

| Includes | Excludes |
|----------|----------|
| The image stack as cargo crates | A claim that a host `ffmpeg` binary is required |
| StatusLog, compiled in this crate | Calling that logger a separate crate |
| A console next step when a required import is missing | Root auto-install |

| Surface | What you open | What for |
|---------|---------------|----------|
| cargo | `chrono`, `image`, `serde_json`, `unicode-width` | Timestamp, decode, views, and menu columns |
| About page | Runtime-tools line | The word `none`. Does not probe PATH (`requirement-rust-about`) |
| First model build that has an outline | Vision stack | Imported only when the folder has a supported image. No weight download |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Install the crate | You get rust3d and the image crates. StatusLog is in the crate. | `cargo install rust3d` |
| Convert an empty folder | The program returns 0 and does not load the model. | `rust3d model` |
| Start with the logger library missing | The console prints the cargo next step and returns non-zero. | `rust3d` |

## 2. Core Rules (Mandatory)

### 2.1 Scope

1. **MUST** document every external tool required at runtime.
2. **MUST NOT** claim the product auto-installs system packages via root or sudo.
3. **MUST** separate cargo crates from system binaries.

### 2.2 Required runtime components

| Component | Kind | Required for | Install surface |
|-----------|------|--------------|-----------------|
| rustc and cargo | toolchain | the CLI | The host Rust toolchain. This product does not require pyenv |
| StatusLog | in-tree module `src/status_log.rs` | the one status logger | Not a cargo crate. Required, not optional |
| `image>=0.25`, `serde_json>=1`, `unicode-width>=0.2` | cargo crates | a model when the folder has a supported outline | `requirement-rust-dependency-management`. Required. An empty folder does not import them |
| FFmpeg | not required | nothing in this product | not installed and not checked |

### 2.3 Validation

4. **MUST NOT** fail because `ffmpeg` is missing. The front board and `model` do not look for it.
5. The front board **MAY** open when the image stack is not imported yet. About does not probe those packages and does not install them.
6. **MUST** document that FFmpeg is not installed by `cargo install rust3d` and is not required.
7. **MUST** document the input formats as the domain format set.
8. StatusLog is compiled into the crate (`src/status_log.rs`). It is not a separate import. If the image crate cannot decode while a folder has a supported image, `model` **MUST** print the cargo next step and return 1.

### 2.4 Privilege

9. **MUST NOT** require root to satisfy runtime prerequisites for normal use.
10. Type 1 elevation is out of scope.

### 2.5 Implementation Notes (this project)

| Item | Value |
|------|--------|
| **Crate install** | `cargo install rust3d`, or `cargo install --path .` from a checkout |
| **Declared cargo dependency (law)** | The four strings in `requirement-rust-dependency-management`. All required. StatusLog is in-tree |
| **Live manifest** | `Cargo.toml` lists those four strings. Edition is `2021` |
| **System binary** | none. `ffmpeg` is not required |
| **Auto install command** | none. The product does not run a root install. The build does not download weights |
| **Platform notes** | Linux primary. macOS and Windows when rustc is available |
| **Product version** | 1.0.1 |
| **Startup check** | No encoder check. The image stack is imported only when the chosen folder has a supported image |
| **User docs** | Root `README.md` names the cargo image stack and says no external media tool is required |

### 2.6 Why This Requirement Exists (CIAO)

- **Principle 1 – Caution**: A missing image library is reported when an image is actually converted.
- **Principle 10 – Least privilege**: No root installer.
- **Principle 2 – Intentional**: The cargo libraries and a host encoder are different facts. This product has no host encoder.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, the normal user installs the cargo package. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to install StatusLog or the image stack from inside rust3d. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Document a missing library before the conversion claims success.
- **Intentional:** cargo installs the image stack. It does not install FFmpeg. StatusLog is required.
- **Anti-fragile:** An empty folder still returns 0 without the model.
- **Over-protect:** No silent root install.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Claim FFmpeg is installed by `cargo install rust3d`, or claim this product requires it.
2. Add a root package-manager install from this product.
3. Treat a missing `ffmpeg` binary as a failed model build.
4. Store secrets in this file.
5. Drop `image`, `serde_json`, or `unicode-width` while `model` still decodes an outline.
6. Call StatusLog optional.

**Violating this rule is a critical honesty regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | FFmpeg is listed as not required |
| AC-2 | StatusLog is in-tree, and the image floors are `requirement-rust-dependency-management` |
| AC-3 | No root auto-install claim |
| AC-4 | A missing image library on a folder that has an image produces the cargo next step |
| AC-5 | Missing StatusLog produces the cargo next step and a non-zero return |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-dependency-management` | Cargo strings |
| `requirement-rust-packaging` | Manifest shape |
| `requirement-rust-cli-logging` | Construct. Missing-import sentence |
| `requirement-video-ffmpeg-pipeline` | Retired. This file does not require an encoder |
| `requirement-rust-error-handling` | Missing-library messages |
| `requirement-domain-rust3d` | About runtime-tools line is `none` |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-PRE-01** | `tests/product.rs` | have | An empty folder does not decode. Help does not name FFmpeg |
| **TP-PRE-02** | `tests/product.rs` | n/a | StatusLog is in-tree. There is no crate floor `>=1.3.1` |
| **TP-ERR-01** | `tests/product.rs` | have | A missing folder returns 1. An empty folder returns 0 |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Rust crates.io CLI

**Definition:** A **Rust crates.io CLI** is a **software-development** product whose primary operator surface is a **short-lived console script** installed from **Cargo**: `Cargo.toml` is the packaging SSOT, ship source lives under **`src/`**, and the operator runs the **declared console entry** (not a long-running HTTP process). The default coding shape is **thin `src/cli.rs`** plus a **StateLogic + Attr** orchestrator (OOP **L3**). It **does not** use `backend/` + `frontend/` as the ship tree.

**Human daily-life explanation:**

A **rust-crates-cli** is a short-lived console command installed with cargo: `Cargo.toml`, source under `src/`, declared entry. It is not a backend/frontend diner.

**Daily-life example:** A mixer you install from a catalog listing, with a named “on” switch—not a café that stays open with a dining room.

This product's package name is `rust3d` (`Cargo.toml` `package.name`), the import package is `rust3d` under `src/`, and the console script is `rust3d`.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-08-09 | Active 1.0.0 | Initial runtime prerequisites law for rust3d |
| 2026-10-04 | Active 1.1.0 | StatusLog `>=1.3.1` is required. Live manifest still `>=1.2.3`. FFmpeg stays a system binary. The front board may open without it |
| 2026-10-04 | Active 1.1.1 | Product version **1.0.4**. Manifest floor is `chrono>=0.4` |
| 2026-10-04 | Active 1.1.2 | The about page names FFmpeg and does not probe PATH. Product version **1.0.5** |
| 2026-10-05 | Active 1.2.0 | No host encoder. Image stack is cargo crates. Product version **1.0.0**. About runtime tools are `none` |
| 2026-10-05 | Active 1.2.1 | Pip strings point at `requirement-rust-dependency-management`. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.2.2 | Product version **1.0.1** |
| 2026-10-05 | Active 1.2.3 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-05 | Active 1.2.4 | Terminologies name the package `rust3d`, matching `package.name`. The console script stays `rust3d`. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-05
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
