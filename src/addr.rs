// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

/// Considering being on a 7 bits base
pub const OLED_ADDRESS: i32 = 0x3c;         // = 0x78 on a 8 bits base
pub const OLED_OTHER_ADDRESS: i32 = 0x3D;   // = 0x71 on a 8 bits base

pub const I2C_SLAVE: i32 = 0x0703;
pub const I2C_RDWR : i32 = 0x0707;
pub const I2C_SMBUS: i32 = 0x0720;