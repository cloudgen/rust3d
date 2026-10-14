# Requirements

Authoritative specialized product law for **rust3d** lives here.

**Current state (2026-10-11):** Specialized **software-development** product. Left genesis. Registry is populated — see `index.md`. Fifteen Active requirements and one Retired encoder file. Product version is **1.0.1**.

## Product identity (summary)

| Field | Value |
|-------|--------|
| Product / crate | `rust3d` |
| Version SSOT | **`1.0.1`** (`Cargo.toml` `package.version` + `src/lib.rs` `VERSION`) |
| Product README SSOT | Root `README.md` (app-name, short description, target version **1.0.1**). Sections and pictures: `requirement-rust-readme` |
| Ship surface | Rust crate; the only binary is **`rust3d`**. `cargo run` starts it |
| Install mode | **cargo / local crate** (`cargo install rust3d`; this tree **1.0.1**; checkout `cargo install --path .`) — not shell Type O |
| Domain surface | `requirement-domain-rust3d` — four pillars (`model`, the folder board, help, about) |
| Text menu | `requirement-rust-tui` — front rows model, system-log, language, self-management, Exit. Row 1 lists the current folder, each subfolder, and back. Default path word is `Path` |
| Menu language | `requirement-rust-cli-language` — row 7, thirteen codes, leaf `~/.local/rust3d/language`. `language` is not an argv verb |
| Status logger | `requirement-rust-cli-logging` — `StatusLog::new` inside `fn main`. In-tree. Required |
| Struct map | `requirement-rust-oop` — modules under `src/` |
| Encode ops | `requirement-video-ffmpeg-pipeline` — **Retired**. No encoder and no media publish |
| Coding style | `requirement-rust-coding-style` — temps + `std::fs::rename` when a publish-from-temp exists |
| Runtime tools | No host binary. Cargo floors are `requirement-rust-dependency-management`: **chrono>=0.4**, **image>=0.25**, **serde_json>=1**, **unicode-width>=0.2** |

## Class requirement gate

| Class | Required class file |
|-------|---------------------|
| software-development | `requirement-class-software-dev.md` (**Active**) |
| genesis-template | N/A — this workspace is no longer genesis for product law |

## Purpose

- **Plan** designs work by reading and updating these docs.
- **Implement** delivers code that **traces** to these requirements.
- **Review** verifies delivery against requirements and CIAO checklists.
- Product **README** must stay honest with this law (install, features, version).

## Layout

| Path | Role |
|------|------|
| `docs/requirements/index.md` | Registry of all requirements — keep in sync |
| `docs/requirements/requirement-*.md` | CIAO-style project requirements |

## Status values

Typical: `draft` · `Active` · `approved` · `in-progress` · `done` · `deprecated` · `superseded`

## Rules

1. Never invent paths — verify on disk.
2. Class files only via class process; non-class via create-specific process.
3. Never dump harness inventories into this versioned surface.
4. Online shell install and Type 1 elevation stay **absent** unless product mode is explicitly changed.
5. Sole domain SSOT: `requirement-domain-rust3d.md`.
6. When changing model output: update the domain file **and** root `README.md` in the same change.
7. Version dual SSOT + README Version badge must match when a release is claimed.
