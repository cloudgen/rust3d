**file**: docs/requirements/requirement-domain-rust3d.md
**Status**: Active (Version 1.0.2 – Terminologies section before status history. Product version stays 1.0.1)
**Area**: domain
**Key**: `requirement-domain-rust3d`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

This requirement is the domain surface Single Source of Truth for rust3d: one chosen folder of outline images becomes a glTF model and an HTML viewer.

Typed verbs and empty argv are owned by `requirement-rust-cli-interface`. The screen that shows menu row 1 is owned by `requirement-rust-tui`. The conversion functions live in `src/model.rs`. The about page is `requirement-rust-about`.

This file remains the sole Active `requirement-domain-*`. Headings and pictures in the root user document are `requirement-rust-readme`.

### 1.1 Human-facing

**In one sentence:** rust3d builds a glTF model and an HTML viewer from the outline images in a chosen folder.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Person who has several outline images of one object | `rust3d model` |
| The other role | The text menu, row 1 | `requirement-rust-tui` |
| Not this file | The rounded menu frame, cargo install, and the log folder | TUI, packaging, and logging requirements |

| Includes | Excludes |
|----------|----------|
| Top-level outline images in one folder, written as `model.glb` and `viewer.html` | Joining media, an encoder, a recursive scan, a segmentation download |
| Typed `model`, which builds and does not draw the menu and does not prompt | `hello`, `join`, `list-videos`, and `outline` as verbs |
| Menu row 1 **model**: **1** current folder, each immediate subfolder, **0** back, then the same build | A second prompt after the folder is chosen |
| One English sentence before the build | A sentence that says an AI model is downloading |
| Help and about as real pages | A downloaded-script installer advertised on the about page |

| Surface | What you open | What for |
|---------|---------------|----------|
| `rust3d model` | The terminal | Build from the current directory. No menu |
| `rust3d model photos` | The terminal | Build from that folder. No menu |
| Menu row 1 | The folder board, then a result page | Current folder or one subfolder, then the model |
| `cargo run -- model` | A checkout | The same build as the verb. `cargo run` with no words opens the menu |
| `rust3d about` | About page | Name, version, what the product does, how to start it |
| `rust3d help` | Help page | The same capabilities, including what this product does not do |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Build from this folder | `model.glb` and `viewer.html` in `./model`. The menu does not open. The process does not prompt. | `rust3d model` |
| Build a named folder | The same, for that folder's own `model` directory. | `rust3d model photos` |
| Name the cameras | Use the angles in a views file. | `rust3d model photos --views views.json` |
| Pick from the menu | On a terminal, open the menu and choose **1**. Then **1** is the current folder, the next numbers are subfolders, and **0** returns. Before the build starts, the screen says that choice has been selected and that the work takes time. The result page shows the build lines. | `rust3d`, then `1`, then a number |
| Read about | You see rust3d, the package version, and the domain sentence. | `rust3d about` |

## 2. Core Rules / Requirements (Mandatory)

### 2.1 Pillar A — The model verb

`model` builds one folder. It **MUST NOT** draw the menu and **MUST NOT** prompt. With no folder, the folder is the current directory.

Supported inputs, top level only, are `.webp`, `.png`, `.jpg`, and `.jpeg`. Subfolders are not scanned for images. `cargo run -- model` calls the same function.

The build fills each outline into a solid silhouette, carves a visual hull, and writes a triangle mesh. Azimuth 0 places the camera on the +Z side, looking at the origin. Positive elevation raises the camera. Degrees are the unit. When `--views` is set, that JSON file is the only camera list. When `--views` is omitted and `<folder>/views.json` is a file, that file is the camera list. Otherwise every supported image is used, sorted by name, with equal azimuth steps and elevation 0.

| Step ID | User action | Inputs | Output / effect | Ops SSOT |
|---------|-------------|--------|-----------------|----------|
| D-01 | Build from the terminal | typed verb `model`, optional folder, optional `--views`, `--grid`, `--output` | `model.glb` and `viewer.html`. Return 0 when the folder exists and both files are written. The menu does not open | this file + CLI |
| D-02 | Build from the menu | menu row 1, then one folder row | The same build. Before it starts, the working page shows the waiting sentence. The lines are the result page. That page does not paint the working page again | this file + TUI |

**Routing:** `rust3d model` **MUST** run D-01 and **MUST NOT** draw the folder board. Front-board row 1 **MUST** open the folder board and **MUST NOT** build until a folder row is chosen. Bare `rust3d` on a terminal opens the front board and **MUST NOT** run D-01.

**Empty folder:** when the folder exists and has no supported image, and no views file names an image, the process **MUST** return 0 and **MUST** say no supported images were found. It **MUST NOT** decode an image for that path.

**Missing folder, bad grid, or bad views file:** return 1 and a next-step line. `--grid` must be an integer from 16 to 160. The default is 96. A failure on one image **MUST** be reported and **MUST NOT** stop the remaining images. Any image failure makes the exit code 1. When both files are written, the last line is `Model saved in:` plus the output directory.

**Waiting sentence:** This file owns the words. Before building, the operator sees one English sentence: `{choice} has been selected. {process} takes time to finish.` The process words are `Building the 3D model`. Those process words stay English when the menu language is Chinese. The choice token is the typed verb `model`, or the folder-board short: `current` when the pick is `.`, otherwise the child directory name.

Show `Building the 3D model` only when at least one outline will be read and the output directory was created, and show it before the vision-stack import. An empty folder, a missing folder, a rejected grid, a bad views file, and a failed output-directory create do not show the sentence. Opening the folder board does not show it. Language, Exit, version, about, help, and the cargo lifecycle do not show it. This product does not download a segmentation model and does not show a download sentence.

A function that returns the sentence is enough. Do not store the sentence in a module-level constant. Returned lines lead with the sentence when it was shown. On the terminal, a caller that already printed it prints only what follows. On the text screen, the result text may still start with that sentence, and the screen does not paint the working page a second time. The caller rules are `requirement-rust-cli-interface` and `requirement-rust-tui`.

**Flags:** `--views`, `--grid`, `--output`, and a folder operand are legal only on `model`. On any other verb they are an error. `hello`, `join`, `list-videos`, and `outline` are unknown verbs.

**Non-goals:** joining media, calling FFmpeg or any other encoder, a shell installer, a recursive image walk, a prompt on the typed verb, COLMAP, and a background-removal download.

**Output files:**

| File | Grammar | Sample basename |
|------|---------|-----------------|
| glTF model | binary glTF 2.0, one mesh, `POSITION` and `NORMAL`, indices | `model.glb` |
| Viewer | one HTML document, a canvas, the mesh embedded, no network request | `viewer.html` |

`model.glb` begins with the ASCII bytes `glTF`. Its JSON chunk names asset version `2.0`. `viewer.html` contains `<canvas` and does not contain a `script src=` URL.

Complete views sample this pillar owns:

```json
{
  "views": [
    {"file": "front_detailed_outline.png", "azimuth": 0, "elevation": 0},
    {"file": "side_detailed_outline.png", "azimuth": 90, "elevation": 0}
  ]
}
```

`file` is one basename in the chosen folder. `azimuth` and `elevation` are numbers in degrees.

Complete invocation samples this pillar owns:

```text
rust3d model
rust3d model photos
rust3d model photos --views views.json --grid 96
cargo run -- model
```

### 2.2 Pillar B — The folder board

Menu row 1, kind `model`, short `model`, opens layer `folders`.

| Row | Short | Kind | Effect |
|-----|-------|------|--------|
| **1** | `current` | `pick:.` | Build from the current directory |
| **2** … | the directory name | `pick:` plus that name | Build from that immediate child |
| **0** | Back | `back` | Return to the front board. Nothing is built |

Child rows are immediate directories whose names do not start with `.`, sorted by name ignoring case. `model` and `__pycache__` are listed when they exist. A hidden name is omitted. `.` as a child name, `..`, and a name that contains a separator are refused. Esc on this board returns to the front board.

The explain on row 1 and on each child follows the menu language. The short tokens `model` and `current`, and each directory name, stay as written. Opening this board does not show the waiting sentence. After a folder pick, and only when pillar A says a process is about to start, the screen shows that sentence before the build. The result page is the build lines. It does not show that waiting page a second time. There is no second question.

### 2.3 Pillar C — Help

`rust3d help` and `cargo run -- help` **MUST** describe the glTF build and **MUST NOT** list `hello`, `join`, `list-videos`, `outline`, or an encoder. Help is a typed verb. It is not a numbered menu row.

The help page **MUST** include:

| Help row | Text intent |
|----------|-------------|
| Model | `model` builds one folder and does not draw the menu. With no folder, it uses the current directory. The files are `model.glb` and `viewer.html` in that folder's `model` directory |
| Menu | On a terminal, `rust3d` with no words shows the front board. Row 1 lists the current folder, each subfolder, and back |
| Absence | This program does not join media and does not require FFmpeg |

### 2.4 Pillar D — About

The domain sentence on the about page is `Build a glTF model and an HTML viewer from outline images in a folder`. The page, the host check, and the star box are `requirement-rust-about`.

| Field | Content |
|-------|---------|
| Product name | rust3d |
| Version | `VERSION`, the same string as `Cargo.toml` (current `1.0.1`) |
| Domain summary | Build a glTF model and an HTML viewer from outline images in a folder |

The page **MUST** use that domain sentence. The runtime-tools line **MUST** be `none`. It **MUST NOT** name FFmpeg. `image`, `serde_json`, `unicode-width`, and `chrono` are cargo crates. They are not a host binary on that line.

### 2.5 Implementation Notes (this project)

| Item | Value |
|------|--------|
| **Product / package name** | `rust3d` |
| **Console script** | `rust3d` |
| **Checkout entry** | `cargo run` opens the menu. `cargo run -- model` calls the same build as `rust3d model` |
| **Conversion** | `convert_folder` in `src/model.rs` |
| **Menu row** | Front row 1, kind `model`, short `model`. Folder board layer `folders` |
| **Output** | `<folder>/model/model.glb` and `<folder>/model/viewer.html` by default |
| **Inputs** | `.webp` `.png` `.jpg` `.jpeg`, this folder only |
| **Views** | `--views`, or `<folder>/views.json` when present, otherwise equal azimuth |
| **Grid** | `--grid`, default 96, allowed 16 through 160 |
| **VERSION** | `1.0.1` (`lib.rs` and `Cargo.toml`) |
| **Bootstrap parent** | OutlineImage. This product does not write outline images. It reads them |
| **Absent** | verbs `hello`, `join`, `list-videos`, and `outline`, FFmpeg, rembg |
| **CLI SSOT** | `requirement-rust-cli-interface` |
| **Screen** | `requirement-rust-tui` |
| **User docs** | Root `README.md` |

### 2.6 Why This Requirement Exists (CIAO)

- **Principle 2 – Intentional**: The folder, the grid, and the camera angles are explicit. Empty argv does not build.
- **Principle 5 – SSOT**: One Active domain file. One conversion function for the verb and the menu.
- **Principle 1 – Caution**: An empty folder does not load the vision stack. A missing folder fails closed. Source images are not deleted.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, `model` and the menu run as the normal user. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to write the model. Do not pipe a downloaded script into a shell.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Do not load the vision stack when the folder has no supported image.
- **Intentional:** One build, two surfaces, one glTF file and one HTML viewer.
- **Anti-fragile:** README, help, about, the terminal, and row 1 name the same output.
- **Over-protect:** Keep the sole Active domain file.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Add `hello`, `join`, `list-videos`, `outline`, or FFmpeg without an explicit user order and a new revision of this file.
2. Add a shell installer or root elevation as silent domain behavior.
3. Create a second Active `requirement-domain-*` without superseding this one.
4. Make the typed verb prompt, or make empty argv start a build.
5. Scan subfolders for images, or write the model somewhere other than `<folder>/model` unless the caller passed an output directory.
6. Import the vision stack when the chosen folder has no supported image.
7. Start a build without the waiting sentence in pillar A, show a download sentence, show the sentence for an empty folder, a missing folder, a rejected grid, a bad views file, or a failed output-directory create, or store the sentence in a module-level constant.

**Violating this rule is a critical domain regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `rust3d model` builds from the current directory, does not draw the menu, and does not prompt |
| AC-2 | The files are `model.glb` and `viewer.html` in `<folder>/model` unless `--output` is set |
| AC-3 | Menu row 1 is `model`. The next board is **1** current folder, numbered subfolders, and **0** back |
| AC-4 | `hello`, `join`, `list-videos`, and `outline` are unknown verbs |
| AC-5 | An empty folder returns 0 and does not import the vision stack |
| AC-6 | Help and about do not describe a join or an encoder. The domain sentence is the one in pillar D |
| AC-7 | This file stays the sole Active domain SSOT |
| AC-8 | Before building, the operator sees the pillar A sentence. An empty folder, a missing folder, a rejected grid, a bad views file, and a failed output-directory create do not show it. The sentence is not a module-level constant |
| AC-9 | `model.glb` is glTF 2.0. `viewer.html` embeds the mesh and does not request a network script |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-video-ffmpeg-pipeline` | Retired. Do not restore |
| `requirement-rust-cli-interface` | Typed `model`, `--views`, `--grid`, `--output`, and the folder operand |
| `requirement-rust-about` | About page. This file keeps the domain sentence |
| `requirement-rust-tui` | Row 1 opens the folder board |
| `requirement-rust-oop` | `model.rs` holds the conversion functions |
| `requirement-rust-dependency-management` | Pip strings for the vision stack and StatusLog |
| `requirement-rust-packaging` | Manifest shape |
| `requirement-runtime-prerequisites` | Those cargo crates. No encoder |
| `requirement-rust-error-handling` | Missing folder, bad grid, one failed image |

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Domain requirements

**Definition:** **Domain requirements** are **project requirements** that specify the specialized product’s **domain product law**—beyond Type 0 CLI self-management. They **MUST** cover at least:

1. **Specialized CLI subcommands** (domain verbs, operands, flags, dispatch routing)  
2. **Specialized features** (semantics, state/storage, identifiers, machine contracts, non-goals). When the product allocates or converts files: **filename grammar + sample basename** and **content structure + complete sample per variant** (field tables alone are not enough; glossary-only samples do not count)  
3. **Specialized project help items** (what `help` must list for domain: verbs, flags, examples, env that are domain-specific)  
4. **Specialized project about items** (what `about` must expose for domain diagnostics, if any—beyond Type 0 install presence)

They are **created or updated after** the bootstrap specialize product process has (or is completing) identity/channel retarget and domain extension on product **B**—not as a substitute for that process, and not by rewriting bootstrap origin **A**’s law into domain DNA.

**Single source of truth (current domain law):** A specialized product with a domain surface **MUST** have **exactly one Active** domain-requirements file as the **current domain SSOT** (registered in `docs/requirements/index.md`). Agents **MUST** treat that file as the only authoritative domain catalog (subcommands, features, help items, about items). Parallel Active domain-law files are forbidden unless one **supersedes** the other in the same change. **Exception:** genesis template — no specialized product domain; **no** domain SSOT is required (empty registry / no domain file is honest). **Bootstrap project** (`bootstrap-project.md`) — baseline origin-class product **may not have** domain requirements; honest empty domain SSOT is valid.

**Human daily-life explanation:**

**Domain requirements** are this product’s **written extra-job law** — specialized commands, features (with real samples when files are allocated), help items, and about items — beyond install/self-update. A domain product has exactly one Active file as current SSOT. Genesis and bootstrap may honestly have none.

**Daily-life example:** A café’s extra-job binder lists how takeout works, with a filled sample ticket — not just “we also serve food.” That binder is **Domain requirements**; opening/closing the shop is a different binder.

### Well-known time-consuming process

**Definition:** A **well-known time-consuming process** is a closed catalog of operator work that is known to take long enough that the selected choice must show one sentence **before** that work starts. The catalog is **downloading an AI model** and **converting images**. The sentence is `{choice} has been selected. {process} takes time to finish.` The choice token is the verb, the script name, or the menu-row short the operator selected. The process words are `Downloading the AI model` or `Converting images`. The sentence is shown only when that process is about to start. A choice that only opens a submenu does not announce. An empty input, a missing path, a rejected format, a failure to create the output directory, language, Exit, version, about, and a package-manager lifecycle do not announce. Stay honest: do not say the model is downloading when the weights file is already on disk. On a terminal the sentence is printed and flushed before the blocking call. On a text screen it is painted and refreshed and does not wait for a key. The same sentence may lead the returned result text. A caller that also prints that returned text must not print the sentence a second time.

**Human daily-life explanation:**

A **well-known time-consuming process** is a dish the kitchen already knows will take a while. The waiter says the dish you picked and that it takes time, then goes to the kitchen. Opening the menu book is not that dish. A glass of water that is already on the table is not announced as “we are still filling the glass.”

**Daily-life example:** You point at “today’s photos” on the board. The board says that choice has been selected and that converting the pictures takes time, and only then does the work start. If you only open the folder list and walk away, nobody pretends the oven is already on. If the model jar is already in the cupboard, the waiter does not say they are still fetching it.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-05 | Active 1.0.1 | Product version **1.0.1** |
| 2026-10-05 | Active 1.0.2 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-05
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
