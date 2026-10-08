# TODO — HID device wiring



`set_feature(&self, …)` takes `&self`, so either way `&Devices` is enough —
no `RefCell`, no `&mut` plumbing, no borrow conflict with `&mut ApplicationState`.

## 4. Error surfacing

- [ ] `last_error: Option<String>` (or richer) on `ApplicationState`
- [ ] Render it somewhere — a status line / toast
- [ ] A failed write must never panic the TUI and must never leave the on-screen
      value disagreeing with the hardware without saying so



## Open questions / deferred

- [ ] Per-zone lighting: does `LightingPageState` hold N zone colours, or one
      colour + a zone selector? Drives the shape of `to_commands()`.
- [ ] Does anything need to READ from the device (battery level, current profile)?
      Read-back fits the action queue badly — those want a direct call.
- [ ] `commands/backlight_timeout.rs` exposes no `pub fn` yet — unfinished.
- [ ] `Page::Dashboard` has no event handler (falls through to Battery's).
