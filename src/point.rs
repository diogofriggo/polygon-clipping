use crate::{segment::Segment, vector::Vector3d};
use float_cmp::ApproxEq;
/// it'd probably be more efficient to use to_bits if we can be sure that there won't ever be nans
use std::{
    fmt::Display,
    ops::{Add, Div, Sub},
};

#[derive(PartialEq, Clone, Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
    pub key: (u64, u64),
}

const EPSILON_MARGIN: f64 = 1e-14;
const ULP_MARGIN: i64 = 1;
const MARGIN: (f64, i64) = (EPSILON_MARGIN, ULP_MARGIN);

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        let key = (x.to_bits(), y.to_bits());
        Self { x, y, key }
    }

    pub fn is_inside_of_old(&self, mould_segments: &[Segment]) -> bool {
        // if all vectors from segment[i].start to self are pointing inwards
        let up = -Vector3d::z();
        for mould_segment in mould_segments {
            let along = &mould_segment.end - &mould_segment.start;
            let along: Vector3d = along.into();
            let inwards = up.curl(&along);
            // println!("mould_segment: {mould_segment} inwards: {inwards}");

            // end - start
            let to_point = self - &mould_segment.start;
            let to_point = Vector3d::from_coordinates(to_point.x, to_point.y, 0.0);
            let numerator = inwards.dot(&to_point);
            let denominator = inwards.norm() * to_point.norm();
            let is_outside = (numerator / denominator).cos() < 0.0;
            if is_outside {
                // println!("{self} is OUTSIDE of {}", mould_segments[0]);
                return false;
            }
        }
        // println!("{self} is INSIDE of {}", mould_segments[0]);
        true
    }

    pub fn is_inside_of_or_touches(&self, mould_segments: &[Segment]) -> bool {
        let is_inside = !self.is_outside_of(mould_segments);
        let touches = self.touches(mould_segments);
        is_inside || touches
    }

    // TODO: what about a collinear point ?
    pub fn is_outside_of(&self, mould_segments: &[Segment]) -> bool {
        // TODO: numerical error??????

        let total_angular_distance = mould_segments
            .iter()
            .map(|mould_segment| self.angular_distance(mould_segment))
            .sum::<f64>();

        // if total_angular_distance.approx_eq(0.0, MARGIN) {
        //     println!(
        //         "point: {self} is outside of mould: {} {total_angular_distance}",
        //         mould_segments[0]
        //     );
        // }
        total_angular_distance.approx_eq(0.0, MARGIN)
    }

    fn angular_distance(&self, segment: &Segment) -> f64 {
        let a = &segment.start - self;
        let a: Vector3d = a.into();

        let up = -Vector3d::z();
        let ortho = a.curl(&up);

        let b = &segment.end - self;
        let b: Vector3d = b.into();

        let sign = if ortho.dot(&b) < 0.0 { -1.0 } else { 1.0 };

        let c = &b - &a;
        let numerator = a.norm_sq() + b.norm_sq() - c.norm_sq();
        let denominator = 2.0 * a.norm() * b.norm();
        let cos_theta = numerator / denominator;
        sign * cos_theta.acos()
    }

    fn touches(&self, segments: &[Segment]) -> bool {
        for segment in segments {
            if segment.contains(self) {
                return true;
            }
        }

        false
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
