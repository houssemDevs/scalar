
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