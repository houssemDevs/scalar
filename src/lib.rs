#![feature(op_assign_traits)]
#![feature(zero_one)]

use std::num::*;
use std::default::*;
use std::ops::*;

pub trait Scalare:
    Copy + PartialEq + PartialOrd
    +One + Zero
    +Default
    +Add<Self, Output=Self> + AddAssign<Self>
    +Sub<Self, Output=Self> + SubAssign<Self>
    +Mul<Self, Output=Self> + MulAssign<Self>
    +Div<Self, Output=Self> + DivAssign<Self>
    +Rem<Self,Output=Self> + RemAssign<Self>
    +Neg<Output=Self>
    +Abs
    {}

impl<S> Scalare for S
    where S:
    Copy + PartialEq + PartialOrd
    +One + Zero
    +Default
    +Add<S, Output=S> + AddAssign<S>
    +Sub<S, Output=S> + SubAssign<S>
    +Mul<S, Output=S> + MulAssign<S>
    +Div<S, Output=S> + DivAssign<S>
    +Rem<S,Output=S> + RemAssign<S>
    +Neg<Output=S>
    +Abs
    {}

pub trait Abs {
    fn abs(self) -> Self;
}


impl Abs for f32 {
    fn abs(self) -> f32 {
        self.abs()
    }
}

impl Abs for f64 {
    fn abs(self) -> f64 {
        self.abs()
    }
}

impl Abs for i8 {
    fn abs(self) -> i8 {
        self.abs()
    }
}

impl Abs for i16 {
    fn abs(self) -> i16 {
        self.abs()
    }
}

impl Abs for i32 {
    fn abs(self) -> i32 {
        self.abs()
    }
}

impl Abs for i64 {
    fn abs(self) -> i64 {
        self.abs()
    }
}

impl Abs for u8 {
    fn abs(self) -> u8 {
        self
    }
}

impl Abs for u16 {
    fn abs(self) -> u16 {
        self
    }
}

impl Abs for u32 {
    fn abs(self) -> u32 {
        self
    }
}

impl Abs for u64 {
    fn abs(self) -> u64 {
        self
    }
}