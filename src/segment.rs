use crate::bounds::Bounds;
use crate::point::Point;
use crate::polygon::Polygon;
use std::cmp::Ordering::*;

#[derive(Clone)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
    slope: f64,
    offset: f64,
    bounds: Bounds,
    sin_alpha: f64,
    cos_alpha: f64,
}

impl Segment {
    pub fn new(start: Point, end: Point) -> Self {
        let x0 = start.x;
        let y0 = start.y;
        let x1 = end.x;
        let y1 = end.y;

        let slope = (y1 - y0) / (x1 - x0);
        let offset = y0 - slope * x0;

        let (min_x, max_x) = min_max(&x0, &x1);
        let (min_y, max_y) = min_max(&y0, &y1);
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        // TODO: confirm that this makes sense
        let alpha = dx.atan2(dy);
        let sin_alpha = alpha.sin();
        let cos_alpha = alpha.cos();

        let bounds = Bounds {
            min_x: *min_x,
            max_x: *max_x,
            min_y: *min_y,
            max_y: *max_y,
        };

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

    pub fn intersection_with(&self, other_segment: &Segment) -> Option<Point> {
        let segments_are_parallel = self.slope.partial_cmp(&other_segment.slope) == Some(Equal);

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

    fn contains(&self, point: &Point) -> bool {
        let point_on_self_basis = self.change_to_self_basis(point);
        !(point_on_self_basis.x > self.bounds.max_x || point_on_self_basis.x < self.bounds.min_x)
    }

    fn change_to_self_basis(&self, point: &Point) -> Point {
        let Point { x, y } = point;

        let Self {
            bounds,
            sin_alpha,
            cos_alpha,
            ..
        } = self;

        let Bounds { min_x, min_y, .. } = bounds;

        let xprime = (x - min_x) * sin_alpha + (y - min_y) * cos_alpha;
        let yprime = (x - min_x) * cos_alpha - (y - min_y) * sin_alpha;

        Point::new(xprime, yprime)
    }

    // TODO: this implementation is incorrect
    pub fn points_outwards_of(&self, polygon: &Polygon) -> bool {
        let y_is_increasing = self.end.y > self.start.y;
        let extended_y = if y_is_increasing {
            polygon.bounds.max_y
        } else {
            polygon.bounds.min_y
        };

        let extended_x = (extended_y - self.offset) / self.slope;
        let extended_end = Point::new(extended_x, extended_y);

        let extension_brought_end_to_start = self.start == extended_end;
        extension_brought_end_to_start
    }
}

fn min_max<'a>(a: &'a f64, b: &'a f64) -> (&'a f64, &'a f64) {
    if a.partial_cmp(b) == Some(Less) {
        (a, b)
    } else {
        (b, a)
    }
}
