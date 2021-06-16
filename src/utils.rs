//-------------------------------------------------------------------------
// @file utils.rs
//
// @date 06/15/21 19:47:54
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
use num::Float;
/// a comparison function for floating point values
///
pub fn nearly_equal<T: Float>(a: T, b: T, epsilon: T) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let abs_diff = (a - b).abs();
    let zero = T::zero();
    // short-cut, handles infinity
    if a == b { true }

    else if a == zero || b == zero || (abs_a + abs_b < T::min_value()) {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        abs_diff < epsilon
    } else {
      abs_diff / T::min(abs_a + abs_b, T::max_value()) < epsilon
    }
}

pub fn nearly_zero<T: Float>(a: T) -> bool {
    nearly_equal(a, T::zero(), T::epsilon())
}

