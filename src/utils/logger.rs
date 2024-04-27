use std::{panic};
use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn};

pub fn setup_logger() {
    // create logger
    Builder::new()
        .filter(None, log::LevelFilter::Info) // Filters all modules to log up to Info level
        .init();

    // bind logger to panic
    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        // Log the panic message
        error!("Panic occurred: {:?}", info);
        // Optionally keep the default panic behavior
        default_panic(info);
    }));
}