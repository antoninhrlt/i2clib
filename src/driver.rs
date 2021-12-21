// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::{
    thread, 
    time::Duration,
    io,
};

use crate::{
    gpio::Gpio,
};


pub const STANDARD_SPEED: f64   = 0.000000047;
pub const FAST_SPEED: f64       = 0.000000013;
pub const FAST_PLUS_SPEED: f64  = 0.000000005;

pub struct Driver {
    pub port: i32,
    pub address: i32,
    pub speed: f64,
    sda: Gpio,
    sck: Gpio,
}

impl Driver {
    pub fn new(port: i32, address: i32, sda: Gpio, sck: Gpio) -> Driver {
        Driver {
            port,
            address,
            speed: STANDARD_SPEED,
            sda,
            sck,
        }
    }

    pub fn wait(&self) {
        thread::sleep(Duration::new(self.speed as u64, 0))
    }

    pub fn change_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn start(&mut self) {
        self.sda.high().apply();
        self.wait();
        self.sda.low().apply();

        self.sck.high().apply();
    }

    pub fn restart(&mut self) {
        self.sck.low().apply();
        self.wait();
        self.sck.high().apply();
    }

    pub fn stop(&mut self) {
        self.sda.low().apply();
        self.wait();
        self.sda.high().apply();
    }

    pub unsafe fn get_c_file_descriptor(&mut self) -> Result<i32, io::Error> {
        let file_name = String::from("/dev/i2c-") + &self.port.to_string();
        
        let result: i32 = libc::open(
            file_name.as_ptr() as *const u8,
            libc::O_RDWR
        );
        
        if result < 0 {
            Err(io::Error::new(io::ErrorKind::Other, 
                format!("Error opening file {}", file_name)
            ))
        } else {
            Ok(result)
        }
    }
}
