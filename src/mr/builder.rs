use mr;

pub struct Builder<'a> {
    module: Option<mr::Module<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder { module: None }
    }

    pub fn initialize(&mut self, header: mr::ModuleHeader) {
        let mut module = mr::Module::new();
        module.header = Some(header);
        self.module = Some(module);
    }

    pub fn finalize(&mut self) -> Option<mr::Module<'a>> {
        self.module.take()
    }
}
