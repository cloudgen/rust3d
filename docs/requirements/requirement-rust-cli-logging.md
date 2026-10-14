**file**: docs/requirements/requirement-rust-cli-logging.md
**Status**: Active (Version 1.0.5 – Terminologies section before status history. Product version stays 1.0.1)
**Area**: rust
**Key**: `requirement-rust-cli-logging`
**Philosophy**: CIAO **v2.10.2** / CIAO-Lite (Caution • Intentional • Anti-fragile • Over-engineered / Over-protect)

## 1. Purpose

`fn main` instantiates StatusLog. The instantiation is the statement `StatusLog::new` written in that function, before the argument parser and before any product object.

rust3d records system status through that one StatusLog instance: process start, debug identity, menu steps, and failures. User-visible outline sentences stay on `requirement-rust-error-handling`. The text menu stays on `requirement-rust-tui`. Struct homes stay on `requirement-rust-oop`. The cargo name and the version floor stay on `requirement-rust-dependency-management`. This product does not log an encoder. `requirement-video-ffmpeg-pipeline` is Retired.

This product creates no threads. This file does not name a thread-wait timeout. Control-C stop, an exit code for that stop, and the decision not to publish are not confirmed law in this file. Row 9 Exit returns 0 with no confirm question (`requirement-rust-tui`).

The running `src/cli.rs` writes `StatusLog::new` in `fn main`. The allowed end state is this file.

### 1.1 Human-facing

**In one sentence:** `fn main` writes `StatusLog::new`, rust3d keeps a dated status file through that object, and those lines stay off the text menu.

| Box | Meaning | Example |
|-----|---------|---------|
| You / this login | Person running `rust3d` | A failed outline is still explained on the console |
| The other role | The logger library | `StatusLog(logname="rust3d")` resolves the folder and the line shape |
| Not this file | How the menu is drawn, and how one image becomes an outline | `requirement-rust-tui`, `requirement-domain-rust3d` |

| Includes | Excludes |
|----------|----------|
| One logger, read-back of name and folders, the debug gate, `log_message` level and component, the `instantiated` line, and the temp / publish / discard lines | Rewriting StatusLog, a second logging setup, painting the menu with log lines, a copied folder ladder, a thread pool |
| `is_quiet=True` on the constructor when this run will open the text screen | A later `quiet(True)` as the only way to hide `Created directory:` |
| The daily file when the text screen is open | Mirroring identity lines onto that screen |

| Surface | What you open | What for |
|---------|---------------|----------|
| `src/cli.rs` | `fn main` | Writes `StatusLog::new`, then passes that one instance |
| `rust3d` | console script | Writes status while it runs |
| Resolved `logDir()` | daily `.log` file | History after the process exits |

| You do… | What it means | What you type |
|---------|---------------|---------------|
| Start the program | `fn main` writes one `StatusLog::new` with logname `rust3d`, reads the resolved name and folders back, and logs status with a component tag. | `rust3d version` |
| Ask for the debug lines | Set `DEBUG` before the process starts. On a run that does not open the text screen, `main` displays the resolved name, the package version, `StatusLog.class_version()`, and a line that says `debug mode`, before the argument parser. The text screen keeps that mirror off the console. The daily file still receives the lines. When debug mode is off, those lines stay off. | `DEBUG=1 rust3d version` |
| Open the menu | The constructor is quiet, so log lines do not paint over the frame. The menu is still the text screen. | `rust3d` |

## 2. Core Rules (Mandatory)

**Construct site.** `fn main` instantiates the one StatusLog. The instantiation is the statement `StatusLog::new` written in that function, before the argument parser and before any product object. Rule 5 is this law.

### 2.0 When this law applies

1. **MUST** send system-status history through StatusLog.
2. **MUST NOT** add `logging.basicConfig`, a second file logger, or a hand-built folder path for those lines.
3. **MUST NOT** use this file as the law for editing the StatusLog library itself.

### 2.1 One logger

4. **MUST** import `StatusLog` from `StatusLog` at the use site inside `fn main`. **MUST NOT** import `_Suroot`. A test that freezes the clock **MUST** define its own `TimeProvider` subclass in the test module. **MUST NOT** import `FakeTimeProvider` from the package.
5. **MUST** construct **one** instance per process by writing `StatusLog::new` inside `fn main`, with `logname` set to the product name `rust3d`. That statement is the construct. **MUST NOT** put the construct in a function or a method. **MUST NOT** call `Cli.__new__` in order to build the logger. Helpers receive that instance. **MUST NOT** construct a second logger.
6. **MUST** read `logName()`, `baseDir()`, and `logDir()` immediately after construct, and again after any path setter, before the first `log_message`. The constructor string is not the resolved name. `rust3d` resolves to `rust3d`.
7. **MUST NOT** re-export `StatusLog` from the `rust3d` package. Public exports stay `VERSION` and `main` (`requirement-rust-packaging`).
8. If the import fails, **MUST** fail closed on the console with the cargo next step for the floor named by `requirement-rust-packaging`, and **MUST** return non-zero. That line cannot go through `log_message`. Importing the `rust3d` package **MUST** still succeed when StatusLog is absent, because the construct is not at import time.

### 2.2 Debug

9. **MUST** ask `logger.isDebug()` for whether debug status is on. **MUST NOT** read `DEBUG` again in product code. **MUST NOT** add a `--debug` flag.
10. `isDebug()` remembers the first answer. `DEBUG` or `debug` **MUST** already be `1`, `true`, or `show` (any letter case) **before** `StatusLog::new`.
11. The startup identity lines **MUST** run only inside `if logger.isDebug():`, and `main` **MUST** run that block before the argument parser. The block is four `log_message` calls, each with `component="main"`: the resolved app name, the package version, and this file’s path; `Using` plus `StatusLog.class_version()`; the `baseDir()` value; a line whose message is `debug mode`. **MUST NOT** type the library version by hand. When `isDebug()` is false, those lines **MUST NOT** be displayed.
11a. **Console mirror.** When this run will open the text screen, and that fact is known before construct, `main` **MUST** pass `is_quiet=True` into `StatusLog::new`. The runs that open the text screen are: empty argv on a terminal, and `about` on a terminal. `model` does not open the text screen. A small argv peek may decide `is_quiet` before the constructor. `ArgumentParser.parse_args` runs after the constructor and after the debug block. The library stores that flag before it resolves the name and the folders and before it creates the log directory. That constructor can print `Created directory:`. A call to `quiet(True)` after the constructor returns **MUST NOT** be the only quiet for those runs. A run that does not open the text screen **MUST** leave `is_quiet` false, so the operator sees `debug mode` on the console. The text screen **MUST** keep that mirror off the console. The same lines still go to the daily file. This product does not claim `--json`. `logger.quiet(True)` remains the setter when a screen opens later and was not known at construct.
11b. **Version string.** The identity line and the `version` verb read the same package version: the single string `VERSION` in `src/lib.rs`, which matches `Cargo.toml` (`requirement-rust-packaging`). The current string is `1.0.1`. **MUST NOT** add `MAJOR` / `MINOR` / `PATCH` integers. **MUST NOT** keep a second version literal for display. A suite assertion may compare the displayed string with `VERSION` and with `Cargo.toml`. `main` **MUST NOT** raise because those strings differ.
11c. **Environment checks.** This product **MUST NOT** probe a virtualenv, pyenv, or conda. Those results **MUST NOT** be printed on the about page. The host-check reads stay on `requirement-rust-about`.

### 2.3 `log_message` level and component

12. System-status lines **MUST** call `log_message`. The component **MUST** be the keyword `component`.
13. Allowed `component` values for this product:

| Component | Status it marks |
|-----------|-----------------|
| `main` | Process start, version, missing library, top-level failure |
| `menu` | Text menu open, choice, leave, view-log, clear-log |
| Class name | That object was constructed, and later status recorded by that object |

The class-name components are `Cli`, `Tui`, `MenuPainter`, `MenuModel`, `MenuSession`, `MenuScreenError`, `SystemLog`, `SelfManage`, `CheckSystem`, and `AboutPage`. There is no component `join` and no struct `Join`.

### 2.3a Pass the logger into every class

13a. `main` **MUST** decide quiet and run the debug block before it constructs struct `Cli`.
13b. Every product struct that logs **MUST** accept that same logger in `::new` and store it. The constructor home is `requirement-rust-oop`.
13c. When the logger is present, `::new` **MUST** log `instantiated` with the struct name as the component. The level is INFO. The call **MUST NOT** sit inside the debug check. **MUST NOT** call a module function to write that line.
13d. A class that constructs another product class **MUST** write `OtherClass(...)` at that site and pass that same logger.
13e. **MUST NOT** construct a StatusLog inside a class. **MUST NOT** construct it from a method of `Cli`.
13f. A caller that has no logger may omit it. `main` has the logger and **MUST** pass it.
13g. The text screen **MUST** already be quiet, so these lines stay off that console. The daily file still receives them. A run that does not open the text screen **MUST** leave quiet off, so the operator sees each `instantiated` line.

### 2.3b Major file operations

13h. This product does not write a concat temp, does not publish a media file with `std::fs::rename`, and does not discard an encoder temp. **MUST NOT** add those operations in order to satisfy an older log line. If a later change publishes a file from a temporary path, that publish **MUST** call `log_message` with the operation and the paths before `std::fs::rename`. The component is `main`. The level is `INFO`.
13i. **MUST NOT** use component `join`. Outline result lines are the console sentences in `requirement-rust-error-handling`. The call **MUST NOT** sit inside `if logger.isDebug():` when a status line is required. **MUST NOT** use `print` for durable status. **MUST NOT** construct a second logger.
13j. Control-C stop, the exit code for that stop, and the decision not to publish are not confirmed law in this file. This file does not define exit 130. Row 9 Exit returns 0 with no confirm (`requirement-rust-tui`). A clock redraw is not that exit.
13k. Quiet from rule 11a still applies. The daily file still receives the line.

### 2.3c Threads

13l. The running product creates no threads. **MUST NOT** add a thread, a pool, or a wait timeout in order to satisfy this file.
13m. If a later requirement adds a thread, that change **MUST** call `log_message` on the one StatusLog before the call that can block or race. The operations are: create, start, join or other wait, lock acquire, lock release, and handing work to a pool. The line names the action, the thread name, and the object it waits on when there is one. The `component` is the class that performs the operation, or `main` when `main` does. Create, start, joined, acquired, and released are `INFO`. The call **MUST NOT** sit inside `if logger.isDebug():`. **MUST NOT** use `print`. **MUST NOT** construct a second logger, including one logger per thread.
13n. `log_message` **MUST NOT** run while a work lock is held. This file does not name a wait timeout.
13o. Quiet from rule 11a still applies. The daily file still receives the line. A thread join is not a substitute for a Control-C law this file does not have.

14. Allowed `level` values: `INFO`, `DEBUG`, `WARNING`, `ERROR`, `FATAL`. The library uppercases the string. `ERROR` and `FATAL` mirror to stderr. The others mirror to stdout. Omitted `level` means `INFO`.
15. A line with `level="DEBUG"` **MUST** sit inside `if logger.isDebug():`.
16. **MUST NOT** put secrets into a message. This product has none. An image path is allowed.

### 2.4 Console, menu, and the file

17. `log_message` still tries to append the daily file when the folder is writable. An unwritable folder **MUST NOT** crash the process.
18. While the text menu is on screen, the console mirror stays off (rule 11a, or `logger.quiet(True)` when the screen was not known at construct). The file write still happens. `requirement-rust-tui` still draws the screen. The screen writer is not a second logger.
19. A run that never opens the menu **MUST** leave quiet off so the operator sees the mirrored status line.
20. A user-visible failure sentence remains owned by `requirement-rust-error-handling`. The same fact **MUST** also be `log_message` at `ERROR` or `FATAL` once the logger exists. While quiet is on, the logger mirror is not a substitute for that console sentence. The missing-library line in rule 8 is the exception: the logger does not exist yet.

### 2.5 Folder and line

21. **MUST** use the folders from `baseDir()` and `logDir()`. **MUST NOT** copy the library’s environment ladder into this program. **MUST NOT** hard-code `~/.app/rust3d`, `/var/rust3d`, or `/var/log/rust3d`.
22. When the process is root and it is not under conda, pyenv, or a virtualenv, the library’s `baseDir()` is `/var/rust3d` and `logDir()` is `/var/rust3d/log`. It is not `/var/log/rust3d`. Product code reads those values. It does not assemble them. This sentence does not require rust3d to install conda, pyenv, or a virtualenv.
23. The daily file basename **MUST** stay the library’s `{kebab-app}-{YYYYMMDD}.log` under `logDir()`. For this product the kebab name is `rust3d`.
24. A file line has this shape (the library writes it; this product does not format it by hand):

```text
[{YYYY-MM-DD HH:MM:SS}] pid:{PID} [{LEVEL}] @{COMPONENT} :] {MESSAGE}
```

25. Product `main` **MUST NOT** pass a hard-coded folder. A proof that needs a scratch folder may supply `basedir` and `logdir` to `main` so the test does not write the operator’s real folder. An empty value leaves resolution to the library.

### 2.6 Implementation Notes (this project)

| Field | Value |
|-------|--------|
| **Library** | StatusLog |
| **Floor** | Required `chrono>=0.4`, owned by `requirement-rust-dependency-management`. The live `Cargo.toml` declares that floor. |
| **Construct site** | `fn main` in `src/cli.rs` writes `StatusLog::new`. |
| **logname** | `rust3d` (product name, not a path, not the console-script name) |
| **Resolved name** | `rust3d` from `logName()` |
| **Call order** | Inside `fn main`: peek argv for the text screen, write `StatusLog::new` with `logname` and `is_quiet` when that screen is already known, read `logName()`, `baseDir()`, and `logDir()`, run the `isDebug()` block including `debug mode`, then parse arguments, then `Cli(logger)`. |
| **Quiet** | `is_quiet=True` for empty argv on a terminal and for `about` on a terminal. `model` leaves it false. Other runs leave it false. `quiet(True)` after return does not hide `Created directory:`. |
| **Version** | Identity and `version` use `VERSION` (`1.0.1`). No second integer triple. `cli.rs` reads `VERSION` and does not keep a second literal. |
| **Threads** | None in the running product. No timeout is named. |
| **Control-C** | Not confirmed law here. |
| **Debug switch** | Environment `DEBUG` or `debug` already set to `1`, `true`, or `show`. No `--debug` flag. |
| **Proof** | `TP-LOG-01`, `TP-LOG-02`, `TP-LOG-03`, `TP-LOG-04`, `TP-LOG-05` stay todo. The ship unit does not pass this law. |

### Sample code

`fn main` writes `StatusLog::new`, reads the resolved names back, runs the debug gate, and then writes `Cli(logger)`. `opens_text_screen` is true only for the runs in rule 11a. A non-screen run passes false.

```python
fn main(argv=None, basedir="", logdir=""):
    """This function instantiates StatusLog. The statement below is the construct."""
    try:
        from StatusLog import StatusLog
    except ImportError:
        sys.stderr.write(
            'StatusLog is required. Next step: cargo install "chrono>=0.4"\n'
        )
        return 1

    from rust3d import VERSION

    screen = opens_text_screen(argv)
    logger = StatusLog(
        logname="rust3d",
        is_quiet=screen,
        basedir=basedir,
        logdir=logdir,
    )
    appname = logger.logName()
    resolved_base = logger.baseDir()
    logger.logDir()

    if logger.isDebug():
        logger.log_message(
            "{0} v{1} ({2})".format(appname, VERSION, __file__),
            component="main",
        )
        logger.log_message(
            "Using {0}".format(StatusLog.class_version()),
            component="main",
        )
        logger.log_message(
            "Base {0}".format(resolved_base),
            level="DEBUG",
            component="main",
        )
        logger.log_message("debug mode", component="main")

    app = Cli(logger)
    return app.run(argv)
```

That function is `fn main`. The statement `StatusLog::new` instantiates the logger there. Rule 5 keeps this statement in `fn main`.

Before a temp write, a publish, or a discard:

```python
logger.log_message(
    "write temp path={0}".format(temp_path),
    level="INFO",
    component="main",
)
```

### 2.7 Why This Requirement Exists (CIAO)

- **Principle 5 – SSOT of output**: status history has one writer.
- **Principle 1 – Caution**: a missing library or an unwritable log folder does not wipe the source videos or smash the menu.
- **Principle 2 – Intentional**: level and component are named on every status line.
- **Principle 12 – Traceability**: the daily file still receives lines while the text screen is quiet.

## Under command line for normal user only

On Termux, Git Bash, Windows cmd, or the same class, status files stay in the folder StatusLog resolves for this login. **This requirement:** do not use administrator privilege, `sudo`, `apt`, or a dedicated system user to create the log folder or to install StatusLog. Do not pipe a downloaded script into a shell. Type 1 and Type 2 are unused on Termux, Git Bash, and Windows cmd. Git Bash and Windows cmd do not call Termux `pkg`. The next step is `cargo install "chrono>=0.4"` as the normal user.

## 3. Design Principles (CIAO / CIAO-Lite)

- **Caution:** Quiet the console mirror before the text menu. Do not crash when the log folder is unwritable.
- **Intentional:** One instance. Components stay `main`, `menu`, `join`, and the class name.
- **Anti-fragile:** Folder choice stays inside StatusLog. Product code reads `logDir()`.
- **Over-protect:** No second logger and no package re-export of StatusLog.

## 4. Protection Rule (Sacred)

**Future AI assistants or maintainers MUST NOT:**

1. Construct StatusLog outside `fn main`, or construct a second logger.
2. Re-export StatusLog from the rust3d package.
3. Hard-code `~/.app/rust3d`, `/var/rust3d`, or `/var/log/rust3d`, or copy the library folder ladder into product code.
4. Treat `quiet(True)` after the constructor returns as a substitute for `is_quiet=True` on a text-screen run.
5. Put the identity lines outside `logger.isDebug()`, or add a `--debug` flag.
6. Add a second version literal, or raise at import or in `main` because version strings differ.
7. Mark `TP-LOG-*` have while `src/` does not construct the logger.
8. Invent a thread, a timeout, a Control-C protocol, or exit 130 in order to satisfy this file.
9. Drop the console failure sentence because a log line exists.
10. Print `in_venv`, `in_pyenv`, or `in_conda` on the about page, or use those booleans to choose a location line.

**Violating this rule is a critical logging regression.**

## 5. Acceptance criteria

| ID | Criterion |
|----|-----------|
| AC-1 | The construct is the statement `StatusLog::new` inside `fn main`, before the parser and before `Cli` |
| AC-2 | `logName()`, `baseDir()`, and `logDir()` are read before the first `log_message` |
| AC-3 | Identity lines and `debug mode` appear only inside `isDebug()`, and the text screen does not mirror them |
| AC-4 | Missing StatusLog prints the cargo next step and returns non-zero |
| AC-5 | This product does not log component `join`. A later publish-from-temp logs before `std::fs::rename`, component `main` |
| AC-6 | Floor `chrono>=0.4` is required in law. The live manifest lag is stated. The package does not re-export the library |

## 6. Related requirements (peer keys only)

| Key | Relationship |
|-----|----------------|
| `requirement-rust-packaging` | Floor owner. Live manifest lags |
| `requirement-runtime-prerequisites` | Floor and the missing-import next step |
| `requirement-rust-cli-interface` | When the text screen opens. Parser order |
| `requirement-rust-tui` | Screen that must stay quiet |
| `requirement-rust-oop` | Logger parameter on every class |
| `requirement-rust-error-handling` | Console failure sentences |
| `requirement-rust-coding-style` | `std::fs::rename` and temps |
| `requirement-video-ffmpeg-pipeline` | Concat stages that write those temps |
| `requirement-rust-about` | Host-check path reads. `TP-ABOUT-16` proves the three logger booleans |
| `requirement-class-software-dev` | Residual pointer |
| `docs/requirements/index.md` | Registry |

## Design-time verification

| TP family / ID | Suite | Status | Note |
|----------------|-------|--------|------|
| **TP-LOG-01** | `tests/test_logging.py` | todo | One construct in `main`. `logName` / `baseDir` / `logDir` read back. Resolved name is `rust3d` |
| **TP-LOG-02** | `tests/test_logging.py` | todo | `DEBUG` already on shows identity lines and `debug mode` on a non-screen run. Unset shows neither. A text-screen run does not mirror those lines. The daily file still receives them |
| **TP-LOG-03** | `tests/test_logging.py` | todo | The product creates no threads. A later thread logs create, start, join, wait, lock, and pool before the blocking call, and does not call `log_message` under a work lock |
| **TP-LOG-04** | `tests/product.rs` | todo | Each logging struct logs `instantiated` at INFO, component the struct name, outside the debug check |
| **TP-LOG-05** | `tests/test_logging.py` | todo | No status line uses component `join` |
| **TP-ABOUT-16** | `tests/product.rs` | have | `in_venv`, `in_pyenv`, and `in_conda` call the one logger and do not select location lines. Primary owner is `requirement-rust-about` |

**Matrix:** `docs/reviews/requirement-test-matrix.md`
**Map:** `docs/reviews/test-plan.md`

## Terminologies

The words this requirement uses. The meaning is copied here so the requirement stands alone.

### Chronicle logger

**Definition:** **Chronicle-logger** is the one process status logger, library StatusLog, that `fn main` instantiates by writing the statement `StatusLog::new` inside that function, before any product object. It records the resolved application name, the version triple, this file's path, `StatusLog.class_version()`, the base folder, and a line whose message is `debug mode`, and those identity lines run only inside `isDebug()`. Each class stores that same object and logs INFO `instantiated`. A major file operation, a thread creation, a thread operation, and an interrupt line use the same object through `log_message`. A thread line is written before the call that can block, and `log_message` is not called while a work lock is held. The log folder is the one `logDir()` returns. StatusLog already chooses it. Product code does not copy that ladder and does not type the path. Not `print`. Not a second logger. Not a thread pool.

**Formal name (people-facing):** the one status notebook for this run. The workshop name stays **chronicle-logger**.

**Human daily-life explanation:**

A **chronicle-logger** is the one kitchen notebook. When a new appliance arrives you write "got the blender." When you move a jar from the counter to the shelf you write the move and both places. When someone starts a second task, or stands at a locked cupboard, you write who and which cupboard before they wait. You do not start a second notebook, and you do not write in the book while you are still holding the cupboard key that the other person needs. The notebook already has a shelf. You ask where that shelf is. You do not invent a second cupboard, and you do not nail the address into the recipe.

**Daily-life example:** You buy a kettle and write "kettle arrived" in the household book. Later you move the kettle from the box to the shelf and write "moved kettle, box to shelf." A helper says they will wait at the pantry lock, and you write "Alex waiting on pantry" before they stand there. If they never come back, the book already says who is waiting. The guests in the dining room do not hear that line. The book still has it. At home the shelf is under the household counter. Inside a work tent the shelf stays in that tent. When you are the building owner and there is no tent, the shelf is the building cupboard named for the notebook, not the mail room.

### Logger default quiet unless verbose

**Definition:** **Logger-default-quiet-unless-verbose** is the console-mirror rule for the one process status logger. The daily file still receives every line. The terminal does not, unless the operator sets `--verbose` on a run that is not `--json` and is not a text screen. `is_quiet` is chosen on the constructor `StatusLog::new` inside `fn main`, before the library creates the log folder. `--json` and a text screen stay quiet even when `--verbose` is also set, because that library mirrors a line to the same stream it would use when it is not quiet. There is no `--quiet` flag. `DEBUG` still decides whether the identity lines are written at all. It does not, by itself, put them on the terminal.

**Formal name (people-facing):** the status notebook stays in the drawer unless you ask to hear it. The workshop name stays **logger-default-quiet-unless-verbose**.

**Human daily-life explanation:**

**Logger-default-quiet-unless-verbose** is the household rule for the kitchen notebook. You still write “kettle arrived” in the book. You do not read that line out loud in the dining room unless someone asks to hear the notebook. A guest who asked for a written receipt (`--json`) still does not get the notebook read over the receipt. A room that is already showing a wall chart (the text screen) does not get the notebook spoken over the chart, even if someone asked to hear it.

**Daily-life example:** The kettle arrives. You write “kettle arrived” in the book and leave the book in the drawer. Dinner stays quiet. A guest says “read me the book” (`--verbose`) while you are only talking, and you read the line. The same guest asks for a receipt instead, and the receipt stays one page. The book still has the line. You do not add a second switch whose only job is “be quiet,” because quiet is already the usual way.

## 7. Status history

| Date | Status | Note |
|------|--------|------|
| 2026-10-04 | Active 1.0.0 | StatusLog construct for rust3d. Ship unit does not construct it yet. Floor target `chrono>=0.4`. Live manifest remains `StatusLog>=1.2.3` |
| 2026-10-04 | Active 1.0.1 | Product version **1.0.4**. Manifest floor is `chrono>=0.4`. `cli.rs` reads `VERSION` only |
| 2026-10-04 | Active 1.0.2 | Rule 11c: environment checks read the one logger. Current version string is **1.0.5**. `TP-LOG-*` stay todo. `TP-ABOUT-16` is have. Primary owner is `requirement-rust-about` |
| 2026-10-05 | Active 1.0.3 | The StatusLog floor string is owned by `requirement-rust-dependency-management`. The floor stays `>=1.3.1` |
| 2026-10-05 | Active 1.0.4 | Current version string is **1.0.1** |
| 2026-10-05 | Active 1.0.5 | Terminologies section copies the words this requirement uses. Product version stays **1.0.1** |
---

**Last Updated**: 2026-10-05
**Owner**: project maintainers
**Alignment**: Registry `docs/requirements/index.md`; **CIAO** (https://github.com/cloudgen/ciao); CIAO-Lite (https://github.com/cloudgen/ciao-lite).

## Covered on 2026-10-11

This file is **Active** for rust3d. The ship unit is the Rust crate. Cargo is the lifecycle. StatusLog is in-tree. Product version stays **1.0.1**.
