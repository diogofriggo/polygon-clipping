use std::{cmp::Ordering, collections::HashMap};

use hash::DecomposedPoint;
use rustvision::{
    image::Image,
    rgb, save_pnm_p6,
    shapes::{Polygon as RustVisionPolygon, Shape},
    vec2,
};

mod clip;
mod hash;

fn main() {
    let a0 = Point::new(1.0, 1.0);
    let a1 = Point::new(1.0, 3.0);
    let a2 = Point::new(3.0, 3.0);
    let a3 = Point::new(3.0, 1.0);
    let points_a = vec![a0, a1, a2, a3];
    let polygon_a = Polygon::from_points(points_a);

    let b0 = Point::new(2.0, 0.0);
    let b1 = Point::new(2.0, 2.0);
    let b2 = Point::new(4.0, 2.0);
    let b3 = Point::new(4.0, 0.0);
    let points_b = vec![b0, b1, b2, b3];
    let polygon_b = Polygon::from_points(points_b);

    let polygons = vec![polygon_a, polygon_b];
    let set = PolygonSet::new(polygons);

    draw(&set);
}

// struct PolygonSet<'a> {
//     polygons: Vec<Polygon<'a>>,
// }
//
// impl PolygonSet {
//     fn new(polygons: Vec<Polygon>) -> Self {
//         Self { polygons }
//     }
//
//     // fn contains(&self, point: &Point) -> bool {
//     //     // todo: rayon (or maybe leave rayon to higher level calc?)
//     //     for polygon in &self.polygons {
//     //         if polygon.contains(point) {
//     //             return true;
//     //         }
//     //     }
//     //     false
//     // }
//
//     fn points(&self) -> Vec<&Point> {
//         self.polygons
//             .iter()
//             .flat_map(|polygon| polygon.points.iter())
//             .collect()
//     }
//
//     fn clip(&self) -> Polygon {
//         Polygon::from_points(vec![Point::new(0.0, 0.0)])
//     }
// }

fn draw(polygons: &[Polygon]) {
    let mut img = Image::new(500, 500);
    img.fill_with(&rgb!(0, 0, 0));
    for polygon in &polygons {
        let polygon: RustVisionPolygon = polygon.into();
        img.draw(&polygon);
    }
    save_pnm_p6!("clip.ppm", img);
}

struct Polygon<'a> {
    points: Vec<Point>,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    segments: Vec<Segment<'a>>,
}

impl<'a> Polygon<'a> {
    fn from_points(points: Vec<Point>) -> Self {
        let is_at_least_a_triangle = points.len() > 2;
        debug_assert!(is_at_least_a_triangle);

        let there_are_nans = points
            .iter()
            .any(|point| f64::is_nan(point.x) || f64::is_nan(point.x));
        debug_assert!(!there_are_nans);

        // polygons can be malformed (i.e. not closed, self intersecting)

        let xs: Vec<_> = points.iter().map(|point| point.x).collect();
        let ys: Vec<_> = points.iter().map(|point| point.y).collect();

        let segments = points
            .iter()
            .zip(points.iter().skip(1))
            .map(|(start, end)| Segment::new(start, end))
            .collect::<Vec<_>>();

        Self {
            points,
            min_x: *min(&xs),
            max_x: *max(&xs),
            min_y: *min(&ys),
            max_y: *max(&ys),
            segments,
        }
    }

    // fn does_not_contain(&self, point: &Point) -> bool {
    //     let Point { x, y } = point;
    //     // short-circuits as quickly as possible
    //     x < &self.min_x || x > &self.max_x || y < &self.min_y || y > &self.max_y
    // }
    //
    // fn contains(&self, point: &Point) -> bool {
    //     !self.does_not_contain(point)
    // }

    pub fn contains(&self, other_segment: &Segment) -> bool {
        false
    }

    pub fn from_unordered_segments(unordered_segments: Vec<&'a Segment<'_>>) -> Polygon<'a> {
        let n = unordered_segments.len();
        let mut hashmap: HashMap<DecomposedPoint, _> = HashMap::with_capacity(n);
        for segment in unordered_segments {
            hashmap.insert(segment.start, segment);
        }

        let mut start = unordered_segments.first().unwrap().end;
        let mut segments = Vec::with_capacity(n);
        while let Some(segment) = hashmap.remove(start) {
            start = segment.end;
            segments.push(segment);
        }

        Polygon::from_segments(segments)
    }
}

fn min(points: &[f64]) -> &f64 {
    points
        .iter()
        // can't fail: points does not have NaNs
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        // would fail: points is never empty
        .unwrap()
}

fn max(points: &[f64]) -> &f64 {
    points
        .iter()
        // can't fail: points does not have NaNs
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        // would fail: points is never empty
        .unwrap()
}

#[derive(PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl From<&Polygon> for RustVisionPolygon {
    fn from(polygon: &Polygon) -> Self {
        let scale = 100.0;
        let points = polygon
            .points
            .iter()
            .map(|point| vec2![point.x, point.y] * scale)
            .collect();
        let mut polygon = RustVisionPolygon::from_points(points);
        polygon.set_color(rgb!(255, 255, 255));
        polygon.set_filled(false);
        polygon
    }
}

struct Segment<'a> {
    start: &'a Point,
    end: &'a Point,
    slope: f64,
    offset: f64,
    min_x: &'a f64,
    max_x: &'a f64,
    min_y: &'a f64,
    max_y: &'a f64,
    cos_alpha: f64,
    sin_alpha: f64,
}

impl<'a> Segment<'a> {
    pub fn new(start: &'a Point, end: &'a Point) -> Self {
        let Point { x: x0, y: y0 } = start;
        let Point { x: x1, y: y1 } = end;
        let slope = (y1 - y0) / (x1 - x0);
        let offset = y0 - slope * x0;

        let (min_x, max_x) = min_max(x0, x1);
        let (min_y, max_y) = min_max(y0, y1);
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let alpha = dx.atan2(dy);
        let cos_alpha = alpha.cos();
        let sin_alpha = alpha.sin();

        Self {
            start,
            end,
            slope,
            offset,
            min_x,
            max_x,
            min_y,
            max_y,
            cos_alpha,
            sin_alpha,
        }
    }

    pub fn intersection_with(&self, other_segment: &Segment<'_>) -> Option<Point> {
        let segments_are_parallel =
            self.slope.partial_cmp(&other_segment.slope) == Some(Ordering::Equal);

        if segments_are_parallel {
            return None;
        }

        let x = (self.offset - other_segment.offset) / (self.slope - other_segment.slope);
        let y = self.slope * x + self.offset;
        let intersection = Point::new(x, y);

        if self.contains(&intersection) {
            Some(intersection)
        } else {
            None
        }
    }

    pub fn intersects_polygon(&self, polygon: &Polygon) -> Option<(Point, Segment)> {
        for other_segment in polygon.segments {
            if let Some(point) = self.intersection_with(&other_segment) {
                return Some((point, other_segment));
            }
        }
        None
    }

    fn contains(&self, point: &Point) -> bool {
        let point = self.transform(point);
        !(&point.x > self.max_x || &point.x < self.min_x)
    }

    fn transform(&self, point: &Point) -> Point {
        let Point { x, y } = point;

        let Self {
            min_x,
            min_y,
            sin_alpha,
            cos_alpha,
            ..
        } = self;

        let min_x = *min_x;
        let min_y = *min_y;

        let xprime = (x - min_x) * sin_alpha + (y - min_y) * cos_alpha;
        let yprime = (x - min_x) * cos_alpha - (y - min_y) * sin_alpha;

        Point::new(xprime, yprime)
    }

    fn points_outwards_of(&self, polygon: &Polygon) -> bool {
        let y_is_increasing = self.end.y > self.start.y;
        let extended_y = if y_is_increasing {
            polygon.max_y
        } else {
            polygon.min_y
        };

        let extended_x = (extended_y - self.offset) / self.slope;
        let extended_end = Point::new(extended_x, extended_y);

        let extension_brought_end_to_start = self.start == &extended_end;
        extension_brought_end_to_start
    }

    // fn crosses(&self, polygon: &Polygon) -> bool {
    //
    // }
    //
    // fn crosses_polygon(&self, polygon: &Polygon) -> bool {
    //     for segment in polygon.segments {
    //         if self.crosses(segment) {
    //             return true
    //         }
    //     }
    //     false
    // }
}

fn min_max<'a>(a: &'a f64, b: &'a f64) -> (&'a f64, &'a f64) {
    if a.partial_cmp(b) == Some(Ordering::Less) {
        (a, b)
    } else {
        (b, a)
    }
}
