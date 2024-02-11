/// it'd probably be more efficient to use to_bits if we can be sure that there won't ever be nans
use std::{
    fmt::Display,
    ops::{Add, Div, Sub},
};

use crate::segment::Segment;

#[derive(PartialEq, Clone, Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
    pub key: (u64, u64),
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        let key = (x.to_bits(), y.to_bits());
        Self { x, y, key }
    }

    pub fn is_inside_of(&self, mould_segments: &[Segment]) -> bool {
        // if all vectors from segment[i].start to self are pointing inward
        for mould_segment in mould_segments {
            let other = Segment::new(mould_segment.start.clone(), self.clone());
            let points_outwards = !other.points_inwards_of(mould_segment);
            if points_outwards {
                return false;
            }
        }

        true
    }
}

impl Add for &Point2d {
    type Output = Point2d;

    fn add(self, other: &Point2d) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let key = (x.to_bits(), y.to_bits());
        Point2d { x, y, key }
    }
}

impl Sub for &Point2d {
    type Output = Point2d;

    fn sub(self, other: &Point2d) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let key = (x.to_bits(), y.to_bits());
        Point2d { x, y, key }
    }
}

impl Sub for Point2d {
    type Output = Point2d;

    fn sub(self, other: Point2d) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let key = (x.to_bits(), y.to_bits());
        Point2d { x, y, key }
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
        let x = point.x;
        let y = point.y;
        let key = (x.to_bits(), y.to_bits());
        Self { x, y, key }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct DecomposedPoint {
    pub x: (u64, i16, i8),
    pub y: (u64, i16, i8),
}

impl From<&Point2d> for DecomposedPoint {
    fn from(val: &Point2d) -> Self {
        DecomposedPoint {
            x: integer_decode(val.x),
            y: integer_decode(val.y),
        }
    }
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = val.to_bits();
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

impl Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
