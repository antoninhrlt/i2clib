// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::io;

use crate::{
    addr,
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

    pub unsafe fn write(&mut self, data: i32) -> Result<i32, io::Error> {
        let fd: i32 = self.driver.get_c_file_descriptor().unwrap();

        let ioctl_result = libc::ioctl(
            fd,
            addr::I2C_SLAVE as u32, 
            addr::I2C_ADDR
        );

        if ioctl_result < 0 {
            Err(io::Error::new(io::ErrorKind::Other, "ioctl error"))
        } else {
            Ok(ioctl_result)
        }
    }

    // pub fn read(&mut self, data: ?????) {

    // }

    pub fn draw(&mut self, shape: Shape, fill: bool) {
        todo!()
    }
}
