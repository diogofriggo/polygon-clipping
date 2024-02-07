use crate::point::{DecomposedPoint, Point};
use crate::segment::Segment;
use std::collections::HashMap;
use std::slice::Iter;

pub struct Polygon<'a> {
    pub points: Vec<Point>,
    pub segments: Vec<Segment<'a>>,
    pub bounds: Bounds,
}

impl<'a> Polygon<'a> {
    // polygons can be "malformed" (i.e. with holes, self intersecting, ...)
    // we are assuming that they are "well formed"
    pub fn from_points(points: Vec<Point>) -> Self {
        let is_at_least_a_triangle = points.len() > 2;
        debug_assert!(is_at_least_a_triangle);

        let there_are_nans = points
            .iter()
            .any(|point| f64::is_nan(point.x) || f64::is_nan(point.x));
        debug_assert!(there_are_nans == false);

        let segments = points
            .iter()
            .zip(points.iter().skip(1))
            .map(|(start, end)| Segment::new(start, end))
            .collect::<Vec<_>>();

        let bounds = Bounds::from_points(&points);

        Self {
            points,
            segments,
            bounds,
        }
    }

    pub fn from_unordered_segments(unordered_segments: Vec<Segment<'_>>) -> Polygon<'a> {
        let n = unordered_segments.len();
        let mut hashmap: HashMap<DecomposedPoint, _> = HashMap::with_capacity(n);
        for segment in unordered_segments {
            hashmap.insert(segment.start.into(), segment);
        }

        let mut points = Vec::with_capacity(n);
        let mut segments = Vec::with_capacity(n);
        if !unordered_segments.is_empty() {
            let mut start: DecomposedPoint = unordered_segments[0].end.into();
            while let Some(segment) = hashmap.remove(&start) {
                // TODO: A polygon's points array has ownership of start, how to move it to this new
                // array? Segment would have to take &mut Point and that would complicate things
                // sustantially
                points.push(segment.start.clone());
                start = segment.end.into();
                segments.push(segment);
            }
        }

        let bounds = Bounds::from_points(&points);

        Polygon {
            points,
            segments,
            bounds,
        }
    }

    pub fn iter_points(&self) -> Iter<'_, Point> {
        self.points.iter()
    }
}

// remember that the cost of storing a reference and 64 bits is the same
pub struct Bounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl Bounds {
    pub fn from_points(points: &[Point]) -> Bounds {
        let xs: Vec<_> = points.iter().map(|point| point.x).collect();
        let ys: Vec<_> = points.iter().map(|point| point.y).collect();
        Bounds {
            min_x: *min(&xs),
            max_x: *max(&xs),
            min_y: *min(&ys),
            max_y: *max(&ys),
        }
    }
}

fn min(values: &[f64]) -> &f64 {
    values
        .iter()
        // can't fail: points does not have NaNs
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        // would fail: points is never empty
        .unwrap()
}

fn max(values: &[f64]) -> &f64 {
    values
        .iter()
        // can't fail: points does not have NaNs
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        // would fail: points is never empty
        .unwrap()
}
