/// it'd probably be more efficient to use to_bits if we can be sure that there won't ever be nans
use std::mem;

#[derive(PartialEq, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct DecomposedPoint {
    pub x: (u64, i16, i8),
    pub y: (u64, i16, i8),
}

impl Into<DecomposedPoint> for &Point {
    fn into(self) -> DecomposedPoint {
        DecomposedPoint {
            x: integer_decode(self.x),
            y: integer_decode(self.y),
        }
    }
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}
