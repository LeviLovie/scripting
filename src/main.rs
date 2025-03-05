mod globals;
mod structs;

use rand::Rng;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, ContextError, Diagnostics, Module, Source, Sources, Vm};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

fn main_script_path() -> String {
    let exe_dir = std::env::current_exe().unwrap();
    let main_file = exe_dir.parent().unwrap().join("scripts").join("main.rn");
    return main_file.to_str().unwrap().to_string();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let init_instant = Instant::now();

    // VM
    let mut context = Context::with_default_modules()?;
    context.install(structs::module()?)?;
    context.install(globals::module()?)?;
    context.install(module()?)?;
    let runtime = Arc::new(context.runtime()?);

    // Load source code
    let mut sources = Sources::new();
    sources.insert(Source::from_path(main_script_path())?)?;

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
    let mut data = structs::Data::new();
    let mut globals = globals::Globals::new();
    vm.call(["start"], (&mut data, &mut globals))?;
    println!("Initialization time: {}us", init_elapsed.as_micros());

    let mut rune_deltas = vec![];
    let mut present_instant = Instant::now();
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

        canvas.set_draw_color(Color::RGB(
            data.clear_color_r as u8,
            data.clear_color_g as u8,
            data.clear_color_b as u8,
        ));
        canvas.clear();
        canvas.present();

        {
            let elapsed = present_instant.elapsed();
            data.busy_delta = elapsed.as_micros();
            let max_frame_time = 1_000_000 / data.target_fps;
            let sleep_time: i64 = max_frame_time as i64 - data.busy_delta as i64;
            if sleep_time > 0 {
                std::thread::sleep(Duration::from_micros(sleep_time as u64));
            }
            let full_elapsed = present_instant.elapsed();
            data.delta = full_elapsed.as_micros();
            data.fps = (1_000_000 / data.delta) as u32;
        }
        present_instant = Instant::now();

        let update_instant = Instant::now();
        vm.call(["update"], (&mut data, &mut globals))?;
        rune_deltas.push(update_instant.elapsed().as_micros());
        data.last_rune = rune_deltas[rune_deltas.len() - 1];
        if rune_deltas.len() > 100 {
            rune_deltas.remove(0);
        }
        data.rune_delta = rune_deltas.iter().sum::<u128>() / rune_deltas.len() as u128;
        if data.exit {
            break 'running;
        }
    }

    Ok(())
}

#[rune::function]
fn random_range_int(min: i64, max: i64) -> i64 {
    rand::rng().random_range(min..max)
}

fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();
    module.function_meta(random_range_int)?;
    Ok(module)
}
