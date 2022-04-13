// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::{
    thread, 
    time::Duration,
    io,
    fs::File,
    os::unix::io::{
        RawFd,
        IntoRawFd,
    }
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
    sda: Gpio, // serial data line
    sck: Gpio, // serial clock (lines), or named "scl"
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

    pub fn as_fd(&mut self) -> Result<RawFd, io::Error> {
        let filename = format!("/dev/i2c-{}", &self.port);
        let file = File::create(filename); // read-write mode
        
        match file {
            Ok(file) => Ok(file.into_raw_fd().clone()),
            Err(e) => Err(e),
        }
    }
}
