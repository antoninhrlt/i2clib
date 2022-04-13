// This file is part of i2clib
// Copyright (c) 2021 Antonin Hérault
// Under the MIT License

pub struct Shape {
    pub x: i32,
    pub y: i32,

    pub width: i32,
    pub height: i32,
}

impl Shape {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Shape {
        Shape {
            x,
            y,
            width,
            height,
        }
    }
}
