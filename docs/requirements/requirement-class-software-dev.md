**file**: docs/requirements/requirement-class-software-dev.md
**Status**: Active (Version 1.1.8 – Menu language is front row 7. Product version stays 1.0.1)
**Area**: class
**Key**: `requirement-class-software-dev`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

Declare this workspace as a software-development project class and hold the residual collection of software-engineering stack facts not already owned by more specific Active peer requirements: primary language, toolchain policy, package and build tooling, and runtime OS family.

This file is class law plus the residual SSOT. It is not a second copy of the outline domain, the CLI surface, the text menu, the logger construct, the struct map, packaging tables, or error-handling tables. Those stay on peer requirements.

### 1.1 Human-facing

**In one sentence:** This file records that rust3d is Rust, built with cargo, and it points at the requirement that owns each other topic.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Maintainer checking which file owns a topic | The menu picture is the TUI requirement. The language row stays here |
| The other role | Peer requirements | CLI, TUI, logging, domain, packaging |
| Not this file | The outline steps, the frame glyphs, and the cargo command strings | Their own requirements. This file only points |

| Includes | Excludes |
|----------|----------|
| Language, interpreter, package tool, and the residual pointer table | A second copy of the menu, the logger construct, or the struct map |
| Shell install and root elevation marked absent | A cargo lifecycle marked absent. That lifecycle is owned by the CLI and the TUI |
| One human operator. No dest approver | A new actor-requirement file |

| Surface | What you open | What for |
|---------|---------------|----------|
| This file | Residual table | Which requirement owns the topic |
| `Cargo.toml` | Package tool facts | The concrete manifest. Packaging owns the tables |
| `rust3d` | The program | You run it as the normal user |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Find the menu law | Open the TUI requirement. Do not expect the picture in this file. | `rust3d` |
| Find the logger law | Open the logging requirement. The floor is packaging and runtime. | `rust3d version` |
| Install | Use cargo. There is no shell installer. | `cargo install rust3d` |

## 2. Core Rules (Mandatory)

### 2.0 Project class membership

1. **MUST** treat this workspace as software-development (shippable software), not genesis-template and not server-maintenance.
2. **MUST** use basename `requirement-class-software-dev.md` as the sole Active class-law file for this class.
3. **MUST NOT** register an Active `requirement-class-server-maintenance.md` while the class is software-development.
4. **MUST** retain portable harness knowledge when present. Specialized product knowledge lives in this and peer `requirement-*.md` files.
5. **MUST** apply the software-development SSOT and gate posture when claimed (identity, package ship surface, and a pre-commit check when git is used — as applicable).
6. **MUST NOT** invent hollow product docs solely to look specialized.

### 2.1 Residual collection principle (SSOT hygiene)

7. **MUST** treat this file as the default home for software-stack facts not owned by another Active requirement.
8. **MUST NOT** duplicate full normative tables that already live in a more specific Active requirement. Prefer a one-line pointer to the peer requirement key.
9. When a new specialized requirement takes ownership of a topic previously only listed here, **MUST** update this file in the same change: shrink the residual entry and point to the new owner.
10. **MUST NOT** leave contradictory stack facts across this file and peer requirements.

### 2.2 Programming language(s)

11. **MUST** declare at least one primary programming language for the ship unit.
12. **SHOULD** list secondary languages only when they are real product law.
13. **MUST** state whether the product is primarily interpreted, compiled, polyglot, or package-multi-language.
14. **MUST NOT** freeze a marketing product name as if it were the language name.

### 2.3 Compilers, interpreters, and toolchains

15. **MUST** declare the target toolchain class used to build or run the product.
16. **MUST** state version policy as one of: unconstrained, minimum version, range, or pinned.
17. **SHOULD** record whether cross-compilation is in scope.
18. **MUST** fail closed in docs claims: do not claim “supports all interpreters” without tests or an explicit unconstrained policy.

### 2.4 Project / package / build tools

19. **MUST** declare the primary project or package tool used for dependencies and builds.
20. **MUST** declare how dependencies are resolved when the ecosystem supports lockfiles.
21. **SHOULD** name the test runner and linter or formatter classes when they are project law.
22. **MUST NOT** require a secret token or a private registry password in this file.

### 2.5 Runtime and platform (residual)

23. **MUST** declare the intended primary runtime and OS family when not fully owned by another architecture requirement.
24. **SHOULD** declare minimum CPU or architecture support only when it is real product law.
25. **MUST** separate developer-machine toolchain requirements from end-user runtime requirements when they differ.

### 2.6 No-hardcode / dual policy (class file)

26. **MUST NOT** hard-code a single product brand, one organization’s production hostname, or a personal owner identity as universal core law.
27. **MUST** put the live product name, the repo slug, and the concrete stack choices in Implementation Notes.
28. **MUST NOT** store secrets, PATs, or toy credentials in this file.

### 2.7 Implementation Notes (this project)

| Field | Value (rust3d) |
|-------|---------------------|
| **Project display name** | rust3d |
| **Project class** | software-development |
| **Class requirement basename** | `requirement-class-software-dev.md` |
| **Primary language(s)** | Rust |
| **Language role** | primary only for the runtime package under `src/` |
| **Execution model** | interpreted package. Optional Cython or `build.sh` tooling is not required for the CLI |
| **Toolchain / interpreter** | rustc |
| **Toolchain version policy** | range declared in `Cargo.toml` (`rust-version`). Re-verify before advertising a specific minor as tested |
| **Cross-compile in scope?** | no |
| **Primary project/package tool** | cargo via the Cargo manifest `Cargo.toml` |
| **Lockfile policy** | not used as product law |
| **Test runner** | none as project law today. Proof rows in `docs/reviews/test-plan.md` stay todo |
| **Linter/formatter** | none as project law |
| **Primary runtime / OS family** | multi-OS where rustc runs (documented focus: Linux; macOS and Windows when the dependencies exist) |
| **Architectures supported** | any architecture with rustc |
| **Git surface** | used — remote `https://github.com/cloudgen/rust3d` |
| **Ship surface** | Rust crate `rust3d`; the only binary is `rust3d` |
| **Product version SSOT** | `src/lib.rs` → `VERSION` and `Cargo.toml` `package.version` stay equal when either is bumped (current: **1.0.1**) |
| **Install mode** | cargo / local crate. Not a shell online-install product |
| **Type 1 elevation** | intentionally absent. No root or sudo product surface |
| **Actor / role / subject / approver** | **Considered — no dest approver.** The human operator of `rust3d` is the only actor. No dest approval machine. **None** is valid. Do not add an actor requirement file |
| **Dest fence conditions** | **Considered — no dest fence conditions.** No dest inbound, approve, or reject queue |
| **Author contact (non-secret)** | Wilgat Wong · `wilgat.wong@gmail.com` |

**Residual ownership table:**

| Topic | Owner | Notes |
|-------|-------|--------|
| Project class membership | **this file** | Fixed |
| Primary language + toolchain policy | **this file** | Rust / rustc and cargo |
| Package/build tool + lockfile | **this file** + `requirement-rust-packaging` | packaging owns `Cargo.toml`. Cargo strings are `requirement-rust-dependency-management` |
| Project layout | `requirement-rust-project-structure` | Running tree versus the struct map |
| Struct homes | `requirement-rust-oop` | One class per file. Do not duplicate the map |
| CLI entry / typed verbs | `requirement-rust-cli-interface` | Empty argv and the verb table |
| Text menu | `requirement-rust-tui` | Front board and the picture |
| Menu language | `requirement-rust-cli-language` | Front row 7, the thirteen codes, and the language leaf |
| Status logger | `requirement-rust-cli-logging` | Construct in `fn main` |
| Domain surface | `requirement-domain-rust3d` | Four pillars. The about page uses the domain sentence |
| About page | `requirement-rust-about` | Identity, host check, and the star box |
| Encoder / concat | `requirement-video-ffmpeg-pipeline` | Retired. Not an ops SSOT |
| Rust coding style / temps / `std::fs::rename` | `requirement-rust-coding-style` | Publish. StateLogic stays unordered |
| Error / fail-closed user messaging | `requirement-rust-error-handling` | Console sentences |
| Host runtime deps | `requirement-runtime-prerequisites` | No host encoder. Pip strings are `requirement-rust-dependency-management` |
| Shell online install / shell self-update / Type O | **intentionally absent** | Not a shell channel |
| Cargo lifecycle (`version-check`, `self-update`, `self-install`, `self-uninstall`) | `requirement-rust-cli-interface` and `requirement-rust-tui` | `cargo` only. Not a shell channel |
| Type 1 sudoers / root elevation | **intentionally absent** | No elevation law |
| Actor / role / subject / approver | **this file (residual)** | Considered — no dest approver |
| Dest fence conditions | **this file (residual)** | Considered — no dest fence conditions |

## 3. Why This Requirement Exists (Direct CIAO Alignment)

- **CIAO Principle 2 – Intentional**: Class and stack choices are explicit.
- **CIAO Principle 5 – SSOT**: Residual facts stay here until a peer takes the topic. The menu, the logger, and the struct map have already moved.
- **CIAO Principle 1 – Caution**: This file does not invent a shell installer.
- **CIAO Principle 21 – Dual Policies**: Pointers stay short. Concrete values sit in Implementation Notes.
- **CIAO Principle 4 (O) + Principle 20**: One class file. No second stack SSOT.

## 4. Design Principles (CIAO / CIAO-Lite)

- **Caution**: This file does not install a host tool. The image stack is cargo crates.
- **Intentional**: The residual table points. It does not copy peer bodies.
- **Anti-fragile**: Packaging identity stays in `Cargo.toml`.
- **Over-protect**: Shell Type O stays absent while cargo lifecycle is claimed on the CLI and the TUI.

## 5. Protection Rule (Sacred)

**Future AI assistants, Grok, or maintainers MUST NOT**:

1. Delete this file while the workspace remains software-development with other Active product requirements.
2. Rename the specialized basename away from `requirement-class-software-dev.md` without an explicit class-model change.
3. Hard-code secrets, personal tokens, or production host names into core rules as universal law.
4. Duplicate full peer requirement bodies into this residual section.
5. Leave Implementation Notes hollow while Status is Active.
6. Introduce a shell online-install channel, a `curl|sh` installer, or shell Type O empty-argv install-ensure. Cargo lifecycle owned by the CLI and TUI requirements is not that channel.
7. Introduce Type 1 sudoers or root elevation without an explicit user order and an elevation allowlist.
8. Treat this file as server-maintenance allowlist law, or register an Active server-maintenance class file in parallel.
9. Invent a second primary language that contradicts the Rust requirements.
10. Add an actor-requirement file while the residual says no dest approver.
11. Mark cargo lifecycle absent, or mark shell Type O present.

**Violating any of these is a critical regression.**

## 6. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | The sole Active class file is `requirement-class-software-dev.md` |
| AC-2 | Primary language is Rust with the rustc toolchain |
| AC-3 | The package tool is cargo plus `Cargo.toml` |
| AC-4 | The residual table points at the CLI, TUI, logging, OOP, and domain peers without copying their bodies |
| AC-5 | Shell Type O and Type 1 elevation are absent. Cargo lifecycle is owned by the CLI and the TUI |
| AC-6 | Registered in `docs/requirements/index.md` with Area `class` |
| AC-7 | Actor residual stays “no dest approver”. No actor requirement file |

## 7. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-packaging` | Manifest and entry point |
| `requirement-rust-dependency-management` | Pip strings |
| `requirement-rust-project-structure` | Layout |
| `requirement-rust-oop` | Class map |
| `requirement-rust-cli-interface` | Typed verbs and empty argv |
| `requirement-rust-tui` | Text menu |
| `requirement-rust-cli-logging` | Logger construct |
| `requirement-domain-rust3d` | Domain four pillars |
| `requirement-video-ffmpeg-pipeline` | Processing ops |
| `requirement-rust-coding-style` | Temps and `std::fs::rename` |
| `requirement-rust-error-handling` | Console errors |
| `requirement-runtime-prerequisites` | No host encoder. Points at the cargo strings |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| n/a | packaging and structure smoke | deferred | Class residual is exercised by TP-PKG, TP-STRUCT, TP-LOG, TP-TUI, and TP-OOP when those rows leave todo |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, rust3d runs as the normal user. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`. Cargo lifecycle, when the operator asks for it, is `cargo` from the CLI and TUI requirements.

## 7. Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Software-development project

**Definition:** A **software-development project** is one of the **eleven peer project classes** (class #2) whose primary deliverable is **shippable software** (CLI, library, service, app, or bootstrap Type 0 seed). It **retains harness knowledge** and **adds specialized product knowledge**: product law + product proof + product user surface when claimed — optional **git-versioned** tree, identity/contact/channel **SSOTs** (README / LICENSE / Stars / Config), **precommit / file-leaks** when git is used, and (when applicable) ship unit, install channel, tests, and public `reviews/`. It **MUST** consider actor / role / subject / approver even if dest approver is **None**. It **MUST review** dest fence-condition requirements and convert each dest **Fence** row to an independent REQ (honest **none** if dest does not refuse inbound). It **MUST** have a coding-style related requirement (specialize-in home for portable learned lessons; **without it, those lessons arrive raw**; honest **none** is **not** valid). It is **not** server-maintenance, **not** genesis-template, **not** policy, **not** marketing-analysis, and **not** cmos-design.

**Human daily-life explanation:**

A **software-development project** is a house whose job is to *make a thing people can run or install*. It keeps the shared workshop, then adds this product’s rules, tests, and front-door sign. It is not the janitor clipboard class, and it is not an empty seed with no product.

**Daily-life example:** You are writing a kitchen timer people can download. That is this class. Rewriting only the building’s allowed-chore list is a different class.

### Project requirements

**Definition:** **Project requirements** are the workspace’s **product law** — durable normative documents under `docs/requirements/` that state what *this* engineering system **must / should** do. They specialize templates into **complete, project-scoped rules**, including **parameterized project choices** (app type, path models, privilege-actor models). Every `requirement-*.md` **MUST** print **§1.1 Human-facing** (human-intro voice: one sentence, three boxes, includes/excludes, surfaces, practice) and **MUST NOT** lead with Type 0 / Type 1 / euid / F6 as the only words. They are **not** reusable blanks, **not** skill procedures, **not** a separate human-intro page, and **not** a dumping ground for a single frozen product brand string as if it were the only possible configuration.

**Human daily-life explanation:**

**Project requirements** are this product’s must/should law under `docs/requirements/`. They are finished, project-scoped rules, not blank molds.

**Daily-life example:** “This household must lock the door by 10” is law for this house; a blank “curfew rule” form in a stationery shop is not.

### Coding-style requirement

**Definition:** A **coding-style requirement** is the software-development **specialize-in home** for portable learned lessons about how the ship unit is written. Every software-development project **MUST** have an Active language-matched coding-style related `requirement-*.md`. **Without that file, agents bring portable learned lessons raw** (from coding skills and law molds) and treat them as this product’s law. The coding-style REQ is where those lessons are adopted, pointed at peer REQs, or refused.

**Human daily-life explanation:**

A **Coding-style requirement** is this product’s **adopted writing style home**. Every software-dev project must have one. Without it, helpers would treat portable lessons as this product’s law raw. The REQ is where those lessons are taken, pointed, or refused.

**Daily-life example:** A newsroom keeps one style sheet (“we write dates this way”). That adopted sheet is the **Coding-style requirement**; the industry handbook on the shelf is not automatically house law until the sheet says so.

### Rust crates.io CLI

**Definition:** A **Rust crates.io CLI** is a **software-development** product whose primary operator surface is a **short-lived console script** installed from **Cargo**: `Cargo.toml` is the packaging SSOT, ship source lives under **`src/`**, and the operator runs the **declared console entry** (not a long-running HTTP process). The default coding shape is **thin `src/cli.rs`** plus a **StateLogic + Attr** orchestrator (OOP **L3**). It **does not** use `backend/` + `frontend/` as the ship tree.

**Human daily-life explanation:**

A **rust-crates-cli** is a short-lived console command installed with cargo: `Cargo.toml`, source under `src/`, declared binaries. It is not a backend/frontend diner.

**Daily-life example:** A mixer you install from a catalog listing, with a named “on” switch—not a café that stays open with a dining room.

This product's package name is `rust3d` (`Cargo.toml` `package.name`), the import package is `rust3d` under `src/`, and the console script is `rust3d`.

## 8. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-08-09 | Active 1.0.0 | Initial software-dev class law for rust3d |
| 2026-10-04 | Active 1.1.0 | Residual pointers for the text menu, the logger, the struct map, and cargo lifecycle. Shell Type O stays absent |
| 2026-10-04 | Active 1.1.1 | Menu language points at `requirement-rust-cli-language`. No new actor file |
| 2026-10-04 | Active 1.1.2 | Residual row for the about page. Product version **1.0.5** |
| 2026-10-05 | Active 1.1.3 | Pip floors point at `requirement-rust-dependency-management`. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.1.4 | Git surface is `https://github.com/cloudgen/rust3d`. Product version stays **1.0.0** |
| 2026-10-05 | Active 1.1.5 | Product version **1.0.1** |
| 2026-10-05 | Active 1.1.6 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
| 2026-10-05 | Active 1.1.7 | Terminologies name the package `rust3d`, matching `package.name`. The console script stays `rust3d`. Product version stays **1.0.1** |
| 2026-10-14 | Active 1.1.8 | Menu language is front row 7. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-14
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
