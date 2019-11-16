#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::result::TockResult;
use libtock::sensors::*;
use libtock::timer;
use libtock::timer::Duration;

#[libtock::main]
async fn main() -> TockResult<()> {
    let mut console = Console::new();
    let mut humidity = HumiditySensor;
    let mut temperature = TemperatureSensor;
    let mut light = AmbientLightSensor;
    let mut ninedof = unsafe { Ninedof::new() };
    let context = timer::DriverContext::create()?;
    let mut driver = context.create_timer_driver().unwrap();
    let timer_driver = driver.activate()?;

    loop {
        writeln!(console, "Humidity:    {}\n", humidity.read()?)?;
        writeln!(console, "Temperature: {}\n", temperature.read()?)?;
        writeln!(console, "Light:       {}\n", light.read()?)?;
        writeln!(console, "Accel:       {}\n", ninedof.read_acceleration()?)?;
        timer_driver.parallel_sleep(Duration::from_ms(500)).await?;
    }
}
