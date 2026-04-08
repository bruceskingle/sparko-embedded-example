# sparko-embedded-example
Example Applications for Sparko Embedded

## Introduction
The idea of Sparko Embedded is to provide a platform for embedded applications for hardware such as ESP32 SoC boards. Sparko Embedded Std is a version of this platform which includes the standard Rust library which means that the heap and standard collections like ```Vec``` are all available for use. This crate contains example applications for that platform targetting all supported boards.

The following GitHub repos contain source code for the various platform crates:

- https://github.com/bruceskingle/sparko-esp-std
- https://github.com/bruceskingle/sparko-embedded-std

## Example Application
Here is the entire code for an example application:
```rust
use sparko_esp_std::{dyndns2::DynDns2, sparko_esp32_std::SparkoEsp32Std};

fn main() {
    log::info!("Hello, world!");

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

    sparko_esp32.start()
}
```
Additional functinality can be added by implementingmore Features.