extern crate rppal;

use std::thread::sleep;
use std::time::Duration;

use rppal::gpio::{Gpio, Level, Mode};
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 18;

fn main() {
    println!("Hello, Pi!");

    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();
    gpio.set_mode(GPIO_LED, Mode::Output);

    gpio.write(GPIO_LED, Level::High);

    let pin = gpio.read(GPIO_LED);
    println!("level: {:?}", pin);

    loop {
        gpio.write(GPIO_LED, Level::High);
        println!("led on");
        sleep(Duration::from_millis(1000));
        gpio.write(GPIO_LED, Level::Low);
        println!("led off");
        sleep(Duration::from_millis(1000));
    }
}
