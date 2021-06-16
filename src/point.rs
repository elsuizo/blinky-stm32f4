//-------------------------------------------------------------------------
// @file point.rs
//
// @date 06/15/21 19:36:25
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
// TODO(elsuizo:2021-06-15): i dont known if this will compile for no-std (Num)
use num::{Float, Num, Signed, Zero};
use crate::utils::nearly_zero;
use core::ops::{Add, Sub, Div, Mul, SubAssign, AddAssign, Neg};

#[derive(Debug, Copy, Clone,PartialEq)]
pub struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}

//-------------------------------------------------------------------------
//                        arithmetic operations
//-------------------------------------------------------------------------
// Point + Point
impl<T: Num + Copy> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num + Copy> Point<T> {
    pub fn zeros() -> Self {
        <Self as Zero>::zero()
    }
}

// Point * const
impl<T: Num + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

// Point / const
impl<T: Num + Copy> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

// Point - Point
impl<T: Num + Copy> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// -Point
impl<T: Num + Copy + Signed> Neg for Point<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

// Zero trait
impl<T: Num + Copy> Zero for Point<T> {
    fn zero() -> Point<T> {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

//-------------------------------------------------------------------------
//                        Float implementations
//-------------------------------------------------------------------------

impl<T: Float> Point<T> {
    pub fn new_from_polar(r: T, phi: T) -> Self {
        let (s, c) = phi.sin_cos();
        Self::new(r * c, r * s)
    }

    // TODO(elsuizo:2021-06-15): maybe here we could use the fast sqrt
    pub fn length(&self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn angle(&self) -> T {
        T::atan2(self.y, self.x)
    }

    pub fn angle_deg(&self) -> T {
        self.angle().to_degrees()
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    // NOTE(elsuizo:2021-06-15): we mannage properly the zero division ... but in Cpp :P
    pub fn normalized(&self) -> Option<Self> {
        let length = self.length();
        if !nearly_zero(length) {
            // TODO(elsuizo:2021-06-15): impl Div for this
            Some(Self::new(self.x / length, self.y / length))
        } else {
            None
        }
    }

    // TODO(elsuizo:2021-06-15): test this function
    pub fn reflected(&self, normal: &Self) -> Self {
        let one = T::one();
        let two = one + one;
        *self - *normal * (normal.dot(self)) * two
    }

    pub fn perpendicular(&self) -> Self {
        Self::new(-self.y, self.x)
    }
}

