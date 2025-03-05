use rune::{Any, ContextError, Module};

#[derive(Default, Debug, Clone, Any, PartialEq)]
pub struct Data {
    #[rune(get)]
    pub delta: u128,

    #[rune(get)]
    pub busy_delta: u128,

    #[rune(get)]
    pub rune_delta: u128,

    #[rune(get)]
    pub last_rune: u128,

    #[rune(get)]
    pub fps: u32,

    #[rune(get, set)]
    pub target_fps: u32,

    #[rune(get, set)]
    pub clear_color_r: i64,

    #[rune(get, set)]
    pub clear_color_g: i64,

    #[rune(get, set)]
    pub clear_color_b: i64,

    #[rune(set)]
    pub exit: bool,
}

impl Data {
    pub fn new() -> Self {
        Self {
            delta: 0,
            busy_delta: 0,
            rune_delta: 0,
            last_rune: 0,
            fps: 0,
            target_fps: 24,
            clear_color_r: 0,
            clear_color_g: 0,
            clear_color_b: 0,
            exit: false,
        }
    }

    #[rune::function]
    pub fn set_clear_color(&mut self, r: i64, g: i64, b: i64) {
        self.clear_color_r = r;
        self.clear_color_g = g;
        self.clear_color_b = b;
    }
}

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();
    module.ty::<Data>()?;
    module.function_meta(Data::set_clear_color)?;
    Ok(module)
}
