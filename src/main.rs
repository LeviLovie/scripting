mod structs;

use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, ContextError, Diagnostics, Module, Source, Sources, Value, Vm};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

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

    // Call the start function
    let output = vm.call(["start"], ())?;
    let mut data: structs::Data;
    let mut local_data: Value;
    (data, local_data) = rune::from_value(output)?;
    println!("Start output: ({:?}, {:?})", data, local_data);
    println!("Initialization time: {}us", init_elapsed.as_micros());

    let mut update_times = vec![];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / data.target_fps));

        let update_instant = Instant::now();
        let output = vm.call(["update"], (data, local_data))?;
        (data, local_data) = rune::from_value(output)?;
        let update_elapsed = update_instant.elapsed();
        update_times.push(update_elapsed);
        if update_times.len() > 100 {
            update_times.remove(0);
        }
        println!(
            "Update output (.1): {:?}. In {}us ({}us)",
            local_data,
            update_elapsed.as_micros(),
            update_times.iter().map(|x| x.as_micros()).sum::<u128>() / update_times.len() as u128
        );
        if data.exit {
            break 'running;
        }

        // Adjust the window size
        canvas
            .window_mut()
            .set_size(data.window_size.0, data.window_size.1)?;
    }

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
