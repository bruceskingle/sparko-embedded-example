use sparko_esp_std::{dyndns2::DynDns2, sparko_esp32_std::SparkoEsp32Std};

fn main() {


    log::info!("Hello, world!");

    // This is the app level fault barrier.
    // For the moment we just unwrap and panic, but in the future we might want to attempt some sort of recovery or restart.
    match run() {
        Ok(()) => log::info!("Application finished successfully"),
        Err(e) => {
            log::error!("Application failed with error: {}", e);
            panic!("App failed");
        },
    }
}

fn run() -> anyhow::Result<()> {
    let sparko_esp32 = SparkoEsp32Std::builder()?
        .with_feature(Box::new(DynDns2::new()?))?
        .build()?;
    
    log::info!("Trace 2");
    sparko_esp32.start()
}