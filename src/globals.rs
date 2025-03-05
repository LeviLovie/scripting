use rune::{Any, ContextError, Module, Value};

#[derive(Default, Debug, Clone, Any)]
pub struct Globals {
    globals: Vec<Value>,
}

impl Globals {
    pub fn new() -> Self {
        Self {
            globals: Vec::new(),
        }
    }

    #[rune::function]
    pub fn set(&mut self, key: u64, value: Value) {
        match self.globals.get_mut(key as usize) {
            Some(global) => *global = value,
            None => self.globals.push(value),
        }
    }

    #[rune::function]
    pub fn get(&self, key: u64) -> Value {
        self.globals.get(key as usize).cloned().unwrap_or_default()
    }
}

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();
    module.ty::<Globals>()?;
    module.function_meta(Globals::set)?;
    module.function_meta(Globals::get)?;
    Ok(module)
}
