mod structs;

use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, ContextError, Diagnostics, Module, Source, Sources, Vm};
use std::{sync::Arc, time::Instant};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let init_instant = Instant::now();

    // VM
    let mut context = Context::with_default_modules()?;
    let m = module()?;
    context.install(m)?;
    let structs_module = structs::module()?;
    context.install(structs_module)?;
    let runtime = Arc::new(context.runtime()?);

    // Load source code
    let mut sources = Sources::new();
    sources.insert(Source::new("main", include_str!("./script.rn"))?)?;

    let mut diagnostics = Diagnostics::new();

    // Compile
    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    // Emit diagnostics
    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    // Check for errors
    let unit = result?;
    let mut vm = Vm::new(runtime, Arc::new(unit));

    let init_elapsed = init_instant.elapsed();
    let run_instant = Instant::now();

    // Call the start function
    let output = vm.call(["start"], ())?;
    let mut data: structs::Data = rune::from_value(output)?;
    println!("Start output: {:?}", data.clone());

    loop {
        // Call the update function
        let output = vm.call(["update"], (data,))?;
        data = rune::from_value(output)?;
        println!("Update output: {:?}", data.clone());
        if data.exit {
            break;
        }
    }

    let run_elapsed = run_instant.elapsed();

    println!("Initialization time: {}us", init_elapsed.as_micros());
    println!("Run time (1000 executions): {}us", run_elapsed.as_micros());

    Ok(())
}

#[rune::function]
fn atan2(a: f64, b: f64) -> f64 {
    return a.atan2(b);
}

fn module() -> Result<Module, ContextError> {
    let mut m = Module::with_item(["native"])?;
    m.function_meta(atan2)?;
    Ok(m)
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
    }
}
