use crate::bounds::Bounds;
use crate::point::{DecomposedPoint, Point};
use crate::segment::Segment;
use std::collections::HashMap;
use std::slice::Iter;

pub struct Polygon {
    pub points: Vec<Point>,
    pub segments: Vec<Segment>,
    pub bounds: Bounds,
}

impl Polygon {
    // polygons can be "malformed" (i.e. with holes, self intersecting, ...)
    // we are assuming that they are "well formed"
    pub fn from_points(points: Vec<Point>) -> Self {
        let is_at_least_a_triangle = points.len() > 2;
        debug_assert!(is_at_least_a_triangle);

        let there_are_nans = points
            .iter()
            .any(|point| f64::is_nan(point.x) || f64::is_nan(point.x));
        debug_assert!(!there_are_nans);

        let segments = points
            .iter()
            .zip(points.iter().skip(1))
            .map(|(start, end)| Segment::new(start.clone(), end.clone()))
            .collect::<Vec<_>>();

        let bounds = Bounds::from_points(&points);

        Self {
            points,
            segments,
            bounds,
        }
    }

    pub fn from_unordered_segments(unordered_segments: Vec<Segment>) -> Polygon {
        let mut start: DecomposedPoint = unordered_segments[0].end.clone().into();

        let n = unordered_segments.len();
        let mut hashmap: HashMap<DecomposedPoint, _> = HashMap::with_capacity(n);
        for segment in unordered_segments {
            hashmap.insert((&segment.end).into(), segment);
        }

        let mut points = Vec::with_capacity(n);
        // unordered_segments cannot be empty, panic
        while let Some(segment) = hashmap.remove(&start) {
            // TODO: A polygon's points array has ownership of start, how to move it to this new
            // array? Segment would have to take &mut Point and that would complicate things
            // sustantially
            points.push(segment.start.clone());
            start = segment.end.into();
        }

        Polygon::from_points(points)
    }

    pub fn iter_points(&self) -> Iter<'_, Point> {
        self.points.iter()
    }
}
