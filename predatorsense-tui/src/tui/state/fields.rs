use ratatui::{style::Color, widgets::ListState};

pub trait Field {
    fn increment(&mut self) {}
    fn decrement(&mut self) {}
    fn toggle(&mut self) {}
}

pub struct NumericField {
    pub value: u32,
    pub step: u32,
    pub min: u32,
    pub max: u32,
    pub unit: &'static str,
}

impl Default for NumericField {
    fn default() -> Self {
        Self {
            value: 0,
            step: 1,
            min: 0,
            max: 100,
            unit: "",
        }
    }
}

impl Field for NumericField {
    fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
    }
    fn decrement(&mut self) {
        self.value = self.value.saturating_sub(self.step).max(self.min);
    }
}

pub struct BooleanField {
    pub value: bool,
    pub on_text: &'static str,
    pub off_text: &'static str,
}

impl Default for BooleanField {
    fn default() -> Self {
        Self {
            on_text: "On",
            off_text: "Disabled",
            value: false,
        }
    }
}

impl Field for BooleanField {
    fn toggle(&mut self) {
        self.value = !self.value;
    }
}

pub struct ListField {
    pub options: Vec<String>,
    pub state: ListState,
}

impl Default for ListField {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            options: vec![],
            state,
        }
    }
}
impl Field for ListField {
    fn increment(&mut self) {
        let last = self.options.len().saturating_sub(1);
        let next = self.state.selected().map_or(0, |i| (i + 1).min(last));
        self.state.select(Some(next));
    }

    fn decrement(&mut self) {
        self.state.select_previous();
    }
}

pub struct OptionField<T> {
    pub options: Vec<T>,
    pub selected_option_index: usize,
}

impl<T> OptionField<T> {
    pub fn new(options: Vec<T>) -> Self {
        Self {
            options,
            selected_option_index: 0,
        }
    }
}

impl<T: PartialEq> OptionField<T> {
    pub fn select_value(&mut self, value: T) -> bool {
        match self.options.iter().position(|o| o == &value) {
            Some(i) => {
                self.selected_option_index = i;
                true
            }
            None => false,
        }
    }
}

impl<T: Copy> OptionField<T> {
    pub fn value(&self) -> T {
        self.options[self.selected_option_index]
    }
}

impl<T> Field for OptionField<T> {
    fn increment(&mut self) {
        self.selected_option_index =
            (self.selected_option_index + 1).min(self.options.len().saturating_sub(1));
    }

    fn decrement(&mut self) {
        self.selected_option_index = self.selected_option_index.saturating_sub(1);
    }
}

pub struct ColorRange {
    pub start: usize,
    pub end: usize,
    pub color: Color,
}

pub struct SliderField {
    pub value: usize,
    pub min: usize,
    pub max: usize,
    pub step: usize,
    pub unit: &'static str,
    pub ranges: Vec<ColorRange>,
}

impl SliderField {
    pub fn new(min: usize, max: usize, step: usize, unit: &'static str) -> Self {
        Self {
            value: min,
            min,
            max,
            step,
            unit,
            ranges: Vec::new(),
        }
    }
    pub fn set_value(&mut self, value: usize) -> bool {
        let clamped = value.clamp(self.min, self.max);
        self.value = clamped;

        clamped == value
    }
}

impl Field for SliderField {
    fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
    }
    fn decrement(&mut self) {
        self.value = self.value.saturating_sub(self.step).max(self.min);
    }
}

pub struct ColorField {
    pub color: Color,
}

impl Field for ColorField {}

#[cfg(test)]
mod tests {
    use super::*;

    fn numeric(value: u32, step: u32, min: u32, max: u32) -> NumericField {
        NumericField {
            value,
            step,
            min,
            max,
            unit: "",
        }
    }

    fn list(len: usize) -> ListField {
        ListField {
            options: (0..len).map(|i| i.to_string()).collect(),
            ..Default::default()
        }
    }

    // NumericField

    #[test]
    fn numeric_increment_adds_step() {
        let mut field = numeric(10, 5, 0, 100);
        field.increment();
        assert_eq!(field.value, 15);
    }

    #[test]
    fn numeric_increment_clamps_at_max() {
        let mut field = numeric(98, 5, 0, 100);
        field.increment();
        assert_eq!(field.value, 100);
    }

    #[test]
    fn numeric_decrement_subtracts_step() {
        let mut field = numeric(10, 5, 0, 100);
        field.decrement();
        assert_eq!(field.value, 5);
    }

    #[test]
    fn numeric_decrement_clamps_at_zero() {
        let mut field = numeric(2, 5, 0, 100);
        field.decrement();
        assert_eq!(field.value, 0);
    }

    #[test]
    fn numeric_decrement_clamps_at_nonzero_min() {
        let mut field = numeric(10, 5, 8, 100);
        field.decrement();
        assert_eq!(field.value, 8);
    }

    // BooleanField

    #[test]
    fn boolean_toggle_flips_value() {
        let mut field = BooleanField::default();
        field.toggle();
        assert!(field.value);
        field.toggle();
        assert!(!field.value);
    }

    // ListField

    #[test]
    fn list_default_selects_first() {
        assert_eq!(ListField::default().state.selected(), Some(0));
    }

    #[test]
    fn list_increment_selects_next() {
        let mut field = list(3);
        field.increment();
        assert_eq!(field.state.selected(), Some(1));
    }

    #[test]
    fn list_increment_stops_at_last_option() {
        let mut field = list(3);
        for _ in 0..10 {
            field.increment();
        }
        assert_eq!(field.state.selected(), Some(2));
    }

    #[test]
    fn list_decrement_stops_at_first_option() {
        let mut field = list(3);
        field.decrement();
        assert_eq!(field.state.selected(), Some(0));
    }

    // OptionField

    #[test]
    fn option_select_value_found_moves_selection() {
        let mut field = OptionField::new(vec!['a', 'b', 'c']);
        assert!(field.select_value('c'));
        assert_eq!(field.selected_option_index, 2);
    }

    #[test]
    fn option_select_value_missing_keeps_selection() {
        let mut field = OptionField::new(vec!['a', 'b', 'c']);
        field.selected_option_index = 1;
        assert!(!field.select_value('z'));
        assert_eq!(field.selected_option_index, 1);
    }

    #[test]
    fn option_value_returns_selected_option() {
        let mut field = OptionField::new(vec!['a', 'b', 'c']);
        field.selected_option_index = 1;
        assert_eq!(field.value(), 'b');
    }

    #[test]
    fn option_increment_stops_at_last_option() {
        let mut field = OptionField::new(vec!['a', 'b', 'c']);
        for _ in 0..10 {
            field.increment();
        }
        assert_eq!(field.selected_option_index, 2);
    }

    #[test]
    fn option_decrement_stops_at_first_option() {
        let mut field = OptionField::new(vec!['a', 'b', 'c']);
        field.decrement();
        assert_eq!(field.selected_option_index, 0);
    }

    #[test]
    fn option_increment_on_empty_does_not_panic() {
        let mut field: OptionField<char> = OptionField::new(vec![]);
        field.increment();
        assert_eq!(field.selected_option_index, 0);
    }

    // SliderField

    #[test]
    fn slider_new_starts_at_min() {
        let field = SliderField::new(3, 9, 1, "");
        assert_eq!(field.value, 3);
    }

    #[test]
    fn slider_set_value_in_range_is_kept() {
        let mut field = SliderField::new(0, 100, 1, "%");
        assert!(field.set_value(50));
        assert_eq!(field.value, 50);
    }

    #[test]
    fn slider_set_value_above_max_is_clamped() {
        let mut field = SliderField::new(0, 100, 1, "%");
        assert!(!field.set_value(150));
        assert_eq!(field.value, 100);
    }

    #[test]
    fn slider_set_value_below_min_is_clamped() {
        let mut field = SliderField::new(10, 100, 1, "%");
        assert!(!field.set_value(5));
        assert_eq!(field.value, 10);
    }

    #[test]
    fn slider_increment_clamps_at_max() {
        let mut field = SliderField::new(0, 9, 1, "");
        field.set_value(9);
        field.increment();
        assert_eq!(field.value, 9);
    }

    #[test]
    fn slider_decrement_clamps_at_min() {
        let mut field = SliderField::new(0, 9, 1, "");
        field.decrement();
        assert_eq!(field.value, 0);
    }
}
