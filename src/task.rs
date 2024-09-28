pub struct Task {
    description: String,
    done: bool,
}

impl Task {
    pub fn new(description: String, done: bool) -> Self {
        return Self { description, done };
    }

    pub fn get_description(&self) -> &str {
        return self.description.as_str();
    }
}
