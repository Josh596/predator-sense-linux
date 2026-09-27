# TODO — HID device wiring

Status as of the `hid_device` branch: `HidDevice` is defined in `src/hid.rs` but
never constructed outside that file. None of `commands::lighting::apply`,
`commands::power::set_mode`, or `commands::battery::set_limit` is called from
anywhere. The TUI mutates state only. Everything below is about closing that gap.

## 1. Crate split — make TUI-independence compiler-enforced

- [x] `src/lib.rs` is currently 0 bytes (dead lib target). Fill it with
      `pub mod error; pub mod hid; pub mod commands; pub mod devices;`
- [x] Drop `mod error/hid/commands` from `main.rs`; keep only `mod tui;`
- [x] Rewrite `crate::commands::…` imports in `tui/` as `predatorsense::commands::…`
      (currently just `tui/state/performance.rs:4` → `PerfMode`)
- [x] Verify: `commands/` and `hid/` can no longer name anything in `tui/`

## 2. `Devices` — open once, own for program lifetime

- [ ] New `src/devices.rs` holding `HidApi` + the opened device(s)
- [ ] `Devices::open()` — keep `HidApi` alive (its drop runs `hid_exit`)
- [ ] `Devices::offline()` with all handles `None`, so the TUI runs on a machine
      without the hardware / without udev permissions
- [ ] Decide: one HID handle shared by lighting+power+battery, or one per function?
      (all three `commands::*` fns take `&HidDevice` today)
- [ ] `main` constructs it: `Devices::open().unwrap_or_else(|_| Devices::offline())`

## 3. Getting it to the call site — pick one

- [ ] **A. Direct**: add `dev: &Devices` to `PageEventHandler::handle_page_event`
      and to `EventHandler::handle_event`. Fewer moving parts; hardware calls land
      inside event handlers.
- [ ] **B. Action queue**: handlers push onto `state.pending: Vec<…>`; `App::run`
      (or `main`) drains and executes. Handlers stay pure and unit-testable.
      - [ ] NAME COLLISION: `tui::utils::Action` already exists (keybinding hint).
            Call the new enum `Command` / `DeviceAction`.
      - [ ] Coalesce duplicate writes before draining — holding an arrow key
            currently emits one write per keypress at key-repeat speed.

`set_feature(&self, …)` takes `&self`, so either way `&Devices` is enough —
no `RefCell`, no `&mut` plumbing, no borrow conflict with `&mut ApplicationState`.

## 4. Error surfacing

- [ ] `last_error: Option<String>` (or richer) on `ApplicationState`
- [ ] Render it somewhere — a status line / toast
- [ ] A failed write must never panic the TUI and must never leave the on-screen
      value disagreeing with the hardware without saying so

## 5. State → command translation

- [ ] `LightingPageState` produces **multiple** commands — one per zone.
      So the boundary fn is `to_commands(&self) -> Vec<LightingCommand>`,
      not `to_command()`. Decide where per-zone colour actually lives in the
      state first (deferred — see Open questions).
- [ ] DECIDED: keep TUI display enums separate from `commands::lighting::{Effect,
      Direction, Target}`. Therefore:
      - [ ] Write `impl From<tui::…> for commands::…` for each pair, so adding a
            variant on one side is a compile error rather than a silent mismatch
      - [ ] Keep the conversions in ONE place, not scattered through event handlers

## 6. Testability of the byte layout (optional, do only if you'll write the tests)

- [ ] `trait FeatureReport { fn set_feature(&self, buf: &[u8]) -> Result<()>; }`
      in `hid.rs`; `impl FeatureReport for HidDevice`
- [ ] Change `commands::*` signatures to take `&dyn FeatureReport`
- [ ] Fake device recording sent buffers; tests for the sharp edges:
      - [ ] `Zone` is a BITMASK (`Four == 0x08`), not sequential
      - [ ] `brightness.min(100)` clamp
      - [ ] `0xa4` report ID and field order in `lighting::apply`
      - [ ] A fake that returns `Err` → asserts `last_error` is set, no panic

## Open questions / deferred

- [ ] Per-zone lighting: does `LightingPageState` hold N zone colours, or one
      colour + a zone selector? Drives the shape of `to_commands()`.
- [ ] Does anything need to READ from the device (battery level, current profile)?
      Read-back fits the action queue badly — those want a direct call.
- [ ] `commands/backlight_timeout.rs` exposes no `pub fn` yet — unfinished.
- [ ] `Page::Dashboard` has no event handler (falls through to Battery's).
