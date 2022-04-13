// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

use std::{
    io, io::Write,
    convert::TryInto,
    os::unix::io::{
        RawFd,
        FromRawFd
    },
    fs::File,
};

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

    pub unsafe fn write(&mut self, data: i32) -> Result<usize, io::Error> {
        let fd: RawFd = self.driver.as_fd()?;
        let ioctl_result = libc::ioctl(
            fd,
            addr::I2C_SLAVE.try_into().unwrap(), 
            addr::I2C_ADDR
        );

        if ioctl_result < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "ioctl error"));
        }
        
        let mut file = File::from_raw_fd(fd);
        file.write(&[255])
    }

    // pub fn read(&mut self, data: ?????) {

    // }

    pub fn draw(&mut self, shape: Shape, fill: bool) {
        todo!()
    }
}
