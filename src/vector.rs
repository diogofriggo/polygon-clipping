use std::ops::Sub;

use crate::{point::Point2d, segment::Segment};

pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn from_coordinates(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_points(start: &Point2d, end: &Point2d) -> Self {
        Self {
            x: end.x - start.x,
            y: end.y - start.y,
        }
    }

    pub fn dot(&self, vector: &Vector2d) -> f64 {
        self.x * vector.x + self.y * vector.y
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Self) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector2d::from_coordinates(x, y)
    }
}

pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn from_coordinates(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // pub fn from_points(start: &Point3d, end: &Point3d) -> Self {
    //     Self {
    //         x: end.x - start.x,
    //         y: end.y - start.y,
    //         z: end.z - start.z,
    //     }
    // }

    pub fn z() -> Self {
        Self::from_coordinates(f64::default(), f64::default(), 1.0)
    }

    pub fn dot(&self, vector: &Vector3d) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    pub fn curl(&self, along: Vector3d) -> Vector3d {
        let Vector3d {
            x: ax,
            y: ay,
            z: az,
        } = self;

        let Vector3d {
            x: bx,
            y: by,
            z: bz,
        } = along;

        let x = ay * bz - az * by;
        let y = -(ax * bz - az * bx);
        let z = ax * by - ay * bx;

        Self::from_coordinates(x, y, z)
    }
}

impl From<Vector2d> for Vector3d {
    fn from(vector: Vector2d) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: f64::default(),
        }
    }
}

impl From<&Segment> for Vector2d {
    fn from(segment: &Segment) -> Self {
        let Segment { start, end, .. } = segment;
        let point = end - start;
        Self::from_coordinates(point.x, point.y)
    }
}
