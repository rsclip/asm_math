#![allow(unused_assignments)]

use std::arch::asm;

/// Manual implementation of 128 bit unsigned integer
#[repr(C)]
#[derive(Clone, Copy)]
pub struct U128 {
    /// High 64 bits
    high: u64,
    /// Low 64 bits
    low: u64,
}

impl U128 {
    /// Add two 128 bit integers
    /// Returns a tuple of high and low 64 bit integers
    fn addc(&self, other: &Self) -> (u64, u64) {
        let mut result_high = 0u64;
        let mut result_low = 0u64;
        let mut overflow_flag: i8 = 0;

        unsafe {
            asm!(
                "add {low1}, {low2}",
                "adc {high1}, {high2}",
                "setc {ovf}",
                low1 = inout(reg) self.low => result_low,
                high1 = inout(reg) self.high => result_high,
                low2 = in(reg) other.low,
                high2 = in(reg) other.high,
                ovf = out(reg_byte) overflow_flag, // Use reg_byte for a byte-sized register part
                options(nostack),
            );
        }

        if overflow_flag == 1 {
            panic!("U128 addition overflow")
        }

        (result_high, result_low)
    }

    /// Subtract two 128 bit integers
    /// Returns a tuple of high and low 64 bit integers
    fn subc(&self, other: &Self) -> (u64, u64) {
        let mut result_high = 0u64;
        let mut result_low = 0u64;
        let mut overflow_flag: i8 = 0;

        unsafe {
            asm!(
                "sub {low1}, {low2}",
                "sbb {high1}, {high2}",
                "setc {ovf}",
                low1 = inout(reg) self.low => result_low,
                high1 = inout(reg) self.high => result_high,
                low2 = in(reg) other.low,
                high2 = in(reg) other.high,
                ovf = out(reg_byte) overflow_flag,
                options(nostack),
            );
        }

        if overflow_flag == 1 {
            panic!("U128 subtraction overflow")
        }

        (result_high, result_low)
    }

    /// Multiply two 128 bit integers
    /// Returns a tuple of high and low 64 bit integers
    fn mulc(&self, other: &Self) -> (u64, u64) {
        unimplemented!()
    }
}

mod op_impl {
    use super::U128;

    impl std::ops::Add for U128 {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let (high, low) = self.addc(&other);
            Self { high, low }
        }
    }

    impl std::ops::Sub for U128 {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            let (high, low) = self.subc(&other);
            Self { high, low }
        }
    }

    impl std::ops::Mul for U128 {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            let (high, low) = self.mulc(&other);
            Self { high, low }
        }
    }
}

mod misc_impl {
    use super::U128;

    impl std::fmt::Debug for U128 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "U128({:b} {:b})", self.high, self.low)
        }
    }

    impl std::fmt::Display for U128 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let v: u128 = (self.high as u128) << 64 | self.low as u128;
            write!(f, "{}", v)
        }
    }

    impl Default for U128 {
        fn default() -> Self {
            Self { high: 0, low: 0 }
        }
    }
}

mod from_impl {
    use super::U128;

    impl From<u128> for U128 {
        fn from(data: u128) -> Self {
            Self {
                high: (data >> 64) as u64,
                low: data as u64,
            }
        }
    }

    impl From<u64> for U128 {
        fn from(data: u64) -> Self {
            Self { high: 0, low: data }
        }
    }

    impl From<u32> for U128 {
        fn from(data: u32) -> Self {
            Self {
                high: 0,
                low: data as u64,
            }
        }
    }

    impl From<u16> for U128 {
        fn from(data: u16) -> Self {
            Self {
                high: 0,
                low: data as u64,
            }
        }
    }

    impl From<u8> for U128 {
        fn from(data: u8) -> Self {
            Self {
                high: 0,
                low: data as u64,
            }
        }
    }

    impl From<usize> for U128 {
        fn from(data: usize) -> Self {
            Self {
                high: 0,
                low: data as u64,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let a = U128::from(10u64);
        let b = U128::from(20u8);
        let c = a + b;
        println!("{a:?} ({a}) + {b:?} ({b}) = {c:?} ({c})");
        assert_eq!(c.low, 30);
        assert_eq!(c.high, 0);
    }

    #[test]
    fn overflow_safe() {
        let a = U128::from(u64::MAX);
        let b = U128::from(1usize);
        let c = a + b;
        println!("{a:?} ({a}) + {b:?} ({b}) = {c:?} ({c})");
        assert_eq!(c.low, 0);
        assert_eq!(c.high, 1);
    }

    #[test]
    #[should_panic]
    fn overflow_unsafe() {
        let a = U128::from(u128::MAX);
        let d = a + a + a + a;
        println!("{a:?} ({a}) * 4 = {d:?} ({d})");
    }

    #[test]
    fn sub() {
        let a = U128::from(20u64);
        let b = U128::from(10u8);
        let c = a - b;
        println!("{a:?} ({a}) - {b:?} ({b}) = {c:?} ({c})");
        assert_eq!(c.low, 10);
        assert_eq!(c.high, 0);
    }

    #[test]
    #[should_panic]
    fn sub_overflow() {
        let a = U128::from(10u64);
        let b = U128::from(20u8);
        let c = a - b;
        println!("{a:?} ({a}) - {b:?} ({b}) = {c:?} ({c})");
        assert_eq!(c.low, u64::MAX);
        assert_eq!(c.high, u64::MAX);
    }

    #[test]
    fn mul() {
        let a = U128::from(10u64);
        let b = U128::from(20u8);
        let c = a * b;
        println!("{a:?} ({a}) * {b:?} ({b}) = {c:?} ({c})");
        assert_eq!(c.low, 200);
        assert_eq!(c.high, 0);
    }

    #[test]
    #[should_panic]
    fn mul_overflow() {
        let a = U128::from(u64::MAX);
        let b = U128::from(2usize);
        let c = a * b;
        println!("{a:?} ({a}) * {b:?} ({b}) = {c:?} ({c})");
        assert_eq!(c.low, u64::MAX - 1);
        assert_eq!(c.high, 1);
    }

    // #[test]
    // fn div() {
    //     let a = U128::from(20u64);
    //     let b = U128::from(10u8);
    //     let c = a / b;
    //     println!("{a:?} ({a}) / {b:?} ({b}) = {c:?} ({c})");
    //     assert_eq!(c.low, 2);
    //     assert_eq!(c.high, 0);
    // }

    // #[test]
    // #[should_panic]
    // fn div_overflow() {
    //     let a = U128::from(u64::MAX);
    //     let b = U128::from(0usize);
    //     let c = a / b;
    //     println!("{a:?} ({a}) / {b:?} ({b}) = {c:?} ({c})");
    //     assert_eq!(c.low, u64::MAX);
    //     assert_eq!(c.high, 0);
    // }
}
