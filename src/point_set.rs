//-------------------------------------------------------------------------
// @file point_set.rs
//
// @date 06/15/21 20:54:34
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
//
// @detail
//
//  Licence:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
//--------------------------------------------------------------------------
use crate::point::Point;
// use num::{Num, Float, Zero};
use heapless::Vec; // fixed capacity `std::Vec`

const N: usize = 100;

pub struct PointSet<T> {
    pub points: Vec<Point<T>, N>,
    is_visible: bool
}

impl<T> PointSet<T> {
    pub fn new() -> Self {
        let points: Vec<Point<T>, N> = Vec::new();
        Self{points, is_visible: false}
    }
}

