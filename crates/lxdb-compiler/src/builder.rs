/// Builder used to configure the compiler.
#[derive(Debug, Default)]
pub struct Builder {
    language: Option<String>,
    dataset_name: Option<String>,
    input: Option<String>,
    output: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn dataset_name(mut self, name: impl Into<String>) -> Self {
        self.dataset_name = Some(name.into());
        self
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(input.into());
        self
    }

    pub fn output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }

    pub fn build(self) -> Self {
        self
    }

    // Getters
    pub fn input_path(&self) -> Option<&str> {
        self.input.as_deref()
    }

    pub fn output_path(&self) -> Option<&str> {
        self.output.as_deref()
    }
}
