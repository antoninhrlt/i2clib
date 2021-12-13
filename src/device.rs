// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use crate::{
    driver::Driver,
    shape::Shape,
};

pub struct Device {
    driver: Driver,
    width: i32,
    height: i32
}

impl Device {
    pub fn new(driver: Driver, width: i32, height: i32) -> Device {
        Device {
            driver,
            width,
            height
        }
    }

    pub fn write(&mut self, data: &mut [u8]) {

    }

    pub fn read(&mut self, data: &mut [u8]) {

    }

    pub fn draw(&mut self, shape: Shape, fill: bool) {
        todo!()
    }
}
