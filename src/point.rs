/// it'd probably be more efficient to use to_bits if we can be sure that there won't ever be nans
use std::{
    mem,
    ops::{Add, Div},
};

#[derive(PartialEq, Clone, Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add for &Point2d {
    type Output = Point2d;

    fn add(self, other: &Point2d) -> Self::Output {
        Point2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Div<f64> for Point2d {
    type Output = Point2d;

    fn div(self, factor: f64) -> Self::Output {
        Point2d::new(self.x / factor, self.y / factor)
    }
}

impl From<Point2d> for Point3d {
    fn from(point: Point2d) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: f64::default(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl From<Point3d> for Point2d {
    fn from(point: Point3d) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct DecomposedPoint {
    pub x: (u64, i16, i8),
    pub y: (u64, i16, i8),
}

impl Into<DecomposedPoint> for &Point2d {
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
