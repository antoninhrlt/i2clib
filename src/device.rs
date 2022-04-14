// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::convert::TryInto;
use std::io;
use std::io::Write;
use std::os::unix::io::{RawFd, IntoRawFd};

use crate::{
    addr,
    driver::Driver,
    shape::Shape,
};

/// Manage a screen from its adapted driver
pub struct Device {
    driver: Driver,
    width: u32,
    height: u32,
}

impl Device {
    pub fn new(driver: Driver, width: u32, height: u32) -> Device {
        Device {
            driver,
            width,
            height,
        }
    }

    pub fn write(&mut self, data: u32) {
        todo!()
    }

    pub fn read(&mut self) -> u32 {
        todo!()
    }

    pub fn draw(&mut self, shape: Shape, fill: bool) {
        todo!()
    }
}
