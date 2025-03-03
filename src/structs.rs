use once_cell::sync::Lazy;
use rune::{Any, ContextError, Module, Value};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Any, PartialEq)]
pub struct Data {
    #[rune(get)]
    pub delta: u128,

    #[rune(get)]
    pub busy_delta: u128,

    #[rune(get)]
    pub rune_delta: u128,

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
            fps: 0,
            target_fps: 24,
            clear_color_r: 0,
            clear_color_g: 0,
            clear_color_b: 0,
            exit: false,
        }
    }
}

type GlobalsType = HashMap<u64, Value>;

static GLOBALS_PTR: Lazy<u64> = Lazy::new(|| {
    let globals: GlobalsType = HashMap::new();
    let ptr = Box::into_raw(Box::new(globals));
    ptr as u64
});

#[rune::function]
fn create_global(key: u64, value: Value) {
    let globals = unsafe { &mut *(*GLOBALS_PTR as *mut GlobalsType) };
    globals.insert(key, value);
}

#[rune::function]
fn get_global(key: u64) -> Value {
    let globals = unsafe { &mut *(*GLOBALS_PTR as *mut GlobalsType) };
    match globals.get(&key) {
        Some(value) => value.clone(),
        None => {
            eprintln!("Global not found: {}", key);
            panic!();
        }
    }
}

#[rune::function]
fn set_global(key: u64, value: Value) {
    let globals = unsafe { &mut *(*GLOBALS_PTR as *mut GlobalsType) };
    match globals.get_mut(&key) {
        Some(v) => *v = value,
        None => {
            eprintln!("Global not found: {}", key);
            panic!();
        }
    }
}

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();
    module.ty::<Data>()?;
    module.function_meta(create_global)?;
    module.function_meta(get_global)?;
    module.function_meta(set_global)?;
    Ok(module)
}
