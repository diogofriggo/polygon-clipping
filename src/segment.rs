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
    sin_alpha: f64,
    cos_alpha: f64,
}

impl Segment {
    pub fn new(start: Point2d, end: Point2d) -> Self {
        let xs = start.x;
        let ys = start.y;
        let xe = end.x;
        let ye = end.y;

        let slope = (ye - ys) / (xe - xs);
        let offset = ys - slope * xs;

        let y_end = Point2d::new(start.x, end.y);
        let y = Vector2d::from_points(&start, &y_end);
        let w = Vector2d::from_points(&start, &end);
        let y_side = y.norm();
        let w_side = w.norm();
        let x_side = (w - y).norm();
        let sin_alpha = x_side / w_side;
        let cos_alpha = y_side / w_side;

        let (min_x, max_x) = min_max(&xs, &xe);
        let (min_y, max_y) = min_max(&ys, &ye);

        let bounds = Bounds {
            min_x: *min_x,
            max_x: *max_x,
            min_y: *min_y,
            max_y: *max_y,
        };

        // println!(
        //     "{} {} {} {} slope: {} offset: {} sin_alpha: {} cos_alpha: {}",
        //     xs, ys, xe, ye, slope, offset, sin_alpha, cos_alpha
        // );

        Self {
            start,
            end,
            slope,
            offset,
            bounds,
            sin_alpha,
            cos_alpha,
        }
    }

    pub fn intersections_with(&self, other_segment: &Segment) -> Vec<Point2d> {
        // there can be at most two points of intersection (segment start and segment end)
        let mut intersections = Vec::with_capacity(2);

        // prime values are used to ease reasoning, only unprimed ones are returned
        // one needs to be careful not to compare prime with unprime
        let start_prime = self.change_to_self_basis(&other_segment.start);
        let end_prime = self.change_to_self_basis(&other_segment.end);
        let other_segment_prime = Segment::new(start_prime, end_prime);

        let Segment {
            slope: slope_prime,
            start: start_prime,
            end: end_prime,
            ..
        } = other_segment_prime;

        let segments_are_parallel = slope_prime == 0.0;
        let segments_are_perpendicular = slope_prime.is_infinite();

        let self_length = (self.end.x - self.start.x).abs();
        if segments_are_parallel && start_prime.y == 0.0 {
            let start_is_in_self = start_prime.x >= 0.0 && start_prime.x <= self_length;
            if start_is_in_self {
                intersections.push(other_segment.start.clone());
            }

            let end_is_in_self = end_prime.x >= 0.0 && end_prime.x <= self_length;
            if end_is_in_self {
                intersections.push(other_segment.end.clone());
            }
        } else if segments_are_perpendicular {
            println!("prime: {start_prime}-->{end_prime} ... intersections_with(&{self}, {other_segment}: &Segment)");
            let is_in_self = start_prime.x >= 0.0 && start_prime.x <= self_length;
            if is_in_self {
                let intersection = Point2d::new(self.start.x, other_segment.start.y);
                intersections.push(intersection);
            }
        } else {
            let (&min_y, &max_y) = min_max(&start_prime.y, &end_prime.y);
            let crosses_x_axis = min_y <= 0.0 && max_y >= 0.0;

            let (&min_x, &max_x) = min_max(&start_prime.x, &end_prime.x);
            let min_is_in_self = min_x >= 0.0 && min_x <= self_length;
            let max_is_in_self = max_x >= 0.0 && max_x <= self_length;

            let intersection_exists = crosses_x_axis && (min_is_in_self || max_is_in_self);
            if intersection_exists {
                let intersection = Self::compute_intersection(self, other_segment);
                intersections.push(intersection);
            }
        }

        intersections
    }

    // assumes that intersection exists
    fn compute_intersection(segment: &Segment, other_segment: &Segment) -> Point2d {
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

    fn change_to_self_basis(&self, point: &Point2d) -> Point2d {
        let x0 = self.start.x;
        let y0 = self.start.y;
        let Point2d { x, y } = point;

        let Self {
            sin_alpha,
            cos_alpha,
            ..
        } = self;

        let x_prime = (x - x0) * sin_alpha + (y - y0) * cos_alpha;
        let y_prime = (x - x0) * cos_alpha - (y - y0) * sin_alpha;
        //println!("change of basis... self: {self} point: {point} prime ({x_prime}, {y_prime})");
        Point2d::new(x_prime, y_prime)
    }

    pub fn points_inwards(&self, segment: &Segment) -> bool {
        let midpoint = (&self.start + &self.end) / 2.0;
        let along: Vector3d = Vector2d::from_points(&midpoint, &segment.end).into();
        let up = Vector3d::z();
        let perp = up.curl(along);

        let vector: Vector3d = Vector2d::from_points(&segment.start, &segment.end).into();
        vector.dot(&perp) >= 0.0
    }
}

fn min_max<'a>(a: &'a f64, b: &'a f64) -> (&'a f64, &'a f64) {
    if a.partial_cmp(b) == Some(Less) {
        (a, b)
    } else {
        (b, a)
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-->{}", self.start, self.end)
    }
}
