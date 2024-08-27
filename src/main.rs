#![no_std]
#![no_main]

use core::{
    fmt::Write,
    sync::atomic::{AtomicU32, Ordering},
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks},
    gpio::{AnyInput, AnyOutput, Gpio4, Gpio5, Io, Level, Pull},
    i2c::I2C,
    peripherals::{Peripherals, I2C0},
    prelude::*,
    system::SystemControl,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
};

use ssd1306::{prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306};
use static_cell::StaticCell;

static DISTANCE: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn calculate_distance(mut trig: AnyOutput<'static>, mut echo: AnyInput<'static>) {
    loop {
        trig.set_low();
        Timer::after(Duration::from_micros(5)).await;
        trig.set_high();
        Timer::after(Duration::from_micros(10)).await;
        trig.set_low();
        echo.wait_for_high().await;
        let start_time = Instant::now().as_micros();
        echo.wait_for_low().await;
        let end_time = Instant::now().as_micros();
        let echo_duration = end_time - start_time;
        let distance_cm = (echo_duration / 58) as u32;
        DISTANCE.store(distance_cm, Ordering::Relaxed);
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn display_distance(i2c_per: I2C0, sda: Gpio4, scl: Gpio5, clocks: Clocks<'static>) {
    let i2c = I2C::new(i2c_per, sda, scl, 400.kHz(), &clocks, None);

    /* Display configuration */
    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(i2c),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_terminal_mode();

    display.clear().unwrap();
    display.init().unwrap();

    let mut distance = DISTANCE.load(Ordering::Relaxed);
    write!(display, "Distance = {}", distance).unwrap();
    loop {
        let distance_new = DISTANCE.load(Ordering::Relaxed);
        if distance_new != distance {
            //do i really need it?
            display.clear().unwrap();
            write!(display, "\nDistance = {}", distance_new).unwrap();
            distance = distance_new;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    /* esp init */
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_control).freeze();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    esp_println::logger::init_logger_from_env();

    /*embassy init */
    let timer0 = OneShotTimer::new(
        TimerGroup::new(peripherals.TIMG0, &clocks, None)
            .timer0
            .into(),
    );
    static ONESHOTTIMER: StaticCell<[OneShotTimer<ErasedTimer>; 1]> = StaticCell::new();
    let timer = ONESHOTTIMER.uninit().write([timer0]);
    esp_hal_embassy::init(&clocks, timer);

    spawner
        .spawn(calculate_distance(
            AnyOutput::new(io.pins.gpio6, Level::Low),
            AnyInput::new(io.pins.gpio7, Pull::None),
        ))
        .unwrap();
    spawner
        .spawn(display_distance(
            peripherals.I2C0,
            io.pins.gpio4,
            io.pins.gpio5,
            clocks,
        ))
        .unwrap();
}
