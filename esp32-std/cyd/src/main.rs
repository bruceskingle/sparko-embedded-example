use std::time::Duration;

use chrono::Local;
use chrono::Timelike;
use esp_idf_sys::esp_random;
use log::info;
use smart_leds::{RGB8, SmartLedsWrite, hsv::{Hsv, hsv2rgb}};
use sparko_esp_std::binary_clock_feature::BinaryClockFeature;
use sparko_esp_std::{analog_clock_feature::AnalogClock, dyndns2::DynDns2, sparko_esp32_std::SparkoEsp32Std};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

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

// fn to_bits(pixels: &mut [RGB8], index: usize, bits: usize, off: RGB8, on: RGB8, v: u32) {

//     for (i, pixel) in pixels.iter_mut().enumerate() {
//         if i == h1 || i == h2 {
//             *pixel = Rgb8::new(255, 0, 0);
//         }
//         else if i == m1 || i == m2 {
//             *pixel = Rgb8::new(0, 255, 0);
//         }
//         else {
//             *pixel = Rgb8::new(0, 0, 255);
//         }
//     }
// }
fn to_bits(
    pixels: &mut [RGB8],
    index: usize,
    bits: usize,
    off: RGB8,
    on: RGB8,
    v: u32,
) {
    assert!(index + bits <= pixels.len());

    for i in 0..bits {
        // Extract bit from most-significant to least-significant
        let bit = (v >> (bits - 1 - i)) & 1;

        pixels[index + i] = if bit == 0 { off } else { on };
    }
}

fn run() -> anyhow::Result<()> {
    let (builder, remainder) = SparkoEsp32Std::builder()?;

    // let mut ws2812 = Ws2812Esp32Rmt::new(remainder.rmt.channel0, remainder.gpio22).unwrap();

    // let mut pixels = [RGB8::default(); 20];
    // let off = RGB8::new(0, 0, 0);
    // let h1 = RGB8::new(255, 0, 0);
    // let h2 = RGB8::new(255, 100, 100);
    // let m1 = RGB8::new(0, 255, 0);
    // let m2 = RGB8::new(100, 255, 100);
    // let s1 = RGB8::new(0, 0, 255);
    // let s2 = RGB8::new(100, 100, 255);

    // loop {
    //     let now = Local::now();

    //     info!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S"));
    //     to_bits(&mut pixels, 0, 2, off, h1, now.hour() / 10);
    //     to_bits(&mut pixels, 2, 4, off, h2, now.hour() % 10);

    //     to_bits(&mut pixels, 6, 3, off, m1, now.minute() / 10);
    //     to_bits(&mut pixels, 9, 4, off, m2, now.minute() % 10);

    //     to_bits(&mut pixels, 13, 3, off, s1, now.second() / 10);
    //     to_bits(&mut pixels, 16, 4, off, s2, now.second() % 10);

    //     ws2812.write(pixels).unwrap();

    //     std::thread::sleep(Duration::from_millis(1000));
    // }
    
    // println!("Start NeoPixel rainbow!");

    // let mut hue = unsafe { esp_random() } as u8;
    // loop {
    //     let pixels = std::iter::repeat(hsv2rgb(Hsv {
    //         hue,
    //         sat: 255,
    //         val: 8,
    //     }))
    //     .take(25);
    //     ws2812.write(pixels).unwrap();

    //     std::thread::sleep(Duration::from_millis(100));

    //     hue = hue.wrapping_add(10);
    // }


    let mut sparko_esp32 = builder
        .with_feature(Box::new(DynDns2::new()?))?
        .with_feature(Box::new(AnalogClock::builder()
            // .with_layout(|rect| {
            //     let margin = 3;
            //     let size = std::cmp::min(rect.size.width, rect.size.height) - 2 * margin as u32;
            //     Rectangle {
            //         top_left: Point { x: rect.top_left.x + margin, y: rect.top_left.y + margin },
            //         size: Size { width: size, height: size },
            //     }
            // })
            .build()?))?
        .with_feature(Box::new(BinaryClockFeature::new(remainder.rmt.channel0, remainder.gpio22)))?
        .with_display_orientation(sparko_embedded_std::DisplayOrientation::Rotate0)?
        .build()?;

    // let cloned_ap_mode = sparko_cyd.ap_mode.clone();
    // sparko_cyd.server_manager.fn_handler("/", Method::Get, move |req| {

    //         // info!("Received request for / from {}", req.connection().remote_addr());

    //         info!("Received {:?} request for {}", req.method(), req.uri());

    //         if cloned_ap_mode.lock().unwrap().clone() {
    //             let mut resp = req.into_response(
    //                 302,
    //                 Some("Found"),
    //                 &[("Location", "/config")],
    //             )?;
    //         }
    //         else {

    //             let mut resp = req.into_ok_response()?;
    //             resp.write(r#"
    //                 <!DOCTYPE html>
    //                 <html lang="en">
    //                 <head>
    //                     <meta charset="utf-8" />
    //                     <meta name="viewport" content="width=device-width, initial-scale=1" />
    //                     <title>ESP32 Home</title>
    //                     <link rel="stylesheet" href="/main.css">
    //                 </head>
    //                 <body>
    //                     <div class="page">
    //                         <h1>ESP32 Home</h1>
    //                         <p>Welcome to the ESP32 home page!</p>
    //                         <p>Current time: "#.as_bytes())?;

    //             let now = Local::now();
    //             let time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    //             resp.write(time.as_bytes())?;
    //             resp.write(r#"</p>
    //                     </div>
    //                 </body>
    //                 </html>
    //                 "#.as_bytes())?;
    //         }
    //         Ok(())
    //     })?;

    
    log::info!("Trace 2");
    sparko_esp32.start()
    // ?;
    // sparko_cyd.run()
}


    // log::info!("Trace 3");
    // let current_dns = resolve_local_dns()?;
    // info!("Current DNS resolution for home.skingle.org: {}", current_dns);

    // let addr = Arc::new(Mutex::new(current_dns));

    // // let handler_addr = addr.clone();

    // let mut cnt = 0;

    // let mut r = 64;
    // let mut g = 0;
    // let mut b = 0;
    // loop {
    //     log::info!("Top of loop");

    //     // sparko_cyd.led_manager.set_color(r,g,b)?;

    //     // let c = r;
    //     // r = b;
    //     // b = g;
    //     // g = c;

    //     if cnt < 3 {
    //         match get_public_ip_address() {
    //             Ok(public_ip) => {
    //                 cnt = cnt + 1;
    //                 if public_ip != *addr.clone().lock().unwrap() {
    //                     log::info!("Public IP changed: {} -> {}", *addr.lock().unwrap(), public_ip);
    //                     // *addr.lock()? = public_ip;
    //                 } else {
    //                     log::info!("Public IP unchanged: {}", public_ip);
    //                 }
    //             },
    //             Err(e) => {
    //                 log::error!("Failed to get public IP address: {}", e);
    //             }
    //         }
    //     }

        

    //     // let mut led = led.lock()?;
    //     // led.toggle()?;
    //     std::thread::sleep(std::time::Duration::from_secs(10));
    // }


