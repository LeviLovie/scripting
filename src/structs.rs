use rune::{Any, ContextError, Module};

#[derive(Default, Debug, Clone, Any, PartialEq, Eq)]
pub struct Data {
    #[rune(get)]
    pub delta: u128,

    #[rune(get)]
    pub fps: u32,

    #[rune(get, set)]
    pub target_fps: u32,

    #[rune(get, set)]
    pub window_size: (u32, u32),

    #[rune(set)]
    pub exit: bool,
}

impl Data {
    pub fn new() -> Self {
        Self {
            delta: 0,
            fps: 0,
            target_fps: 24,
            window_size: (800, 600),
            exit: false,
        }
    }
}

#[rune::function]
fn new_data() -> Data {
    Data::new()
}

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();
    module.ty::<Data>()?;
    module.function_meta(new_data)?;
    Ok(module)
}
