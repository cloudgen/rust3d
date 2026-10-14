# Tests — rust3d

Executable proof for product law. **Design map:** `docs/reviews/test-plan.md`.  
**RTM:** `docs/reviews/requirement-test-matrix.md`.

## Status (2026-10-11, product 1.0.1)

| Item | State |
|------|--------|
| Design TP map | **present** under `docs/reviews/test-plan.md` |
| Automated suite | `tests/product.rs` covers **TP-STRUCT-01**, **TP-PKG-02**, **TP-DEP-01** through **TP-DEP-03**, **TP-PRE-01**, **TP-CLI-01**, **TP-CLI-04**, **TP-MODEL-01** through **TP-MODEL-03**, **TP-LANG-01**, **TP-TUI-09**, **TP-TUI-10**, **TP-ABOUT-01**, **TP-LOG-01**, and **TP-DOC-01** (have). **TP-DOC-03**, **TP-LOG-02**, and **TP-TUI-02** stay `todo` |
| Runner | `tests/run.sh` (`cargo test -- --test-threads=1`) |
| Last run | 2026-10-11 `./tests/run.sh` — 4 tests, OK |

## Layout

| File | TP families |
|------|-------------|
| `product.rs` | The have rows in `docs/reviews/test-plan.md` |

## Rules

1. Assert messages **MUST** include the **TP-ID** when they name a mapped case.  
2. Run image cases only with **generated fixtures** in temp dirs.  
3. Core cases **MUST NOT** require a public network.  
4. Flip `todo` → `have` in `docs/reviews/test-plan.md` only after a green run.  
5. Do not place executable tests under `docs/templates/`.

## Run

```bash
./tests/run.sh
```
