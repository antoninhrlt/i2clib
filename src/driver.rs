// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use crate::gpio::Gpio;
use std::{
    thread, 
    time::Duration
};

pub const STANDARD_SPEED: f64   = 0.000000047;
pub const FAST_SPEED: f64       = 0.000000013;
pub const FAST_PLUS_SPEED: f64  = 0.000000005;

pub struct Driver {
    pub port: i32,
    pub address: i32,
    pub speed: f64,
    sda: Gpio,
    scl: Gpio,
}

impl Driver {
    pub fn new(port: i32, address: i32, sda: Gpio, scl: Gpio) -> Driver {
        Driver {
            port,
            address,
            speed: STANDARD_SPEED,
            sda,
            scl,
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

        self.scl.high().apply();
    }

    pub fn restart(&mut self) {
        self.scl.low().apply();
        self.wait();
        self.scl.high().apply();
    }

    pub fn stop(&mut self) {
        self.sda.low().apply();
        self.wait();
        self.sda.high().apply();
    }
}
