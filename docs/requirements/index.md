# Requirements index

**Product:** rust3d (Rust CLI — text menu on a terminal; `model` builds a glTF model and an HTML viewer from outline images in a chosen folder, default `model.glb` and `viewer.html` in that folder's `model` directory; menu row 1 picks the current folder or a subfolder)
**Workspace state:** Specialized product law (left genesis); **software-development** class; **cargo/local crate** install (not shell online Type O).
**Product version:** **1.0.1** (align `Cargo.toml` `package.version`, `src/lib.rs` `VERSION`, root `README.md` Version badge)
**Updated:** 2026-10-14

| ID / key | Title | Area | Status | Path | Updated |
|----------|-------|------|--------|------|---------|
| requirement-class-software-dev | Software-development class law + residual stack (Rust, cargo) | class | Active | `requirement-class-software-dev.md` | 2026-10-14 |
| requirement-domain-rust3d | Domain surface SSOT (typed `model`, folder board on menu row 1, help, about, waiting sentence before building the glTF model) | domain | Active | `requirement-domain-rust3d.md` | 2026-10-11 |
| requirement-rust-about | About page: identity, host check, star box. No `--json` | rust | Active | `requirement-rust-about.md` | 2026-10-14 |
| requirement-video-ffmpeg-pipeline | Retired. No encoder, no concat, no media publish | video | Retired | `requirement-video-ffmpeg-pipeline.md` | 2026-10-11 |
| requirement-rust-cli-interface | CLI entry, typed verbs, empty-argv menu on a terminal and help off a terminal. `model` and `cargo run -- model` print the waiting sentence and flush before the work | rust | Active | `requirement-rust-cli-interface.md` | 2026-10-14 |
| requirement-rust-cli-logging | StatusLog construct in `fn main`; in-tree, not a crate floor | rust | Active | `requirement-rust-cli-logging.md` | 2026-10-11 |
| requirement-rust-tui | Text menu: model, system-log, self-management, language, Exit. Row 1 lists the current folder, each subfolder, and back. A folder pick paints a working page with the waiting sentence before the build. Path word is selected by the language requirement. Default is `Path`. Menu columns are display columns. Live screen is `TermScreen` | rust | Active | `requirement-rust-tui.md` | 2026-10-14 |
| requirement-rust-cli-language | Menu language: front row 7, codes 71–83, leaf `~/.local/rust3d/language`. `language` is not an argv verb | rust | Active | `requirement-rust-cli-language.md` | 2026-10-14 |
| requirement-rust-oop | Struct map. Modules under `src/`. Conversion stays in `model.rs`. No `convert` binary | rust | Active | `requirement-rust-oop.md` | 2026-10-14 |
| requirement-rust-coding-style | Rust style; temps; `std::fs::rename` when a publish-from-temp exists; end state is the struct map | rust | Active | `requirement-rust-coding-style.md` | 2026-10-11 |
| requirement-rust-packaging | `Cargo.toml` / version / one binary `rust3d` with `default-run`. Cargo strings point at the dependency file | rust | Active | `requirement-rust-packaging.md` | 2026-10-14 |
| requirement-rust-dependency-management | Cargo floors: `chrono>=0.4`, `crossterm>=0.28`, `image>=0.25`, `serde_json>=1`, `unicode-width>=0.2`. No GUI `opencv-python`. No menu framework. StatusLog is in-tree | rust | Active | `requirement-rust-dependency-management.md` | 2026-10-14 |
| requirement-rust-project-structure | Repository and `src/` layout. One binary, `rust3d` | rust | Active | `requirement-rust-project-structure.md` | 2026-10-14 |
| requirement-rust-readme | Root user document: sections, badges, picture catalog, related projects | rust | Active | `requirement-rust-readme.md` | 2026-10-14 |
| requirement-rust-error-handling | Fail-closed errors; source-safe cleanup; console sentences stay | rust | Active | `requirement-rust-error-handling.md` | 2026-10-11 |
| requirement-runtime-prerequisites | Cargo image stack; in-tree StatusLog; no host FFmpeg; no root auto-install | runtime | Active | `requirement-runtime-prerequisites.md` | 2026-10-11 |

## Intentionally absent (by design)

| Surface | Status on rust3d |
|---------|----------------------|
| Shell online install / `SCRIPT_URL` / Type O empty-argv install-ensure / `curl\|sh` | **Absent** |
| Shell local `install` / `uninstall` / shell self-update | **Absent** (cargo crate) |
| Type 1 sudoers / root elevation allowlist | **Absent** |
| Automatic companion `.sha256` channel integrity law | **Absent** |
| OpenCV as a host duration probe | **Absent**. Silhouette masks use the `image` crate (`requirement-rust-dependency-management`) |
| Second Active `requirement-domain-*` | **Forbidden**. The sole Active domain file is `requirement-domain-rust3d.md` |
| Front row 2, cut / speed / boomerang, `list-mp4`, `hello`, `join`, `list-videos`, FFmpeg | **Absent**. Front row 1 is `model` |
| `requirement-rust-version` | **Absent**. Version SSOT is `Cargo.toml` `package.version` and `src/lib.rs` `VERSION` |
| `requirement-rust-json-output` | **Absent**. This product does not claim `--json`. The about page does not grow a JSON stream |
| `requirement-rust-graceful-exit` | **Absent**. Control-C is not confirmed law. Row 9 Exit returns 0 with no confirm |
| `requirement-rust-interactive-vs-noninteractive` | **Absent**. Empty argv on a terminal versus help off a terminal is the CLI interface |
| `requirement-rust-pyenv`, `requirement-rust-conda`, build-script, `requirement-rust-oop-architecture` | **Absent**. Host check names rustc, cargo, and rustup on `requirement-rust-about`. Struct homes are `requirement-rust-oop` |
| Actor-role requirement file | **Absent**. Class residual is no dest approver |
| Active `requirement-python-*` beside this Rust law | **Absent**. Python law is covered by the `requirement-rust-*` files |

**Install mode:** **cargo / local crate** (`rust3d` binary). Not a shell installer.

**Cargo lifecycle:** claimed. Rows 64–67 and the typed verbs `version-check`, `self-update`, `self-install`, and `self-uninstall` are `requirement-rust-cli-interface` and `requirement-rust-tui`. They use `cargo`. They are not a shell channel. Empty argv does not run them.

**Logger:** required, in-tree. `fn main` in `src/cli.rs` writes `StatusLog::new`. It is not a crates.io floor. `chrono>=0.4` formats the timestamp.

**Rules for agents:**

1. Treat rows above as the **live product-law inventory** for rust3d.
2. **Do not invent** additional `requirement-*.md` paths — verify on disk and add a registry row in the same change when creating one.
3. Product source comments cite **only** these live requirement files.
4. This versioned surface lists **requirement rows only**.
5. Keep Status and Path in sync with each file’s header when status changes.
6. **Class gate:** software-development requires exactly one Active `requirement-class-software-dev.md` (this registry includes it).
7. **Domain SSOT:** exactly one Active domain file (`requirement-domain-rust3d`).
8. **Do not introduce** a shell installer or Type 1 elevation without an explicit user order and a registry update.
9. **Do not** treat cargo rows 64–67 as a reason to add a shell installer. **Do not** mark those cargo rows absent.

When adding a requirement: append a row, create the file under `docs/requirements/`, keep Status in sync with the file header.
