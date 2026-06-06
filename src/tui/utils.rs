// Add lifetimes

pub struct Action {
    pub key: &'static str,
    pub action: &'static str,
}

impl Action {
    pub fn new(key: &'static str, action: &'static str) -> Self {
        Action {
            key: key,
            action: action,
        }
    }

    pub fn to_str(&self) -> String {
        return format!("[{}] {}", self.key, self.action);
    }
}
