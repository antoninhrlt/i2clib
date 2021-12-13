// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::fs::File;
use std::io::prelude::*;

const GPIO_PATH: &str = "/sys/class/gpio/";

pub struct Gpio {
    id: i32,
    value: bool,
}

impl Gpio {
    pub fn new(id: i32) -> Gpio {
        let mut stream = File::create(
            GPIO_PATH.to_owned() + "export"
        ).unwrap();
        stream.write_all(id.to_string().as_bytes());

        stream = File::create(
            GPIO_PATH.to_owned() + "gpio" + id.to_string().as_str() + "/direction"
        ).unwrap();
        stream.write_all(b"out");

        Gpio {
            id,
            value: false,
        }
    }

    pub fn high(&mut self) -> &Gpio {
        self.value = true;
        self
    }

    pub fn low(&mut self) -> &Gpio {
        self.value = false;
        self
    }

    pub fn apply(&self) {
        let mut stream = File::create(
            GPIO_PATH.to_owned() + "gpio" + self.id.to_string().as_str() + "/value"
        ).unwrap();
        stream.write_all(self.value.to_string().as_bytes());
    }
}
