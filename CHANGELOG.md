# CHANGELOG

All notable changes to **rust3d** are documented here.  
Version SSOT: `Cargo.toml` `package.version` and `src/lib.rs` `VERSION`.

## [1.0.1] - 2026-10-11

### Changed

- The main menu lists **self-management** as **6** and **language** as **7**. Self-management children are **62**–**67**. Language children are **71**–**83**. Reserved language numbers **70** and **84**–**89** stay unprinted. Model stays **1**, system-log stays **3**, and Exit stays **9**.
- The ship unit is the Rust crate **rust3d**. The only binary is `rust3d`. `cargo run` starts it. Version stays **1.0.1**.
- `model` builds `model.glb` and `viewer.html` from outline images in one folder. Lifecycle verbs use cargo. StatusLog is in-tree (`src/status_log.rs`).
- Language leaf is `~/.local/rust3d/language`. `RUST3D_LANG` overrides that file for one process and does not write it.
- Public source is `https://github.com/cloudgen/rust3d`.
- Menu pictures in `screenshots/` were recaptured on 2026-10-14. The path-row clock is `20:26:52`. Language is row **7** and self-management is row **6**. The about host check is stamped `2026-10-14 20:26:52.404013`.
- Empty arguments on a terminal open the full-screen menu (`TermScreen` in `src/menu.rs`). The line prompt is gone. `cargo run` starts `rust3d`.

## Historical notes — previous package (2026-10-05)

These notes record the package this crate replaced. They are not a second 1.0.1 release. Picture pixels from that day were not recaptured.

### Changed

- Version was **1.0.1** on the previous package as well.
- Menu pictures in `screenshots/` are captures from 2026-10-05. The clock on those pictures is `20:26:52`. The host check is stamped `2026-10-05 20:26:52.404013`. The pixels still show the previous interface.

## [1.0.0] - 2026-10-05

### Added

- The previous package console script was the modeller command. Version was **1.0.0**.
- `model` builds `model.glb` and `viewer.html` from outline images in one folder. The default directory is that folder's `model`.
- A views file names each outline with azimuth and elevation. Without one, outlines are spaced evenly in azimuth at elevation 0.
- Menu row **1 model** lists **1** current folder, each subfolder, and **0** back, then runs that build.
- `./convert.py` calls the same build.
- Pip dependencies are `ChronicleLogger>=1.3.1`, `numpy>=2.3.0`, `opencv-python-headless>=5.0.0.93`, and `scikit-image>=0.25.0`.
- Public source for this crate is `https://github.com/cloudgen/rust3d`.
- `screenshots/model-viewer.png` is a capture of `sample-images/model/viewer.html` from the previous interface. The picture in `README.md` links to `https://cloudgen.github.io/rust3d/sample-images/model/viewer.html`. Picture addresses are absolute `https` URLs on `raw.githubusercontent.com`.
- Menu pictures in `screenshots/` are captures of this product. `model-folder.png` is the folder board. The previous product's join pictures are not included.

### Changed

- Specialized from the OutlineImage bootstrap. The outline-extraction verb is gone. Inputs are outline images.
- The previous language leaf was under `~/.local/` for that package. This crate uses `~/.local/rust3d/language` and `RUST3D_LANG`.
- Related Projects names OutlineImage on GitHub (`https://github.com/cloudgen/OutlineImage`) and on PyPI (`https://pypi.org/project/OutlineImage/`).

### Removed

- `screenshots/video.png` and the description of that still in `README.md`. The picture is not part of this program.

## [Unreleased] — HelloTui bootstrap history

The notes below, through the 1.0.5 section, are the HelloTui bootstrap this tree was copied from. They are not a second rust3d release.

### Removed

- Join, file listing, and FFmpeg. `join` and `list-videos` are not verbs. The program does not scan the folder for media and does not call an encoder.

### Changed

- Package name is **HelloTui**. Console script is `hello-tui`. Version is reset to **1.0.0**.
- Typed `hello` prints `Welcome to HelloTui 1.0.0.` in the terminal and does not draw the menu.
- Front row **1** is **hello**. Choosing it shows that same welcome line on the menu.
- The about page domain line is `Show a welcome message`. Self-management row 83 reads "version and this computer."

## [1.0.5] - 2026-10-04

### Changed

- `about` shows one page: product identity, a host check of this computer, and a star box. The page stays English. It does not probe `ffmpeg` and it does not print a pip install line.
- Self-management row 83 reads "version, FFmpeg, and this computer."
- Version is **1.0.5**. `ChronicleLogger>=1.3.1` is unchanged.
- Product README Screenshots links every picture in `screenshots/` with an absolute `https` URL. Menu pictures captured before the about page still show **1.0.4**.

## [1.0.4] - 2026-10-04

### Added

- Text menu on a terminal when `video-join` is started with no arguments. Front rows are join (1), system-log (3), language (4), self-management (8), and Exit (9). system-log lists view-log (31), clear-log (32), log-folder (33), and Back (0). language lists English through Ελληνικά (41–53) and Back (0). self-management lists version (82), about (83), version-check (84), self-update (85), self-uninstall (86), self-install (87), and Back (0).
- Menu language file `~/.local/VideoJoin/language`. `VIDEOJOIN_LANG` overrides that file for one process and does not write it. `language` is not a command-line verb.
- Typed verbs `help`, `version`, `about`, `hello`, `join`, `list-videos`, `self-install`, `version-check`, `self-update`, and `self-uninstall`.
- `ChronicleLogger` is constructed in `main` and required at `ChronicleLogger>=1.3.1`.
- Product README shows the main menu, the system-log board, the language board, and the self-management board.
- Product README PyPI version badge (`pypi.org/project/VideoJoin`).

### Changed

- Empty argv on a terminal opens that menu and does not start the join questions. With no terminal, empty argv prints help and returns 0.
- Quick Installation documents **`pip install VideoJoin`**. This tree is **1.0.4**. A registry probe on 2026-08-19 reported **1.0.3**. Checkout `pip install -e .` stays the local path.
- Join still prefers stream copy, then re-encode, and still publishes with `shutil.move`.

---

## [1.0.3] - 2026-08-09

### Added

- Same-FS staging helpers (`staging_dir_for`, `make_temp_path`) and **`promote_file` → `shutil.move`** for publishing intermediates.
- Unique temp concat list and media intermediates (no fixed cwd-only `filelist.txt` strategy).
- Product requirements set (class, domain, pipeline, CLI, coding-style, packaging, structure, errors, runtime).
- Promote gate checklist **`CL-PYTHON-SHUTIL-MOVE-PUBLISH`** (blank + filled run).

### Changed

- Join pipeline: stream-copy to temp → promote; on failure re-encode to temp → promote.
- Package public surface: `__version__` and `main` only (no phantom `ChronicleLogger` re-export from `.cli`).
- Product README rewritten for install/usage honesty (local package primary; FFmpeg system dep).

### Fixed

- Fail-closed re-encode path: non-zero FFmpeg or missing intermediate no longer prints success.
- Reject final output path equal to either source path.

### Security

- Still user-level only; no Type 1 elevation. FFmpeg invoked with argument lists (no shell string interpolation of free-form filters).

---

## [1.0.2] - prior

### Added

- Interactive two-file join with FFmpeg stream-copy and re-encode fallback.
- Package layout under `src/VideoJoin/` with console script `video-join`.

### Notes

- Earlier packaging claims may have listed broad Python support and optional Cython tooling; runtime join does not require Cython.

---

Update this file when shipping version bumps; keep body complete (no hollow TBD sections for claimed releases).
