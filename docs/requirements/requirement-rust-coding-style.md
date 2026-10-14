**file**: docs/requirements/requirement-rust-coding-style.md
**Status**: Active (Version 1.1.3 – Terminologies section before status history. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-coding-style`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Define Python coding style and defensive file I/O conventions for rust3d: how agents and maintainers write Python so path, temp, and publish behavior stays safe across mounts, including USB, without duplicating domain or FFmpeg pipeline tables.

Pipeline-specific application of these rules is owned by `requirement-video-ffmpeg-pipeline`. The struct map is owned by `requirement-rust-oop`. The logger construct is owned by `requirement-rust-cli-logging`.

### 1.1 Human-facing

**In one sentence:** When a file is published from a temporary path, that publish is `std::fs::rename`, and the classes that do the work live in the files the struct map names. Outline images are written by the conversion module.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Maintainer changing the conversion or the menu | Keep a publish-from-temp as `std::fs::rename`. Put the outline steps in `model.rs` |
| The other role | The struct map and the status logger | `requirement-rust-oop`, `requirement-rust-cli-logging` |
| Not this file | The menu picture and the concat filter graph | TUI and pipeline requirements |

| Includes | Excludes |
|----------|----------|
| `std::fs::rename` as the publish call. Temps beside the destination when that folder is writable | Bare `os.rename` as the only publish when the mounts differ |
| The struct map as the allowed end state | Treating the procedural pile in `cli.rs` as the allowed end state |
| Package import that does not construct StatusLog | Re-exporting StatusLog, or a factory whose job is to build a mapped class |

| Surface | What you open | What for |
|---------|---------------|----------|
| `src/cli.rs` | Running session today. `Cli` and `fn main` in the end state | Entry |
| Class files named by `requirement-rust-oop` | One class each, when an implement order lands them | The jobs that `cli.rs` still holds |
| Publish helper | `std::fs::rename` | Move the finished temp to the output name |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Publish the finished file | Call `std::fs::rename`. Log that publish before the call, once the logger exists. | The join output name |
| Add a behavior | Put it on the class in the map. Do not add a module-level function beside that class. | Edit that class file |
| Build the crate | The crate builds when a test only needs `VERSION`. Loading the library does not open the log. | `cargo test` |

## 2. Core Rules (Mandatory)

### 2.1 General style (this product)

1. **MUST** keep the crate under `src/` with binary `rust3d` calling `rust3d::cli::main`.
2. **MUST** cite only live `docs/requirements/requirement-*.md` keys in product-source law comments.
3. **SHOULD** use a clear General Purpose docstring on each public method.
4. **MUST** fail closed with a user-visible message on expected errors (missing folder, bad grid, unknown verb).
5. Importing the rust3d package **MUST NOT** construct StatusLog. The construct stays inside `fn main` (`requirement-rust-cli-logging`). **MUST NOT** re-export `StatusLog`.
6. The allowed end state is the struct map in `requirement-rust-oop`. The running tree is still the procedural session in `cli.rs`, and that tree stays legal until an implement order lands the map. **MUST NOT** treat the procedural pile as the allowed end state. **MUST NOT** order a StateLogic + `Attr` rewrite. This file does not move the code.
6a. The site that needs an object **MUST** write `ClassName(...)`. A function or a method whose job is to instantiate a class in that map is forbidden.

### 2.2 Temporary files

7. When the final destination path is known, **MUST** prefer creating intermediate files on the same filesystem or mount as that destination (a writable parent of the final path).
8. **MUST NOT** assume system `TMPDIR` or `/tmp` is the same mount as the user’s media (USB, network, secondary disks).
9. **MUST** clean intermediate temps on success and on failure (best-effort).
10. **MUST NOT** use a fixed predictable cwd name (for example a bare `filelist.txt`) as the sole concat-list path when a unique temp can be created.

### 2.3 Publishing / moving completed files (sacred)

11. To move or publish a completed intermediate file from a temporary path to its final path, **MUST** use `std::fs::rename` (or a thin wrapper whose only move implementation is `std::fs::rename`).
12. **MUST NOT** use bare `os.replace`, `os.rename`, `pathlib.Path.replace`, or `pathlib.Path.rename` alone as the sole publish mechanism when source and destination may be on different mounts.
13. **MAY** still use same-filesystem rename semantics inside what `std::fs::rename` performs. Do not reimplement a fragile bare rename as product publish.
14. `shutil.copy2`, `copy`, and `copyfile` copy only. If one is used, **MUST** define whether the source is kept or deleted. A copy is not a complete move by itself.
15. FFmpeg **MAY** write the final path directly when no intermediate publish step exists. When an intermediate is used, publish **MUST** go through `std::fs::rename` (or the thin wrapper).

Log the temp write, the publish, and the discard of an unfinished temp before the operation (`requirement-rust-cli-logging`). Those lines do not replace rules 7–15.

### 2.4 Corresponding commands / APIs (reference)

| Intent | Prefer | Avoid as sole cross-mount publish |
|--------|--------|-----------------------------------|
| Move or publish a file | `std::fs::rename(src, dst)` | bare `os.replace`, `os.rename` |
| pathlib-oriented move | `Path` args + `std::fs::rename` | bare `Path.replace`, `Path.rename` across mounts |
| Copy and keep the source | `shutil.copy2` | treating a copy as a move without an unlink policy |
| Same-filesystem atomic finish | OK via the `std::fs::rename` rename path | assuming all mounts are identical |

### 2.5 Implementation Notes (this project)

| Item | Value |
|------|--------|
| **Package** | `rust3d` |
| **Running modules** | `src/lib.rs`, `src/cli.rs`, `src/model.rs`, `src/bin/rust3d.rs` |
| **End-state modules** | Named by `requirement-rust-oop`. On disk |
| **Staging helpers** | This crate does not publish a join. There is no `Join` |
| **Publish helper** | `std::fs::rename` |
| **Ops apply** | `requirement-video-ffmpeg-pipeline` |
| **Gate checklist (cite ID)** | `CL-PYTHON-SHUTIL-MOVE-PUBLISH` — run when auditing promote and staging publish paths |
| **Architecture** | Running tree is procedural. Allowed end state is the struct map. StateLogic + `Attr` stays unordered |
| **Logger** | Package import does not construct StatusLog |
| **Version** | `1.0.1` |
| **User docs** | Root `README.md` Features must not claim a Cython-required runtime or a fixed `filelist.txt`-only strategy |

### 2.6 Why This Requirement Exists (CIAO)

- **Principle 1 – Caution**: Mount mismatches are designed for.
- **Principle 3 – Anti-fragile**: A USB destination and a system disk both publish through `std::fs::rename`.
- **Principle 5 – SSOT**: This file owns move and temp rules. The struct map lives in its own requirement.
- **Principle 11 – Temps**: Staging and cleanup are explicit.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, temps and the published file stay in the normal user’s folders. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to publish a join or to construct a class. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Do not assume one filesystem.
- **Intentional:** `std::fs::rename` is the named publish API. The struct map is the named end state.
- **Anti-fragile:** Temps on the destination mount avoid a full-file copy when the mount allows it.
- **Over-protect:** Do not bring back a bare cross-device rename as the only publish.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Replace `std::fs::rename` publish with bare `os.replace` or `os.rename` for simplicity.
2. Stage every large intermediate only under the system temp when the final destination is known on another mount.
3. Cite a template or a skill from product source as behavioral authority.
4. Treat the procedural session in `cli.rs` as the allowed end state. Do not start the class split inside a style-only edit. Do not order a StateLogic rewrite.
5. Store secrets in style docs or in code.
6. Reintroduce a fixed-name cwd `filelist.txt` as the only concat-list strategy.
7. Construct StatusLog at import time, or add a factory whose job is to instantiate a mapped class.

**Violating this rule is a critical regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | Publish of intermediates uses `std::fs::rename` (or a thin wrapper) |
| AC-2 | A bare cross-mount rename is forbidden as the sole publish |
| AC-3 | Same-filesystem staging is preferred when the destination is known |
| AC-4 | The pipeline requirement remains the concat SSOT |
| AC-5 | Registered in the index |
| AC-6 | Package import does not construct StatusLog and does not re-export it |
| AC-7 | A promote-path audit uses `CL-PYTHON-SHUTIL-MOVE-PUBLISH` when ship publish code changes |
| AC-8 | The allowed end state points at `requirement-rust-oop`. StateLogic stays unordered |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-oop` | Allowed struct map |
| `requirement-rust-cli-logging` | Log temp, publish, and discard. Construct stays in `main` |
| `requirement-video-ffmpeg-pipeline` | Applies move and temp rules to concat |
| `requirement-rust-project-structure` | Running layout versus the map |
| `requirement-rust-error-handling` | Failure messages |
| `requirement-rust-cli-interface` | Entry |
| `requirement-rust-packaging` | Export honesty |
| `requirement-class-software-dev` | Class residual |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-FS-01** | `tests/test_fs_publish.py` | todo | Publish uses `std::fs::rename` |
| **TP-FS-02** | `tests/test_fs_publish.py` | todo | Staging prefers the destination parent when it is writable |
| **TP-PKG-01** | `tests/product.rs` | have | The crate builds. StatusLog is not a cargo dependency |
| **TP-OOP-01** | `tests/product.rs` | todo | Struct files are on disk. Method ownership is not fully asserted |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Coding-style requirement

**Definition:** A **coding-style requirement** is the software-development **specialize-in home** for portable learned lessons about how the ship unit is written. Every software-development project **MUST** have an Active language-matched coding-style related `requirement-*.md`. **Without that file, agents bring portable learned lessons raw** (from coding skills and law molds) and treat them as this product’s law. The coding-style REQ is where those lessons are adopted, pointed at peer REQs, or refused.

**Human daily-life explanation:**

A **Coding-style requirement** is this product’s **adopted writing style home**. Every software-dev project must have one. Without it, helpers would treat portable lessons as this product’s law raw. The REQ is where those lessons are taken, pointed, or refused.

**Daily-life example:** A newsroom keeps one style sheet (“we write dates this way”). That adopted sheet is the **Coding-style requirement**; the industry handbook on the shelf is not automatically house law until the sheet says so.

### Avoid a factory that only creates a class

**Definition:** **Avoid-factory-create-class** is the portable Python construct rule that an object is created by writing `ClassName(...)` at the site that needs that object. A **factory** is a function or a method whose job is to instantiate a class: a module function that returns `SomeClass(...)`, a method that returns `SomeClass(...)`, or `Cls.__new__(Cls)` followed by a method that builds the object. The class can be created without that factory. The factory does not add a way to create the object that the call site lacks. It hides the class name behind a second home.

**Human daily-life explanation:**

**Avoid-factory-create-class** means you pick up the thing itself. You do not hand the job to a clerk whose only task is to pick it up and hand it back. The class already knows how to become an object. Writing `ClassName(...)` is that step. A factory is the clerk.

**Daily-life example:** You want a kettle. You take a kettle off the shelf. You do not fill out a slip that a back-room clerk uses to fetch the same kettle and pass it through a window. That slip is a factory. The kettle was already on the shelf.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-08-09 | Active 1.0.0 | Coding style, `std::fs::rename`, and multi-mount file I/O for rust3d |
| 2026-10-04 | Active 1.1.0 | Procedural `cli.rs` is the running tree, not the allowed end state. The end state is `requirement-rust-oop`. StateLogic stays unordered. Rules 7–15 are unchanged |
| 2026-10-04 | Active 1.1.1 | Current version string is **1.0.5** |
| 2026-10-05 | Active 1.1.2 | Current version string is **1.0.1** |
| 2026-10-05 | Active 1.1.3 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-05
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
