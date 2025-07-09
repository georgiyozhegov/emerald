pub struct Reporter {
    errors: Vec<Box<dyn Report>>,
}

// initialization
impl Reporter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

// logic
impl Reporter {
    pub fn report<E>(&mut self, error: E)
    where
        E: Report + 'static,
    {
        let boxed = Box::new(error);
        self.errors.push(boxed);
    }
}

pub trait Report {
    fn module(&self) -> String;
    fn message(&self) -> String;
    fn location(&self) -> String;
}
