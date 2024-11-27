#![no_std]
#![no_main]


/*
* Wiring is all fine but the connections on the pico side are flimsy
* and need to be soldered to get any reliability
* Currently the temp+humidity sensing is working
* and 'hello world' is being written to the OLED
* Still todo
* - write the sensor values out to the OLED
* - Use the wifi network to make data available in home assistant
* - have a button to spin up a webserver to configure wifi creds
* - Figure out what to do for power
* - Figure out how to tie it all together off the breadboard
*/


use core::fmt::Write;
use embassy_dht::dht22::DHT22;
use embassy_executor::Spawner;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, i2c};
use embassy_time::{Delay, Timer};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text},
};
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{prelude::*, Ssd1306};
use u8g2_fonts::fonts::u8g2_font_wqy12_t_gb2312;
use u8g2_fonts::U8g2TextStyle;

pub mod fmtbuf;
use fmtbuf::FmtBuf;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    Timer::after_secs(3).await;
    log::info!("Starting");

    log::info!("Setting up i2c ");
    let sda = p.PIN_8;
    let scl = p.PIN_9;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, i2c::Config::default());

    // Create the I²C display interface:
    let interface = ssd1306::I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    match display.init() {
        Ok(()) => log::info!("Display Initialised"),
        Err(e) => log::error!("Display Initialise Failed: {e:?}"),
    }

    // Create a text style for drawing the font:
    let character_style = U8g2TextStyle::new(u8g2_font_wqy12_t_gb2312, BinaryColor::On);
    let mut line1 = FmtBuf::new();
    let mut line2 = FmtBuf::new();
    let mut line0_p2 = FmtBuf::new();

    let mut dht_pin = DHT22::new(p.PIN_15, Delay);

    loop {
        Timer::after_secs(1).await;
        match display.clear(BinaryColor::Off) {
            Ok(()) => log::info!("screen on"),
            Err(e) => log::error!("Read Failed: {e:?}"),
        }
        line1.reset();
        line2.reset();

        match dht_pin.read() {
            Ok(reading) => {
                let (temp, humi) = (reading.get_temp(), reading.get_hum());
                log::info!("Temp = {:.2}℃ , Humi = {:.2}%", temp, humi);
                write!(&mut line1, "Temp: {temp:.2}℃ ").unwrap();
                write!(&mut line2, "Humi: {humi:.2}%").unwrap();
            }
            Err(e) => {
                log::error!("Read Failed: {e}");
                write!(&mut line1, "Temp: Error").unwrap();
                write!(&mut line1, "Humi: Error").unwrap();
            }
        };

        match Text::with_baseline(
            line1.as_str(),
            Point::new(3, 2),
            character_style.clone(),
            Baseline::Top,
        )
        .draw(&mut display) {
            Ok(_point) => log::info!("draw"),
            Err(e) => log::error!("Draw Failed: {e:?}"),
        }

        match Text::with_baseline(
            line2.as_str(),
            Point::new(3, 38),
            character_style.clone(),
            Baseline::Top,
        )
        .draw(&mut display) {
            Ok(_point) => log::info!("draw"),
            Err(e) => log::error!("Draw Failed: {e:?}"),
        }

        //Text::with_baseline(
        //    line0_p2.as_str(),
        //    Point::new(74, 2),
        //    character_style.clone(),
        //    Baseline::Top,
        //)
        //.draw(&mut display)
        //.unwrap();
        //
        //write!(&mut line1, "温度： {}℃", 30.0).unwrap(); // ℃ ,°C
        //Text::with_baseline(
        //    line1.as_str(),
        //    Point::new(32, 22),
        //    character_style.clone(),
        //    Baseline::Top,
        //)
        //.draw(&mut display)
        //.unwrap();
        //
        //write!(&mut line2, "湿度： {}%", 60.0).unwrap();
        //Text::with_baseline(
        //    line2.as_str(),
        //    Point::new(32, 38),
        //    character_style.clone(),
        //    Baseline::Top,
        //)
        //.draw(&mut display)
        //.unwrap();
        //
        //Line::new(Point::new(0, 0), Point::new(127, 0))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //
        //Line::new(Point::new(0, 0), Point::new(0, 63))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //
        //Line::new(Point::new(0, 63), Point::new(127, 63))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //
        //Line::new(Point::new(127, 0), Point::new(127, 63))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //
        //Line::new(Point::new(70, 0), Point::new(70, 16))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //
        //Line::new(Point::new(0, 16), Point::new(127, 16))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //
        //Line::new(Point::new(0, 15), Point::new(127, 15))
        //    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        //    .draw(&mut display)
        //    .unwrap();
        //

        match display.flush() {
            Ok(()) => log::info!("flush"),
            Err(e) => log::error!("flush Failed: {e:?}"),
        }
    }
}
