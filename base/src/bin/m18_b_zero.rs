trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($($t:ty),* $(,)?) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0 as $t
                }
            }
        )*
    };
}

impl_zero!(u8, u16, u32, u64, i32, i64, f32, f64);

fn main() {
    println!("{} {}", u8::zero(), f64::zero());
}
