pub struct Action {
    key: String,
    action: String,
}

impl Action {
    pub fn new(key: String, action: String) -> Self {
        Action {
            key: key,
            action: action,
        }
    }
}
