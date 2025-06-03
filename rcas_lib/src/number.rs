use num_bigfloat::BigFloat;
use num_bigint::BigInt;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use std::string::ToString;

fn int_to_float(input: BigInt) -> BigFloat {
    BigFloat::from_str(&input.to_string()).expect("HUH TFFFF")
}

macro_rules! impl_arith_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Number {
            type Output = Number;

            fn $method(self, rhs: Self) -> Self::Output {
                let float_self = match self {
                    Number::Int(i) => int_to_float(i),
                    Number::Float(t) => t
                };

                let float_rhs = match rhs {
                    Number::Int(i) => int_to_float(i),
                    Number::Float(t) => t
                };

                let res = float_self $op float_rhs;
                
                match res.to_
            }
        }
    };
}

/// An abstraction over integer or float - will try and keep to integers as much as possible but
/// will cast to float if it is needed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    Int(BigInt),
    Float(BigFloat),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(int) => write!(f, "{}", int.to_string()),
            Number::Float(float) => write!(f, "{}", float.to_string()),
        }
    }
}

impl_arith_op!(Add, add, +);
impl_arith_op!(Sub, sub, -);
impl_arith_op!(Mul, mul, *);

