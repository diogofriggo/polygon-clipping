use crate::bounds::Bounds;
use crate::point::{DecomposedPoint, Point2d};
use crate::segment::Segment;
use std::collections::HashMap;
use std::slice::Iter;

#[derive(Debug)]
pub struct Polygon {
    pub points: Vec<Point2d>,
    pub segments: Vec<Segment>,
    pub bounds: Bounds,
}

impl Polygon {
    // polygons can be "malformed" (i.e. with holes, self intersecting, ...)
    // we are assuming that they are "well formed"
    pub fn from_points(start_points: Vec<Point2d>) -> Self {
        let is_at_least_a_triangle = start_points.len() > 2;
        debug_assert!(is_at_least_a_triangle);

        let there_are_nans = start_points
            .iter()
            .any(|point| f64::is_nan(point.x) || f64::is_nan(point.x));
        debug_assert!(!there_are_nans);

        let mut end_points = start_points.clone();
        end_points.rotate_left(1);
        let segments = start_points
            .iter()
            .zip(end_points)
            .map(|(start, end)| Segment::new(start.clone(), end.clone()))
            .collect::<Vec<_>>();

        let bounds = Bounds::from_points(&start_points);

        Self {
            points: start_points,
            segments,
            bounds,
        }
    }

    pub fn from_unordered_segments(unordered_segments: Vec<Segment>) -> Polygon {
        // we mean to panic if unordered_segments is empty
        let mut start: DecomposedPoint = (&unordered_segments[0].end).into();

        let n = unordered_segments.len();
        let mut hashmap: HashMap<DecomposedPoint, _> = HashMap::with_capacity(n);
        for segment in unordered_segments {
            hashmap.insert((&segment.end).into(), segment);
        }

        let mut points = Vec::with_capacity(n);
        let mut segments = Vec::with_capacity(n);
        // unordered_segments cannot be empty, panic
        while let Some(segment) = hashmap.remove(&start) {
            // TODO: A polygon's points array has ownership of start, how to move it to this new
            // array? Segment would have to take &mut Point and that would complicate things
            // sustantially
            points.push(segment.start.clone());
            start = (&segment.end).into();
            segments.push(segment);
        }

        let bounds = Bounds::from_points(&points);

        Self {
            points,
            segments,
            bounds,
        }
    }

    pub fn iter_points(&self) -> Iter<'_, Point2d> {
        self.points.iter()
    }
}
