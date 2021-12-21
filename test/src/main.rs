// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use i2clib::{
    addr::OLED_ADDRESS,
    device::Device,
    driver::Driver,
    gpio::Gpio,
};

fn main() {
    let mut test_led = Gpio::new(16);
    test_led.high().apply();

    let sda = Gpio::new(3);
    let sck = Gpio::new(5);
    
    let driver = Driver::new(1, 0x3c, sda, sck);
    
    let mut device = Device::new(driver, 128, 64);
    unsafe {
        device.write(255).unwrap();
    }

    /*
    let square = Shape::new(0, 0, 10, 10, Color::White);
    device.draw(square, true /* fill */);
    */

    test_led.low().apply();
}
