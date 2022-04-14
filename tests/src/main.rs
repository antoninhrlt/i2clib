// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use i2clib::{
    device::Device,
    driver::Driver,
    gpio::Gpio,
};

fn main() {
    let sda = Gpio::new(3);
    let sck = Gpio::new(5);
}
