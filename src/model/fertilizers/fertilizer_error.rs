#[derive(Debug, Clone, PartialEq)]
pub struct FertilizerError {
    name: Option<String>,
}

impl FertilizerError {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub fn set_name(&mut self, error: impl Into<String>) {
        self.name = Some(error.into());
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none()
    }
}
