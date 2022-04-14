// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

pub type Addr = i32; 

/// Considering being on a 7 bits base
pub const OLED_ADDRESS: Addr = 0x3c;         // = 0x78 on a 8 bits base
pub const OLED_OTHER_ADDRESS: Addr = 0x3D;   // = 0x71 on a 8 bits base

pub const I2C_SLAVE: Addr = 0x0703;
pub const I2C_RDWR : Addr = 0x0707;
pub const I2C_SMBUS: Addr = 0x0720;
pub const I2C_ADDR : Addr = 0x20;
