use crate::bounds::Bounds;
use crate::point::Point2d;
use crate::vector::{Vector2d, Vector3d};
use std::cmp::Ordering::*;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Segment {
    pub start: Point2d,
    pub end: Point2d,
    slope: f64,
    offset: f64,
    bounds: Bounds,
}

impl Segment {
    pub fn new(start: Point2d, end: Point2d) -> Self {
        let xs = start.x;
        let ys = start.y;
        let xe = end.x;
        let ye = end.y;

        let slope = (ye - ys) / (xe - xs);
        let offset = ys - slope * xs;

        let (&min_x, &max_x) = min_max(&xs, &xe);
        let (&min_y, &max_y) = min_max(&ys, &ye);

        let bounds = Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        };

        Self {
            start,
            end,
            slope,
            offset,
            bounds,
        }
    }

    pub fn intersections_with(&self, other: &Segment) -> Vec<Point2d> {
        // there can be at most two points of intersection (segment start and segment end)
        let mut intersections = Vec::with_capacity(2);

        let segments_are_parallel = self.slope == other.slope;

        if segments_are_parallel {
            let segments_are_colinear = self.start.y == other.start.y;
            if segments_are_colinear {
                if self.contains_point_within_x(&other.start) {
                    intersections.push(other.start.clone());
                }

                if self.contains_point_within_x(&other.end) {
                    intersections.push(other.end.clone());
                }
            }
        // perpendicular
        } else if self.slope == 0.0 && other.slope.is_infinite() {
            if let Some(intersection) = Self::perpendicular_intersection(self, other) {
                intersections.push(intersection);
            }
        // perpendicular
        } else if self.slope.is_infinite() && other.slope == 0.0 {
            if let Some(intersection) = Self::perpendicular_intersection(other, self) {
                intersections.push(intersection);
            }
        } else {
            let intersection = Self::intersection(self, other);
            if self.contains_point_within_x(&intersection)
                && self.contains_point_within_y(&intersection)
            {
                intersections.push(intersection);
            }
        }

        // println!("intersections between {self} and {other} : {intersections:?}");
        intersections
    }

    fn perpendicular_intersection(
        horizontal_segment: &Segment,
        vertical_segment: &Segment,
    ) -> Option<Point2d> {
        if horizontal_segment.contains_point_within_x(&vertical_segment.start)
            && vertical_segment.contains_point_within_y(&horizontal_segment.start)
        {
            return Some(Point2d::new(
                vertical_segment.start.x,
                horizontal_segment.start.y,
            ));
        }
        None
    }

    // assumes that intersection exists
    fn intersection(segment: &Segment, other_segment: &Segment) -> Point2d {
        let Segment {
            slope: a,
            offset: b,
            ..
        } = segment;

        let Segment {
            slope: c,
            offset: d,
            ..
        } = other_segment;

        // y = ax + b
        // y = cx + d
        // 0 = (a-c)xk + (b-d)
        let x = (b - d) / (a - c);
        let y = a * x + b;

        Point2d::new(x, y)
    }

    pub fn points_inwards(&self, segment: &Segment) -> bool {
        let midpoint = (&self.start + &self.end) / 2.0;
        let along: Vector3d = Vector2d::from_points(&midpoint, &segment.end).into();
        let up = Vector3d::z();
        let perp = up.curl(along);

        let vector: Vector3d = Vector2d::from_points(&segment.start, &segment.end).into();
        vector.dot(&perp) >= 0.0
    }

    pub fn contains_point_within_x(&self, point: &Point2d) -> bool {
        point.x >= self.bounds.min_x && point.x <= self.bounds.max_x
    }

    pub fn contains_point_within_y(&self, point: &Point2d) -> bool {
        point.y >= self.bounds.min_y && point.y <= self.bounds.max_y
    }

    pub fn is_point(&self) -> bool {
        self.start == self.end
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-->{}", self.start, self.end)
    }
}

fn min_max<'a>(a: &'a f64, b: &'a f64) -> (&'a f64, &'a f64) {
    if a.partial_cmp(b) == Some(Less) {
        (a, b)
    } else {
        (b, a)
    }
}
