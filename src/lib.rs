#![feature(op_assign_traits)]

use std::default::*;
use std::ops::*;

mod abs;
mod ident;

pub use abs::*;
pub use ident::*;

pub trait Scalar:
    Copy
    + PartialEq
    + PartialOrd
    + Default
    + Zero
    + One
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Neg<Output = Self>
    + Abs
{
}

impl<S> Scalar for S
where
    S: Copy
        + PartialEq
        + PartialOrd
        + Default
        + Zero
        + One
        + Add<S, Output = S>
        + AddAssign<S>
        + Sub<S, Output = S>
        + SubAssign<S>
        + Mul<S, Output = S>
        + MulAssign<S>
        + Div<S, Output = S>
        + DivAssign<S>
        + Rem<S, Output = S>
        + RemAssign<S>
        + Neg<Output = S>
        + Abs,
{
}
