**file**: docs/requirements/requirement-rust-packaging.md
**Status**: Active (Version 1.2.9 – One binary, `rust3d`, and `default-run` is that binary. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-packaging`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Define the packaging SSOT for the rust3d crate: `Cargo.toml`, metadata, dependencies, binaries, and version consistency.

This file owns the manifest shape. The cargo requirement strings are `requirement-rust-dependency-management`. StatusLog stays required. `requirements.txt` is not an authority.

### 1.1 Human-facing

**In one sentence:** rust3d is a cargo package named rust3d, version 1.0.1. The status library and the image stack are required dependencies.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Person installing the package | `cargo install rust3d` |
| The other role | The manifest | `Cargo.toml` names the version, the console script, and the dependencies |
| Not this file | What the menu does after install, and how an outline image is drawn | CLI, TUI, and the domain file |

| Includes | Excludes |
|----------|----------|
| Package name, version, console script `rust3d`, required `chrono>=0.4` | Re-exporting StatusLog from the rust3d package |
| Manifest floor `chrono>=0.4`, same as this law | Calling the logger optional, or leaving the manifest at `>=1.2.3` |
| MIT license and the homepage as declared | A claim that cargo installs FFmpeg. FFmpeg is not a dependency |

| Surface | What you open | What for |
|---------|---------------|----------|
| `Cargo.toml` | `[project]` | Name, version, dependencies, console script |
| `src/lib.rs` | `VERSION` | The same version string |
| cargo | Install | `cargo install rust3d` |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Install | cargo installs rust3d, the status library, and the image stack. It does not install FFmpeg. | `cargo install rust3d` |
| Check the version | The badge, the manifest, and `VERSION` are the same string. | Open `Cargo.toml` and `lib.rs` |
| Use the library | You get `VERSION`. You do not get StatusLog re-exported as a public crate item. | `cargo test` |

## 2. Core Rules (Mandatory)

### 2.1 Manifest SSOT

1. `Cargo.toml` **MUST** be the primary packaging manifest.
2. **MUST** declare: name, version, description, authors, license, edition, dependencies, and exactly one binary, `rust3d`. **MUST NOT** declare a second binary. `default-run` **MUST** be `rust3d`, so `cargo run` with no `--bin` starts that binary.
3. **MUST NOT** treat an ad-hoc `requirements.txt` as the primary product dependency SSOT.
4. A legacy `setup.py` under a build tree **MUST NOT** become a second source of runtime identity.

### 2.2 Version SSOT

5. The package version in `Cargo.toml` and `src/lib.rs` (`VERSION`) **MUST** match when a release is claimed.
6. Bumping either **MUST** update both in the same change.
7. **MUST NOT** invent a third version constant. Display code **MUST NOT** keep a second literal, including a fallback `"1.0.0"` (`requirement-rust-oop`). The current release string is `1.0.1`. A version bump is a user order, not a side effect of editing this file.

### 2.3 Dependencies

8. **MUST** declare runtime crates required for the shipped CLI.
9. The cargo strings **MUST** be the list in `requirement-rust-dependency-management`. `chrono>=0.4`, `image>=0.25`, `serde_json>=1`, and `unicode-width>=0.2` **MUST** stay required. StatusLog stays in-tree and **MUST NOT** become a crate floor, each with the specifier in that file. The live `Cargo.toml` `[dependencies]` **MUST** be that same list.
10. FFmpeg **MUST NOT** be declared as a cargo package. This product does not require that binary (`requirement-runtime-prerequisites`).
11. **MUST NOT** commit secrets or private index passwords into packaging files.
12. **MUST NOT** re-export `StatusLog` from the rust3d package.

### 2.4 Entry points

13. **MUST** declare the only binary `rust3d` from `src/bin/rust3d.rs`, and that file **MUST** call `rust3d::cli::main`. **MUST NOT** declare `convert` or any other `[[bin]]`.
14. The entry function **MUST** stay `fn main` in `src/cli.rs`. In the allowed end state, `main` constructs StatusLog and then `Cli` (`requirement-rust-cli-logging`, `requirement-rust-oop`).

### 2.5 Build / release helpers

15. Optional `build.sh` **MAY** exist for maintainer packaging. It **MUST NOT** run `cargo publish`.
16. **MUST** keep helper scripts consistent with `Cargo.toml` identity (project name rust3d).
17. Generated `build/` and `dist/` **MUST NOT** be treated as source SSOT.

### 2.6 Implementation Notes (this project)

| Item | Value |
|------|--------|
| **Manifest** | `Cargo.toml` |
| **Project name** | `rust3d` |
| **Version** | `1.0.1` |
| **Edition** | `2021` |
| **Dependencies (law)** | The four strings in `requirement-rust-dependency-management`: `chrono>=0.4`, `image>=0.25`, `serde_json>=1`, `unicode-width>=0.2` |
| **Dependencies (live manifest)** | The same four strings |
| **Build tool** | cargo |
| **Binary** | The only binary is `rust3d` from `src/bin/rust3d.rs`. It calls `rust3d::cli::main`. `default-run` is `rust3d` |
| **Homepage / repo** | `https://github.com/cloudgen/rust3d` |
| **Maintainer build helper** | `build.sh` |
| **License** | MIT |
| **Public package exports** | `VERSION`, `main` only. **MUST NOT** re-export `StatusLog` |
| **User docs** | Root `README.md` Quick Installation documents cargo and **MUST NOT** claim the install fetches FFmpeg |
| **README version badge** | Must match packaging version when README claims complete (`1.0.1`) |

### 2.7 Why This Requirement Exists (CIAO)

- **Principle 5 – SSOT**: One manifest for identity and entry. One floor for StatusLog.
- **Principle 2 – Intentional**: The live manifest lag is written down instead of being described as optional.
- **Principle 1 – Caution**: FFmpeg is not a dependency. The image stack is.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, install and upgrade use cargo as the normal user. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to install rust3d. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`. The install line is `cargo install rust3d`.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** The version string has one home in the manifest and one matching `VERSION`.
- **Intentional:** `Cargo.toml` is the manifest. StatusLog stays in-tree.
- **Anti-fragile:** The console script and the module entry share `main`.
- **Over-protect:** No secrets in packaging files. No re-export of StatusLog.

## 4. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Remove or rename the `rust3d` entry without a CLI and README update. Add a second binary, including `convert`.
2. Let `VERSION` and `Cargo.toml` diverge while claiming a release.
3. Add private credentials to `Cargo.toml`.
4. Replace the packaging SSOT with only `requirements.txt`.
5. Change the product name silently.
6. Mark StatusLog optional, or leave the manifest at `>=1.2.3` once the packaging file is edited for the floor.
7. Re-export StatusLog from the package.
8. Bump the product version as a side effect of editing this requirement.

**Violating this rule is a critical packaging regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `Cargo.toml` names rust3d |
| AC-2 | The only binary is `rust3d`, and `default-run` is `rust3d` |
| AC-3 | Law and the live manifest both require `chrono>=0.4` |
| AC-4 | Version matches `lib.rs` when a release is claimed (`1.0.1`) |
| AC-5 | FFmpeg is not a cargo dependency. The image strings match `requirement-rust-dependency-management` |
| AC-6 | Public exports are `VERSION` and `main` only |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-project-structure` | Package path |
| `requirement-rust-cli-interface` | Entry behavior |
| `requirement-rust-cli-logging` | Points here for the floor |
| `requirement-rust-dependency-management` | Cargo strings |
| `requirement-runtime-prerequisites` | No host encoder. Points at the cargo strings |
| `requirement-rust-oop` | `main` stays the console target |
| `requirement-class-software-dev` | Residual stack |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-PKG-01** | `tests/product.rs` | have | The only binary is `rust3d`, and `default-run` is `rust3d`. StatusLog is not a crate dependency |
| **TP-PKG-02** | `tests/product.rs` | have | `Cargo.toml` version equals `VERSION` |
| **TP-PRE-02** | `tests/product.rs` | n/a | StatusLog is in-tree. There is no crate floor `>=1.3.1` |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Proper project version bump

**Definition:** A **proper project version bump** is a **user-ordered semver increase** of the product ship version that is **strictly newer** than the previous local ship version, applied in the **same change** on every in-scope surface. When `Cargo.toml` exists, `package.name` is the **app and package name** and `package.version` **equals** the import-package `VERSION`, the ship-unit version, the README Version badge (project target version), and the current CHANGELOG heading. A bump is **not** a side effect of editing a requirement, a skill, or a glossary file. Agents **MUST NOT** invent a second version literal beside the one the user ordered.

**Human daily-life explanation:**

A **proper project version bump** is printing one newer edition number on every copy of the book in the same sitting, because someone asked for a new edition. Editing a workshop note does not print a new edition by itself. When the catalog card exists, the name on the card is the product name and the number on the card is the same number as the title page.

**Daily-life example:** The kitchen card says name `pancake-timer` and edition `1.2.0`. The title page, the cover sticker, and the change list all say `1.2.0`. The next ordered edition is `1.2.1` on all of those surfaces together. A note in the workshop drawer does not become `1.2.1` on its own.

### crates.io build

**Definition:** A **crates.io build** produces the **source distribution and wheel** for a Python package with the **`build`** package (`python -m build`, typically `python -m build --sdist --wheel`). When `./build.sh` exists, the steps are **`clean` then `build`**. **Clean** removes the previous `dist/`, `build/`, and egg-info so a **foreign wheel** is not left in `dist/`. A crates.io build **does not upload** and **does not create a git tag**. Generated `dist/` is **not source**.

**Human daily-life explanation:**

A **crates.io build** is packing the finished goods into the shipping cartons. You empty the old cartons first, then pack this product only. Packing is not handing the cartons to the warehouse, and it is not sticking a new luggage tag on the git history.

**Daily-life example:** You run the pack step. The table that held yesterday’s other brand’s carton is cleared. Today’s source carton and wheel carton are the only ones left. Nothing has been sent to the public index yet.

### crates.io upload

**Definition:** A **crates.io upload** publishes the **sdist and wheel** from a crates.io build to the package index with the **`twine`** package (`python -m twine upload` / `twine upload`). It runs only after a proper project version bump, a **fresh push** of that bump (the remote contains the bump commit), and a local version **strictly newer** than the probed crates.io version (`info.version` for `package.name`). When `./build.sh` exists, the steps are **`clean`**, **`build`**, then **`upload`**. **`release` is not this term** unless the user asked to **tag**. Agents **MUST NOT** upload a dirty `dist/` that holds another package’s artifacts, and **MUST NOT** print index credentials.

**Human daily-life explanation:**

A **crates.io upload** is handing this edition’s cartons to the public warehouse only after three checks: the edition number was raised on purpose, that raised number is already on the public shelf of the source history, and the warehouse’s current catalog number is older. You do not hand over yesterday’s other brand, and you do not read the warehouse password aloud.

**Daily-life example:** The catalog already lists edition `1.2.0`. The books on the table say `1.2.1`, and that `1.2.1` commit is already on the remote. You pack, then send. You do not send a carton whose label is `1.2.0` again, and you do not announce a luggage tag unless someone asked for a tag.

### crates.io version

**Definition:** **crates.io version** is the **semver string currently published** for a Python distribution on the **Python Package Index** (or Testcrates.io when that is the product’s declared upload target)—typically the `info.version` field from the package JSON API (`https://pypi.org/pypi/rust3d/json`) or the equivalent Testcrates.io endpoint. It is a **live registry probe** (display/align surface). The **package name** used on crates.io is **not** invented: when a **crates.io upload script** exists, packaging SSOT (e.g. `Cargo.toml` `package.name`) is the **SSOT for `rust3d`** used in README badges, `cargo install` docs, and probes. Agents **MUST NOT** invent a crates.io version without a successful probe (or an honest “not published / not probeable” gap).

**Human daily-life explanation:**

**crates.io version** is the version string the public Python index currently serves for that package. Probe it; do not invent it.

**Daily-life example:** The edition number the bookstore’s catalog currently lists, which might differ from the draft on your kitchen table.

Do not invent this product's live index number in this requirement. When no successful probe is written down, the number stays unknown.

### Version align

**Definition:** **Version align** (also **version alignment**) is the **evidence-based process** of inventorying every **in-scope product version surface**, comparing them for **consistency** with a single current product version string `the live product version`, and reporting **PASS / DISCREPANCY / N/A / not probeable**—including, when a **crates.io upload script** is present on disk, the live **crates.io version** from the package index. It is **not** a VCS commit, **not** a force of registry state, and **not** permission to invent versions.

**Human daily-life explanation:**

**Version align** is checking that every clock in the house shows the same time as the one agreed current time, then reporting match, mismatch, not applicable, or cannot look. It does not wind the clocks for you, and it does not invent a time that was never written.

**Daily-life example:** You compare the oven clock, the hallway clock, and the posted menu date. You report “hallway is five minutes slow.” You do not secretly rewrite the menu.

### Rust crates.io CLI

**Definition:** A **Rust crates.io CLI** is a **software-development** product whose primary operator surface is a **short-lived console script** installed from **Cargo**: `Cargo.toml` is the packaging SSOT, ship source lives under **`src/`**, and the operator runs the **declared console entry** (not a long-running HTTP process). The default coding shape is **thin `src/cli.rs`** plus a **StateLogic + Attr** orchestrator (OOP **L3**). It **does not** use `backend/` + `frontend/` as the ship tree.

**Human daily-life explanation:**

A **rust-crates-cli** is a short-lived console command installed from Python packaging: `Cargo.toml`, source under `src/`, declared entry. It is not a backend/frontend diner.

**Daily-life example:** A mixer you install from a catalog listing, with a named “on” switch—not a café that stays open with a dining room.

This product's package name is `rust3d` (`Cargo.toml` `package.name`), the import package is `rust3d` under `src/`, and the console script is `rust3d`.

### Ship unit

**Definition:** A **ship unit** is the **primary installable artifact**. For this packaged command, that artifact is the Python package `rust3d`. The version string lives in `Cargo.toml` `package.version` and in the import package `VERSION`. A proper project version bump keeps those two equal. This product's ship unit is not a single root shell file fetched by a script URL.

**Human daily-life explanation:**

A **ship unit** is the bottle you install — the program itself — not the shop window. For this product the bottle is the packaged command `rust3d`.

**Daily-life example:** You install the package and run the console command. That installed command is the ship unit. The project web page is not the bottle.

### Precommit check

**Definition:** **Precommit check** (also **commit-check**) is the **fail-closed hygiene gate** agents **MUST** run when the user asks to **git commit**, **commit-check**, **pre-commit**, or **ready to commit**—before staging/committing product work. For a **specialized product** (not blank genesis template), the gate verifies at least:

1. **`README.md`** — product user entry present, install/usage/version truth (not hollow; no stale integrity pins)  
2. **`CHANGELOG.md`** — release narrative covers the **current product version** (and the bump for this commit when required)  
3. **`SECURITY.md`** — present when security policy is claimed for a specialized product; contact/reporting honest; integrity trust bounds when companion/install integrity exists  
4. **Companion digest / sha256** — for shell self-managed CLIs: the companion `.sha256` **matches** current ship-unit bytes. This packaged command does not use that shell digest.  
5. **Version bump** — product version SSOTs are **aligned** and, for user-visible product changes, **bumped** relative to the previous released version (semver), unless the user explicitly scopes **docs-only / no version bump**  
6. **Git identity alignment** — for specialized + git: **`git config user.name`** matches **repository-user**; **`git config user.email`** matches **author-email**; default **current-git-user** matches **repository-user** **or** DISCREPANCY is documented with a vault that probes as repository-user  
7. **Pre-git SSH profile report** (on mismatch / multi-vault) — name **active** ssh-user-profile and **git-capable** profiles (logins) so teams know who is in `~/.ssh` vs who can actually run git SSH  
8. **Vault-bound push** (when user asked to **push** / **commit and push**) — do **not** bare-push while default face ≠ repository-user; **activate** or one-shot `GIT_SSH_COMMAND -i` the matching vault (the commit gate's vault rule)

It is **not** product law, **not** a substitute for tests or reviews, and **not** permission to force-push or commit without user confirmation on shared/hard-to-reverse git ops.

**Human daily-life explanation:**

A **precommit-check** is the hygiene gate before a git commit: README, changelog, identity, and friends must be honest. It is not permission to force-push.

**Daily-life example:** Checking passport, tickets, and labeled bags before leaving for the station—not the train ride itself.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-08-09 | Active 1.0.0 | Initial packaging law for rust3d |
| 2026-08-09 | Active 1.1.0 | Export honesty and version 1.0.3 |
| 2026-10-04 | Active 1.2.0 | StatusLog is required at `>=1.3.1`. Live manifest still `>=1.2.3`. Product version stays 1.0.3 |
| 2026-10-04 | Active 1.2.1 | Product version **1.0.4**. Manifest floor is `chrono>=0.4` |
| 2026-10-04 | Active 1.2.2 | Product version **1.0.5**. Floor stays `chrono>=0.4` |
| 2026-10-05 | Active 1.2.3 | Product version **1.0.0**. Image stack is required. `rust-version` is `>=3.10` |
| 2026-10-05 | Active 1.2.4 | Pip strings move to `requirement-rust-dependency-management`. Each entry has a version floor. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.2.5 | Homepage, repository, and issues URLs are `https://github.com/cloudgen/rust3d`. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.2.6 | Product version **1.0.1**. `Cargo.toml` and `VERSION` match |
| 2026-10-05 | Active 1.2.7 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-05 | Active 1.2.8 | Terminologies name the package `rust3d`, matching `package.name`. The ship unit is that package. The console script stays `rust3d`. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.2.9 | One binary, `rust3d`. `default-run` is `rust3d`. The `convert` binary is gone. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
