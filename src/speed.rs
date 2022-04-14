// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

/// Values are in kbps (kilo bits per second) \
/// Import should look like : 
/// ```rust
/// use i2clib::speed;
/// ```
/// To use them with a prefixed identifier for "speed" \
/// Values retrieved from here : https://www.circuitbasics.com/wp-content/uploads/2016/02/Basics-of-the-I2C-Communication-Protocol-Specifications-Table.png

pub type Speed = u64;

pub const STANDARD_MODE: Speed = 100;
pub const FAST_MODE: Speed = 400;
pub const HIGH_SPEED_MODe: Speed = 3400;
pub const ULTRA_FAST_MODE: Speed = 5000;
