**file**: docs/requirements/requirement-rust-project-structure.md
**Status**: Active (Version 1.1.6 – One binary, `rust3d`. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-project-structure`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Define the repository layout and the package structure for rust3d: where source, packaging, and requirement law live, and which modules exist today versus which modules the struct map names.

### 1.1 Human-facing

**In one sentence:** The installable code lives in `src/` as the rust3d crate.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Maintainer looking for the program | Open `src/cli.rs` |
| The other role | The struct map | `requirement-rust-oop` names the modules. They are on disk |
| Not this file | What the menu shows, and how two videos are concatenated | TUI, domain, and pipeline requirements |

| Includes | Excludes |
|----------|----------|
| `src/lib.rs`, `src/cli.rs`, `src/model.rs`, and the one binary `rust3d` | A second installable package name or a second binary |
| The class files as the allowed end state, landed by a later implement order | Treating those files as already required on disk |
| Root `Cargo.toml`, root `README.md`, root `CHANGELOG.md`, and `docs/requirements/` | Deleted design notes listed as if they were still present |

| Surface | What you open | What for |
|---------|---------------|----------|
| `src/` | The package | Version, module entry, and the running session |
| `Cargo.toml` | Packaging | Name, version, console script |
| `docs/requirements/index.md` | Registry | Which requirement files are Active |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Run from a checkout | The module entry calls `main` in `cli.rs`. | `cargo run` |
| Look for the menu types | They are named by the OOP requirement and live under `src/`. | Read `requirement-rust-oop` |
| Read the product history | Use the changelog at the repository root. | `CHANGELOG.md` |

## 2. Core Rules (Mandatory)

### 2.1 Source package layout

1. **MUST** keep the installable package under `src/`.
2. The crate **MUST** include `src/lib.rs` (`VERSION`), `src/cli.rs` (struct `Cli` and `fn main`), `src/model.rs` (the conversion functions), and `src/bin/rust3d.rs`. **MUST NOT** include `src/bin/convert.rs`.
3. The modules named by `requirement-rust-oop` live under `src/`: `menu.rs`, `language.rs`, `system_log.rs`, `self_manage.rs`, `about.rs`, and `status_log.rs`. There is no `join` module. Root `convert.py` is not a checkout entry. The only binary is `rust3d`.
4. **MUST NOT** scatter a second installable package name that contradicts the packaging SSOT.

### 2.2 Project root layout

5. **MUST** keep `Cargo.toml` at the repository root.
6. **MUST** keep product user docs at root `README.md`. Sections, badges, and pictures in that document are `requirement-rust-readme`.
7. **MUST** keep the product changelog at root `CHANGELOG.md`.
8. **MUST** keep specialized product law under `docs/requirements/` with the `requirement-` prefix and the registry `index.md`.
9. **MUST NOT** list a deleted design note as a present file. `docs/CHANGELOG.md`, `docs/rust3d-spec.md`, and `docs/folder-structure.md` are not in the tree.

### 2.3 Generated / non-source

10. **MUST NOT** commit `build/` or `dist/` artifacts as the product source of truth.
11. `target/` **MUST** remain ignore-friendly. Egg-info and `__pycache__` are not part of this crate.
12. Optional `build.sh` **MAY** exist as maintainer tooling. It **MUST NOT** replace the crate under `src/`.

### 2.4 Requirements surface discipline

13. Product-law files **MUST** use the basename prefix `requirement-`.
14. **MUST** register every Active requirement in `docs/requirements/index.md`.
15. Product source comments that cite law **MUST** cite live `requirement-*.md` keys only.

### 2.5 Implementation Notes (this project)

| Path | Role |
|------|------|
| `src/` | Installable package |
| `src/lib.rs` | `VERSION` (`1.0.1`) |
| `src/bin/rust3d.rs` | Binary `rust3d`. Calls `cli::main` |
| `src/cli.rs` | Struct `Cli` and `fn main` |
| `src/model.rs` | Model functions. `rust3d model` calls `convert_main` |
| `Cargo.toml` | Packaging SSOT |
| `build.sh` | Maintainer build helper |
| `docs/requirements/` | Product law |
| `CHANGELOG.md` | Product changelog at the repository root |
| `tests/` | Proof. `tests/run.sh` discovers the suite. Suites are not law |
| `README.md` | User documentation |

The class modules named in rule 3 are on disk, together with `model.rs`. `join.py` is not.

### 2.6 Why This Requirement Exists (CIAO)

- **Principle 2 – Intentional**: The running tree and the target tree are both named.
- **Principle 5 – SSOT**: One package path. One requirements registry. One changelog path.
- **Principle 17 – Storage**: Generated directories are not source.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, the package is the normal user’s `rust3d`. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to lay out the package or to start it. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Do not invent a parallel package, and do not invent files that were deleted.
- **Intentional:** `src/` is the layout. The struct map says what a later edit adds.
- **Anti-fragile:** Build debris stays ignored.
- **Over-protect:** The registry stays aligned with the files on disk.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Move the installable package out of `src/` without a packaging update.
2. Delete the `docs/requirements/index.md` discipline.
3. Commit secrets under `src/` or `docs/requirements/`.
4. Cite a template or a skill from product source as product law.
5. Treat a design note as a second law SSOT over an Active requirement.
6. List `docs/CHANGELOG.md`, `docs/rust3d-spec.md`, or `docs/folder-structure.md` as present.
7. Require the OOP class files on disk before an implement order, or keep `cli.rs` as the permanent home of every other class after that order.

**Violating this rule is a critical structure regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | The package lives under `src/` |
| AC-2 | Root `Cargo.toml` is present |
| AC-3 | Requirements live under `docs/requirements/` with an index |
| AC-4 | Generated `build/` and `dist/` are not the source SSOT |
| AC-5 | The modules in rule 3 are on disk |
| AC-6 | The product changelog path is root `CHANGELOG.md` |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-oop` | Target modules |
| `requirement-rust-packaging` | Manifest |
| `requirement-rust-cli-interface` | Entry module |
| `requirement-class-software-dev` | Class residual |
| `requirement-rust-readme` | Sections, badges, and pictures in root `README.md` |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-STRUCT-01** | `tests/product.rs` | have | Crate sources include `lib.rs`, `cli.rs`, `model.rs`, and the one binary `rust3d` |
| **TP-OOP-01** | `tests/product.rs` | todo | Struct map files are on disk. Method-level ownership is not fully asserted |

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

### Ship unit

**Definition:** A **ship unit** is the **primary installable artifact**. For this packaged command, that artifact is the Python package `rust3d`. The version string lives in `Cargo.toml` `package.version` and in the import package `VERSION`. A proper project version bump keeps those two equal. This product's ship unit is not a single root shell file fetched by a script URL.

**Human daily-life explanation:**

A **ship unit** is the bottle you install — the program itself — not the shop window. For this product the bottle is the packaged command `rust3d`.

**Daily-life example:** You install the package and run the console command. That installed command is the ship unit. The project web page is not the bottle.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-08-09 | Active 1.0.0 | Initial project structure law for rust3d |
| 2026-10-04 | Active 1.1.0 | Running tree is three modules. Target modules are the OOP map. Deleted design-note paths are not listed as present. Changelog is root `CHANGELOG.md` |
| 2026-10-04 | Active 1.1.1 | Sections, badges, and pictures in root `README.md` are `requirement-rust-readme` |
| 2026-10-04 | Active 1.1.2 | Allowed end state names `check_system.py`. `VERSION` is **1.0.5** |
| 2026-10-05 | Active 1.1.3 | `VERSION` is **1.0.1** |
| 2026-10-05 | Active 1.1.4 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-05 | Active 1.1.5 | Terminologies name the package `rust3d`, matching `package.name`. The ship unit is that package. The console script stays `rust3d`. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.1.6 | The only binary is `rust3d`. `src/bin/convert.rs` is not part of the crate. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
