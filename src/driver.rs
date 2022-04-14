// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::fs::File;
use std::io;
use std::os::unix::io::{RawFd, IntoRawFd};
use std::thread;
use std::time::Duration;

use crate::{
    addr::Addr,
    gpio::Gpio,
    speed::Speed,
};

/// And how much max_of_masters ? It's unlimited.
pub const MAX_OF_SLAVES: u64 = 1008;

/// Way to manage the screen device from GPIOs `sda` and `sck` \ 
pub struct Driver {
    port: i32,
    address: Addr,
    speed: Speed,

    /// Serial data line
    sda: Gpio,
    /// Serial clock (lines), also known as `scl`
    sck: Gpio,
}

impl Driver {
    pub fn new(port: i32, address: Addr, speed: Speed, sda: Gpio, sck: Gpio) -> Driver {
        Driver {
            port,
            address,
            speed,
            sda,
            sck,
        }
    }

    pub fn start(&mut self) {
        todo!()
    }

    pub fn restart(&mut self) {
        todo!()
    }

    pub fn stop(&mut self) {
        todo!()
    }

    /// Time needed between to operations
    pub fn wait(&self) {
        thread::sleep(Duration::new(self.speed, 0))
    }

    pub fn change_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Get the driver as Unix file descriptor
    pub fn as_fd(&mut self) -> Result<RawFd, io::Error> {
        let file = File::create(format!("/dev/i2c-{}", &self.port))?;
        Ok(file.into_raw_fd().clone())
    }

    // Getters

    pub fn port(&self) -> &i32 {
        &self.port
    }

    pub fn address(&self) -> &Addr {
        &self.address
    }

    pub fn speed(&self) -> &Speed {
        &self.speed
    }
}
