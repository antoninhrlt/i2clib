// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

const GPIO_PATH: &str = "/sys/class/gpio/";

pub struct Gpio {
    id: i32,
    value: bool,
}

impl Gpio {
    pub fn new(id: i32) -> Gpio {
        let gpio_path = GPIO_PATH.to_owned() + "gpio" + id.to_string().as_str();

        if !Path::new(&gpio_path).exists() {
            File::create(GPIO_PATH.to_owned() + "export").unwrap()
                .write_all(id.to_string().as_bytes()).unwrap();
        }

        File::create(gpio_path + "/direction")
            
            .unwrap()
            .write_all(b"out").unwrap();

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
        File::create(GPIO_PATH.to_owned() + "gpio" 
            + self.id.to_string().as_str() + "/value").unwrap()

            .write_all(format!("{}", self.value as i32).as_bytes())
            .unwrap();
    }
}
