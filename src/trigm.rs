pub trait Trigonometry {
    type Output;
    fn sin(self) -> Self::Output;
    fn asin(self) -> Self::Output;
    fn sinh(self) -> Self::Output;
    fn asinh(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn acos(self) -> Self::Output;
    fn cosh(self) -> Self::Output;
    fn acosh(self) -> Self::Output;
    fn tan(self) -> Self::Output;
    fn atan(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
    fn atanh(self) -> Self::Output;
}

impl Trigonometry for f32 {
    type Output = f32;
    fn sin(self) -> Self::Output {
        self.sin()
    }
    fn asin(self) -> Self::Output {
        self.asin()
    }
    fn sinh(self) -> Self::Output {
        self.sinh()
    }
    fn asinh(self) -> Self::Output {
        self.asinh()
    }
    fn cos(self) -> Self::Output {
        self.cos()
    }
    fn acos(self) -> Self::Output {
        self.acos()
    }
    fn cosh(self) -> Self::Output {
        self.cosh()
    }
    fn acosh(self) -> Self::Output {
        self.acosh()
    }
    fn tan(self) -> Self::Output {
        self.tan()
    }
    fn atan(self) -> Self::Output {
        self.atan()
    }
    fn tanh(self) -> Self::Output {
        self.tan()
    }
    fn atanh(self) -> Self::Output {
        self.atanh()
    }
}

impl Trigonometry for f64 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        self.sin()
    }
    fn asin(self) -> Self::Output {
        self.asin()
    }
    fn sinh(self) -> Self::Output {
        self.sinh()
    }
    fn asinh(self) -> Self::Output {
        self.asinh()
    }
    fn cos(self) -> Self::Output {
        self.cos()
    }
    fn acos(self) -> Self::Output {
        self.acos()
    }
    fn cosh(self) -> Self::Output {
        self.cosh()
    }
    fn acosh(self) -> Self::Output {
        self.acosh()
    }
    fn tan(self) -> Self::Output {
        self.tan()
    }
    fn atan(self) -> Self::Output {
        self.atan()
    }
    fn tanh(self) -> Self::Output {
        self.tan()
    }
    fn atanh(self) -> Self::Output {
        self.atanh()
    }
}

impl Trigonometry for i8 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for i16 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for i32 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for i64 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for u8 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for u16 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for u32 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}

impl Trigonometry for u64 {
    type Output = f64;
    fn sin(self) -> Self::Output {
        (self as f64).sin()
    }
    fn asin(self) -> Self::Output {
        (self as f64).asin()
    }
    fn sinh(self) -> Self::Output {
        (self as f64).sinh()
    }
    fn asinh(self) -> Self::Output {
        (self as f64).asinh()
    }
    fn cos(self) -> Self::Output {
        (self as f64).cos()
    }
    fn acos(self) -> Self::Output {
        (self as f64).acos()
    }
    fn cosh(self) -> Self::Output {
        (self as f64).cosh()
    }
    fn acosh(self) -> Self::Output {
        (self as f64).acosh()
    }
    fn tan(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atan(self) -> Self::Output {
        (self as f64).atan()
    }
    fn tanh(self) -> Self::Output {
        (self as f64).tan()
    }
    fn atanh(self) -> Self::Output {
        (self as f64).atanh()
    }
}