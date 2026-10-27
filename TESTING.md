# Testing checklist

🔧 = needs the trait or pure-function refactor first (hardware).
⚠️ = depends on environment variables.

Suggested order: `fields.rs` → `commands/*` value and validation functions → `profile.rs` → `services` → event handlers → rendering.

## Tests that will probably fail (start here)

- [ ] `Config::system()` returns an error naming the wrong device (check `role`)
- [x] `ChargingLimit::new`: upper == lower is rejected (replaced the old `set_upper` / `set_lwoer` / `enabled()` mismatch)
- [ ] Battery page lets lower go above upper, and `Applied::desired` builds a `ChargingLimit` without validating it
- [x] Performance page: press Down 10 times, then check `mode_input.state.selected()` (`select_next` doesn't clamp until render)
- [x] `OptionField::increment` / `value()` with an empty `options` vec
- [x] Pressing `'0'` in `handle_key`: which page do you land on?
- [ ] `services::execute` diff path: how perf/battery `apply` results are handled vs lighting

## predatorsense (library)

### `commands/lighting.rs`
- [x] `Speed::new` (clamps above 9; 0 and 9 stay as they are)
- [x] `Speed::max`
- [x] `Speed::value`
- [x] `Zone::value` (every variant; bitmask)
- [x] `Direction::value`
- [x] `Effect::value` (every variant)
- [x] `Target::value`
- [x] `LightingCommand::apply` (byte layout; brightness clamped to 100) 🔧

### `commands/battery.rs`
- [x] `ChargingLimit::new` (valid, enabled flag kept, >100, lower >= upper, edges 0/100)
- [ ] `ChargingLimit::apply` (enabled vs disabled bytes) 🔧
- [ ] `ChargingLimit::from_system` (parses bytes 7–9) 🔧

### `commands/performance.rs`
- [x] `PerfMode` repr values and `from_repr` (0–4 valid, 5 gives `None`)
- [ ] `PerfMode::apply` 🔧
- [ ] `PerfMode::from_system` (valid byte; unknown byte gives `UnknownPerfMode`) 🔧

### `commands/backlight_timeout.rs`
- [ ] `set_backlight_timeout` 🔧 (private, currently unused)

### `config.rs`
- [x] `Config::offline`
- [x] `Config::rgb` (offline gives `Unavailable { role: "rgb" }`)
- [ ] `Config::system` (offline gives `Unavailable`; check the role)
- [ ] `Config::open` / `default` (hardware, or after making it not panic)

### `error.rs`
- [x] `Display` output for each variant (e.g. `NotFound` shows `0cf2:5130`)

### `hid.rs`
- [x] `HidDevice::open_by_vid_pid` (not-found tested; permission-denied path untested)

## predatorsense-tui (binary)

### `tui/state/fields.rs`
- [x] `NumericField::increment` (step, clamp at max)
- [x] `NumericField::decrement` (clamp at min, including min > 0)
- [x] `BooleanField::toggle`
- [x] `ListField::default` (selects 0)
- [x] `ListField::increment` / `decrement` (bounds!)
- [x] `OptionField::new`
- [x] `OptionField::select_value` (found / not found)
- [x] `OptionField::value`
- [x] `OptionField::increment` / `decrement` (clamps; empty vec)
- [x] `SliderField::new` (value starts at min)
- [x] `SliderField::set_value` (in range returns true; out of range clamps and returns false)
- [x] `SliderField::increment` / `decrement`

### `tui/state/battery.rs`
- [x] `BatteryPageInput::prev` (clamps at first)
- [x] `BatteryPageInput::next` (clamps at last)
- [x] `BatteryPageState::new` (copies the limit)
- [x] `BatteryPageState::default`
- [x] `BatteryPageState::get_active_input_mut` (right field for each input)

### `tui/state/performance.rs`
- [x] `PerformancePageState::new` (selects matching mode index)
- [x] `PerformancePageState::default` (Normal)

### `tui/state/color_picker.rs`
- [x] `ColorPickerState::next_input` / `prev_input` (clamping)
- [x] `ColorPickerState::get_active_input_mut`
- [x] `ColorPickerState::sync` (preset goes to sliders)
- [x] `ColorPickerState::current_color`
- [x] `ColorPickerState::default`
- [x] `From<Color> for ColorPickerState` (RGB vs non-RGB colour)

### `tui/state/lighting/effects.rs`
- [x] `LightingEffect::capabilities` (every variant)
- [x] `Display` strings for `EffectDirection` ("Left to Right")

### `tui/state/lighting/targets.rs`
- [x] `KeyboardState::field_mut` (`Target` gives `None`)
- [x] `KeyboardState::visible_inputs` (Static vs Breathing vs Wave)
- [x] `LogoState::field_mut` (`EffectDirection` gives `None`)
- [x] `LogoState::visible_inputs`
- [x] `ModeButtonState::field_mut`
- [x] `ModeButtonState::visible_inputs`

### `tui/state/lighting/mod.rs`
- [x] `LightingPageState::new` (profile applied)
- [x] `LightingPageState::selected_target`
- [x] `LightingPageState::get_active_input_mut`
- [x] `LightingPageState::active_color_field_mut`
- [x] `LightingPageState::visible_inputs` (per target)
- [x] `LightingPageState::next_input` / `prev_input` (clamping; active input not visible)

### `tui/state/lighting/profile.rs`
- [x] `LightingProfile::apply_to` (values land in state; out-of-range clamps; effect not available)
- [x] `From<&LightingPageState> for LightingProfile`
- [x] Round trip: state → profile → state is equal
- [x] TOML round trip: `toml::to_string` → `from_str`
- [x] Partial TOML (missing fields fall back to defaults via `#[serde(default)]`)
- [x] `rgb_from` (non-RGB gives the default)
- [x] `to_color`
- [x] `restore_slider`

### `store.rs`
- [x] `default_path` (XDG set, XDG relative so ignored, only HOME, neither) ⚠️
- [x] `load` (valid file, missing file, bad TOML)
- [x] `save` (creates parent dirs, no `.tmp` left behind, overwrites)
- [x] `save` then `load` round trip

### `services/mod.rs`
- [x] `get_effect_from_state` (every variant)
- [x] `get_color_from_state` (RGB vs other)
- [x] `get_direction_from_state`
- [x] `lighting_commands` (3 commands, correct targets and zones)
- [x] `Applied::desired` (perf index mapping, battery fields, out-of-range perf index)
- [x] `execute` (same state gives `Ok` with no device; `None` old state + offline config gives `Err`; diff path) 🔧

### `tui/event.rs`, `event/*.rs`
- [x] `EventHandler::handle_key` (`q` sets Done; `1`–`3` switch page; `0`; `9`)
- [x] `EventHandler::set_active_page` (out of range ignored)
- [x] `EventHandler::get_active_page_event_handler`
- [x] `EventHandler::handle_event` (use `Config::offline()`)
- [x] `BatteryPageEventHandler::handle_page_event` (Up/Down/Left/Right/Space)
- [x] `PerformancePageEventHandler::handle_page_event`
- [x] `LightingPageEventHandler::handle_page_event` (Enter on Color opens picker)
- [x] `LightingPageEventHandler::handle_modal_event` (sliders update colour; Esc closes)
- [x] `LightingPageEventHandler::modal_open`

### `tui/app.rs`
- [x] `App::persist_lighting` (unchanged means no write; changed means write and update `last_saved`)
- [x] `load_battery` / `load_performance` (offline config gives defaults)

### `tui/utils.rs`
- [x] `Action::new`
- [x] `Action::to_str`
- [x] `cursor` (focused / unfocused)

## Rendering (skipped, not planned)

- [ ] `View::render` (smoke test: no panic at 80×24 and at tiny sizes like 10×5)
- [ ] `View::render_header` (active page highlighted)
- [ ] `View::actions` (global plus per-page)
- [ ] `BatteryPage::actions` / `PerformancePage::actions` / `LightingPage::actions`
- [ ] `inputs.rs`: `Input::render` for `BooleanField`, `NumericField`, `OptionField`, `ColorField`
- [ ] `SliderField::color_for`
- [ ] `ColorPickerPopup::actions`
