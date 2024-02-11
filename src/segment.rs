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

    // TODO: stop computing intersections twice (a,b then b,a)
    pub fn intersections_with(&self, other: &Segment) -> Vec<Point2d> {
        // there can be at most two points of intersection (segment start and segment end)
        let mut intersections = Vec::with_capacity(2);

        if self.slope.is_infinite() && other.slope.is_infinite() {
            let segments_are_colinear = self.start.x == other.start.x;
            if segments_are_colinear {
                if self.contains_point_within_y(&other.start) {
                    intersections.push(other.start.clone());
                }

                if self.contains_point_within_y(&other.end) {
                    intersections.push(other.end.clone());
                }
            }
        } else if self.slope == 0.0 && other.slope == 0.0 {
            let segments_are_colinear = self.start.y == other.start.y;
            if segments_are_colinear {
                if self.contains_point_within_x(&other.start) {
                    intersections.push(other.start.clone());
                }

                if self.contains_point_within_x(&other.end) {
                    intersections.push(other.end.clone());
                }
            }
        } else if self.slope.is_infinite() {
            if let Some(intersection) = Self::infinite_intersection(self, other) {
                intersections.push(intersection);
            }
        } else if other.slope.is_infinite() {
            if let Some(intersection) = Self::infinite_intersection(other, self) {
                intersections.push(intersection);
            }
        } else {
            // there's no intersection iff the two segments are parallel
            if let Some(intersection) = Self::intersection(self, other) {
                if self.boxes(&intersection) && other.boxes(&intersection) {
                    intersections.push(intersection);
                }
            }
        }

        // println!("intersections between {self} and {other} : {intersections:?}");
        intersections
    }

    fn infinite_intersection(
        infinite_segment: &Segment,
        finite_segment: &Segment,
    ) -> Option<Point2d> {
        let y = finite_segment.slope * infinite_segment.start.x + finite_segment.offset;
        let intersection = Point2d::new(infinite_segment.start.x, y);
        if infinite_segment.boxes(&intersection) && finite_segment.boxes(&intersection) {
            Some(intersection)
        } else {
            None
        }
    }

    // assumes that intersection exists
    fn intersection(segment: &Segment, other_segment: &Segment) -> Option<Point2d> {
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

        let segments_are_parallel = a == b || (a.is_infinite() && b.is_infinite());
        if segments_are_parallel {
            None
        } else {
            // y = ax + b
            // y = cx + d
            // 0 = (a-c)x + (b-d)
            let x = (d - b) / (a - c);
            let y = a * x + b;

            let point = Point2d::new(x, y);
            Some(point)
        }
    }

    pub fn points_inwards_of(&self, mould_segment: &Segment) -> bool {
        let along: Vector3d =
            Vector2d::from_points(&mould_segment.start, &mould_segment.end).into();
        // in a dextrogirous system if y goes down z goes into
        let up = -Vector3d::z(); // if polygons were to be defined clockwise
                                 // let down = -Vector3d::z(); // if polygons were to be defined clockwise
        let ortho = up.curl(&along);

        let vector: Vector3d = Vector2d::from_points(&self.start, &self.end).into();
        vector.dot(&ortho) >= 0.0
    }

    pub fn contains_point_within_x(&self, point: &Point2d) -> bool {
        point.x >= self.bounds.min_x && point.x <= self.bounds.max_x
    }

    pub fn contains_point_within_y(&self, point: &Point2d) -> bool {
        point.y >= self.bounds.min_y && point.y <= self.bounds.max_y
    }

    pub fn boxes(&self, point: &Point2d) -> bool {
        self.contains_point_within_x(point) && self.contains_point_within_y(point)
    }

    pub fn is_point(&self) -> bool {
        self.start == self.end
    }

    pub fn is_inside_of_or_touches(&self, mould_segments: &[Segment]) -> bool {
        // let start = self.start.is_inside_of(mould_segments);
        // let end = self.end.is_inside_of(mould_segments);
        // println!(
        //     "is {self} inside of {} ??? : {start} {end}",
        //     mould_segments[0]
        // );
        self.start.is_inside_of_or_touches(mould_segments)
            && self.end.is_inside_of_or_touches(mould_segments)
    }

    pub fn is_collinear_with(&self, mould_segment: &Segment) -> bool {
        self.slope == mould_segment.slope
            || (self.slope.is_infinite() && mould_segment.slope.is_infinite())
    }

    // I could project the point in self's basis but I went down that road before
    pub fn contains(&self, point: &Point2d) -> bool {
        // let y = self.slope * point.x + self.offset;
        let slope = (point.y - self.start.y) / (point.x - self.start.x);
        // TODO: f64 comparisons behind the scenes!!!
        self.boxes(point) && (point == &self.start || slope == self.slope)
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
