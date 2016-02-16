
pub trait MathFunc {
    type Output;
    type Base;
    fn exp(self) -> Self::Output;
    fn exp2(self) -> Self::Output;
    fn ln(self) -> Self::Output;
    fn log(self, base: Self::Base) -> Self::Output;
    fn log10(self) -> Self::Output;
    fn log2(self) -> Self::Output;
}


impl MathFunc for f32 {
    type Output = f32;
    type Base = f32;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        self.exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        self.exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        self.ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        self.log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        self.log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        self.log2()
    }
}

impl MathFunc for f64 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        self.exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        self.exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        self.ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        self.log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        self.log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        self.log2()
    }
}

impl MathFunc for i8 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for i16 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for i32 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for i64 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for u8 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for u16 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for u32 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}

impl MathFunc for u64 {
    type Output = f64;
    type Base = f64;
    #[inline(always)]
    fn exp(self) -> Self::Output {
        (self as f64).exp()
    }
    #[inline(always)]
    fn exp2(self) -> Self::Output{
        (self as f64).exp2()
    }
    #[inline(always)]
    fn ln(self) -> Self::Output{
        (self as f64).ln()
    }
    #[inline(always)]
    fn log(self, base: Self::Base) -> Self::Output{
        (self as f64).log(base)
    }
    #[inline(always)]
    fn log10(self) -> Self::Output{
        (self as f64).log10()
    }
    #[inline(always)]
    fn log2(self) -> Self::Output{
        (self as f64).log2()
    }
}