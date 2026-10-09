# Testing checklist

🔧 = needs the trait or pure-function refactor first (hardware).
⚠️ = depends on environment variables.

Suggested order: `fields.rs` → `commands/*` value and validation functions → `profile.rs` → `services` → event handlers → rendering.

## Tests that will probably fail (start here)

- [ ] `Config::system()` returns an error naming the wrong device (check `role`)
- [ ] `ChargingLimit::set_upper` / `set_lwoer` vs `enabled()`: what happens when upper == lower?
- [ ] Battery page lets lower go above upper, and `Applied::desired` builds a `ChargingLimit` without validating it
- [ ] Performance page: press Down 10 times, then check `mode_input.state.selected()` (`select_next` doesn't clamp until render)
- [ ] `OptionField::increment` / `value()` with an empty `options` vec
- [ ] Pressing `'0'` in `handle_key`: which page do you land on?
- [ ] `services::execute` diff path: how perf/battery `apply` results are handled vs lighting

## predatorsense (library)

### `commands/lighting.rs`
- [ ] `Speed::new` (clamps above 9; 0 and 9 stay as they are)
- [ ] `Speed::max`
- [ ] `Speed::value`
- [ ] `Zone::value` (every variant; bitmask)
- [ ] `Direction::value`
- [ ] `Effect::value` (every variant)
- [ ] `Target::value`
- [ ] `LightingCommand::apply` (byte layout; brightness clamped to 100) 🔧

### `commands/battery.rs`
- [ ] `ChargingLimit::disabled`
- [ ] `ChargingLimit::enabled` (valid, >100, lower >= upper, edges 0/100)
- [ ] `ChargingLimit::set_enabled`
- [ ] `ChargingLimit::set_upper` (>100, below lower, equal to lower)
- [ ] `ChargingLimit::set_lwoer` (>100, above upper, equal to upper)
- [ ] `ChargingLimit::apply` (enabled vs disabled bytes) 🔧
- [ ] `ChargingLimit::from_system` (parses bytes 7–9) 🔧

### `commands/performance.rs`
- [ ] `PerfMode` repr values and `from_repr` (0–4 valid, 5 gives `None`)
- [ ] `PerfMode::apply` 🔧
- [ ] `PerfMode::from_system` (valid byte; unknown byte gives `UnknownPerfMode`) 🔧

### `commands/backlight_timeout.rs`
- [ ] `set_backlight_timeout` 🔧 (private, currently unused)

### `config.rs`
- [ ] `Config::offline`
- [ ] `Config::rgb` (offline gives `Unavailable { role: "rgb" }`)
- [ ] `Config::system` (offline gives `Unavailable`; check the role)
- [ ] `Config::open` / `default` (hardware, or after making it not panic)

### `error.rs`
- [ ] `Display` output for each variant (e.g. `NotFound` shows `0cf2:5130`)

### `hid.rs`
- [ ] `HidDevice::open_by_vid_pid` (permission / not-found mapping; hard to test, fine to skip)

## predatorsense-tui (binary)

### `tui/state/fields.rs`
- [ ] `NumericField::increment` (step, clamp at max)
- [ ] `NumericField::decrement` (clamp at min, including min > 0)
- [ ] `BooleanField::toggle`
- [ ] `ListField::default` (selects 0)
- [ ] `ListField::increment` / `decrement` (bounds!)
- [ ] `OptionField::new`
- [ ] `OptionField::select_value` (found / not found)
- [ ] `OptionField::value`
- [ ] `OptionField::increment` / `decrement` (clamps; empty vec)
- [ ] `SliderField::new` (value starts at min)
- [ ] `SliderField::set_value` (in range returns true; out of range clamps and returns false)
- [ ] `SliderField::increment` / `decrement`

### `tui/state/battery.rs`
- [ ] `BatteryPageInput::prev` (clamps at first)
- [ ] `BatteryPageInput::next` (clamps at last)
- [ ] `BatteryPageState::new` (copies the limit)
- [ ] `BatteryPageState::default`
- [ ] `BatteryPageState::get_active_input_mut` (right field for each input)

### `tui/state/performance.rs`
- [ ] `PerformancePageState::new` (selects matching mode index)
- [ ] `PerformancePageState::default` (Normal)

### `tui/state/color_picker.rs`
- [ ] `ColorPickerState::next_input` / `prev_input` (clamping)
- [ ] `ColorPickerState::get_active_input_mut`
- [ ] `ColorPickerState::sync` (preset goes to sliders)
- [ ] `ColorPickerState::current_color`
- [ ] `ColorPickerState::default`
- [ ] `From<Color> for ColorPickerState` (RGB vs non-RGB colour)

### `tui/state/lighting/effects.rs`
- [ ] `LightingEffect::capabilities` (every variant)
- [ ] `Display` strings for `EffectDirection` ("Left to Right")

### `tui/state/lighting/targets.rs`
- [ ] `KeyboardState::field_mut` (`Target` gives `None`)
- [ ] `KeyboardState::visible_inputs` (Static vs Breathing vs Wave)
- [ ] `LogoState::field_mut` (`EffectDirection` gives `None`)
- [ ] `LogoState::visible_inputs`
- [ ] `ModeButtonState::field_mut`
- [ ] `ModeButtonState::visible_inputs`

### `tui/state/lighting/mod.rs`
- [ ] `LightingPageState::new` (profile applied)
- [ ] `LightingPageState::selected_target`
- [ ] `LightingPageState::get_active_input_mut`
- [ ] `LightingPageState::active_color_field_mut`
- [ ] `LightingPageState::visible_inputs` (per target)
- [ ] `LightingPageState::next_input` / `prev_input` (clamping; active input not visible)

### `tui/state/lighting/profile.rs`
- [ ] `LightingProfile::apply_to` (values land in state; out-of-range clamps; effect not available)
- [ ] `From<&LightingPageState> for LightingProfile`
- [ ] Round trip: state → profile → state is equal
- [ ] TOML round trip: `toml::to_string` → `from_str`
- [ ] Partial TOML (missing fields fall back to defaults via `#[serde(default)]`)
- [ ] `rgb_from` (non-RGB gives the default)
- [ ] `to_color`
- [ ] `restore_slider`

### `store.rs`
- [ ] `default_path` (XDG set, XDG relative so ignored, only HOME, neither) ⚠️
- [ ] `load` (valid file, missing file, bad TOML)
- [ ] `save` (creates parent dirs, no `.tmp` left behind, overwrites)
- [ ] `save` then `load` round trip

### `services/mod.rs`
- [ ] `get_effect_from_state` (every variant)
- [ ] `get_color_from_state` (RGB vs other)
- [ ] `get_direction_from_state`
- [ ] `lighting_commands` (3 commands, correct targets and zones)
- [ ] `Applied::desired` (perf index mapping, battery fields, out-of-range perf index)
- [ ] `execute` (same state gives `Ok` with no device; `None` old state + offline config gives `Err`; diff path) 🔧

### `tui/event.rs`, `event/*.rs`
- [ ] `EventHandler::handle_key` (`q` sets Done; `1`–`3` switch page; `0`; `9`)
- [ ] `EventHandler::set_active_page` (out of range ignored)
- [ ] `EventHandler::get_active_page_event_handler`
- [ ] `EventHandler::handle_event` (use `Config::offline()`)
- [ ] `BatteryPageEventHandler::handle_page_event` (Up/Down/Left/Right/Space)
- [ ] `PerformancePageEventHandler::handle_page_event`
- [ ] `LightingPageEventHandler::handle_page_event` (Enter on Color opens picker)
- [ ] `LightingPageEventHandler::handle_modal_event` (sliders update colour; Esc closes)
- [ ] `LightingPageEventHandler::modal_open`

### `tui/app.rs` (blocked by `Config::default()` panicking)
- [ ] `App::persist_lighting` (unchanged means no write; changed means write and update `last_saved`)
- [ ] `load_battery` / `load_performance` (offline config gives defaults)

### `tui/utils.rs`
- [ ] `Action::new`
- [ ] `Action::to_str`
- [ ] `cursor` (focused / unfocused)

## Rendering (TestBackend, do these last)

- [ ] `View::render` (smoke test: no panic at 80×24 and at tiny sizes like 10×5)
- [ ] `View::render_header` (active page highlighted)
- [ ] `View::actions` (global plus per-page)
- [ ] `BatteryPage::actions` / `PerformancePage::actions` / `LightingPage::actions`
- [ ] `inputs.rs`: `Input::render` for `BooleanField`, `NumericField`, `OptionField`, `ColorField`
- [ ] `SliderField::color_for`
- [ ] `ColorPickerPopup::actions`
