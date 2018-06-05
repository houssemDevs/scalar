pub trait Zero {
  fn zero() -> Self;
}

pub trait One {
  fn one() -> Self;
}

impl Zero for f32 {
  fn zero() -> Self {
    0.0f32
  }
}

impl One for f32 {
  fn one() -> Self {
    1.0f32
  }
}

impl Zero for f64 {
  fn zero() -> Self {
    0.0f64
  }
}

impl One for f64 {
  fn one() -> Self {
    1.0f64
  }
}

impl Zero for i8 {
  fn zero() -> Self {
    0i8
  }
}

impl One for i8 {
  fn one() -> Self {
    1i8
  }
}

impl Zero for i16 {
  fn zero() -> Self {
    0i16
  }
}

impl One for i16 {
  fn one() -> Self {
    1i16
  }
}

impl Zero for i32 {
  fn zero() -> Self {
    0i32
  }
}

impl One for i32 {
  fn one() -> Self {
    1i32
  }
}

impl Zero for i64 {
  fn zero() -> Self {
    0i64
  }
}

impl One for i64 {
  fn one() -> Self {
    1i64
  }
}

impl Zero for u8 {
  fn zero() -> Self {
    0u8
  }
}

impl One for u8 {
  fn one() -> Self {
    1u8
  }
}

impl Zero for u16 {
  fn zero() -> Self {
    0u16
  }
}

impl One for u16 {
  fn one() -> Self {
    1u16
  }
}

impl Zero for u32 {
  fn zero() -> Self {
    0u32
  }
}

impl One for u32 {
  fn one() -> Self {
    1u32
  }
}

impl Zero for u64 {
  fn zero() -> Self {
    0u64
  }
}

impl One for u64 {
  fn one() -> Self {
    1u64
  }
}
