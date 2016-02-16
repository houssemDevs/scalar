#![feature(op_assign_traits)]
#![feature(zero_one)]

use std::num::*;
use std::default::*;
use std::ops::*;

mod abs;
mod trigm;
mod math_func;

pub use abs::*;
pub use trigm::*;
pub use math_func::*;

pub trait Scalar:
    Copy + PartialEq + PartialOrd
    +One + Zero
    +Default
    +Add<Self, Output=Self> + AddAssign<Self>
    +Sub<Self, Output=Self> + SubAssign<Self>
    +Mul<Self, Output=Self> + MulAssign<Self>
    +Div<Self, Output=Self> + DivAssign<Self>
    +Rem<Self, Output=Self> + RemAssign<Self>
    +Neg<Output=Self>
    +Abs
    +Trigonometry
    +MathFunc
    {}

impl<S> Scalar for S
    where S:
    Copy + PartialEq + PartialOrd
    +One + Zero
    +Default
    +Add<S, Output=S> + AddAssign<S>
    +Sub<S, Output=S> + SubAssign<S>
    +Mul<S, Output=S> + MulAssign<S>
    +Div<S, Output=S> + DivAssign<S>
    +Rem<S, Output=S> + RemAssign<S>
    +Neg<Output=S>
    +Abs
    +Trigonometry
    +MathFunc
    {}

