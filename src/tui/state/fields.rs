use ratatui::widgets::ListState;

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
        self.state.select_next();
    }

    fn decrement(&mut self) {
        self.state.select_previous();
    }
}
