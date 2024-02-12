use crate::bounds::Bounds;
use crate::iter_from::IteratorFrom;
use crate::point::Point2d;
use crate::segment::Segment;
use std::collections::HashMap;
use std::fmt::Display;
use std::slice::Iter;

#[derive(Debug, Clone)]
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

    pub fn iter_points(&self) -> Iter<'_, Point2d> {
        self.points.iter()
    }
}

pub fn polygons_from_unordered_segments(unordered_segments: Vec<Segment>) -> Vec<Polygon> {
    // let n = unordered_segments.len();
    // if n < 3 {
    //     panic!("cannot create polygons from just {n} segments");
    // }
    // TODO: HashableSegment, From<HashableSegment> for Segment
    let mut roadmap: HashMap<&(u64, u64), Vec<&Segment>> = HashMap::new();
    for segment in &unordered_segments {
        roadmap.entry(&segment.start.key).or_default().push(segment);
    }

    let mut polygons = vec![];

    // hashmap.values() being a vec and the outer loop are necessary because many polygons may
    // share a vertex in a way that it is ambiguous which path to follow, thus we follow all of
    // them, one at a time
    loop {
        let mut vertex = *match roadmap.keys().next() {
            None => return polygons,
            Some(key) => key,
        };
        let mut points = vec![];
        let mut segments = vec![];
        // let mut visited_vertices = HashSet::new();

        loop {
            // TODO: where you stopped there may no longer be any vertex

            let mut paths = match roadmap.remove(vertex) {
                None => break,
                // if roadmap.is_empty() {
                //     // if there are no more paths we are done
                //     return polygons;
                // } else {
                //     println!("moving on to next polygon");
                //     // move to next polygon
                //     // TODO: how can it be possible for the 'outer roadmap unwrap to fail?
                //
                //     // this branch means we have exausted all paths of a polygon
                //     // but there may be other polygons that are disjoint with the
                //     // one just built so we push these segments as a fully formed
                //     // polygon and break to check for more polygons
                //     break;
                // }
                // }
                Some(paths) => paths,
            };
            // visited_vertices.insert(vertex);

            // take any path, order is not relevant
            // the None case will never happen because only non-empty vecs
            // are ever inserted into roadmap
            if let Some(path) = paths.pop() {
                // println!("popping {path}");
                points.push(path.start.clone());
                if !paths.is_empty() {
                    // this vertex is shared by more polygons so add it back
                    // println!("adding {} back", to_point(vertex));
                    roadmap.insert(vertex, paths);
                }
                // we update the vertex pointer to now point to
                // the path whose start is this path's end
                vertex = &path.end.key;
                segments.push(path.clone());
                // None => {
                //     // remove the visited vertices so that if there are no more vertices
                //     // (i.e. from unvisited disjoint polygons) we can break out of 'outer
                //     // and return from the function
                //     // TODO: this is wrong, I cannot remove vertices just because I'm done
                //     // with one polygon, I may be deleting a shared vertex
                //     // for vertex in visited_vertices {
                //     //     roadmap.remove(vertex);
                //     // }
                //
                //     break;
                // }
                // Some(path) => path,
            };
            // println!("{} popped {path}", visited_vertices.len());
        }

        let bounds = Bounds::from_points(&points);
        let polygon = Polygon {
            points,
            segments,
            bounds,
        };
        polygons.push(polygon);
    }
}

// fn to_point(vertex: &(u64, u64)) -> Point2d {
//     let x = f64::from_bits(vertex.0);
//     let y = f64::from_bits(vertex.1);
//     Point2d::new(x, y)
// }

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        if self.points.is_empty() {
            return other.points.is_empty();
        }

        if self.points.len() != other.points.len() {
            return false;
        }

        let first_point = &self.points[0];
        self.points
            .iter()
            .zip(other.points.iter().from(first_point))
            .all(|(self_point, other_point)| self_point == other_point)
    }
}

impl Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segments = self
            .segments
            .iter()
            .map(|segment| format!("{segment}"))
            .collect::<Vec<_>>()
            .join("; ");

        write!(f, "{}", segments)
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point2d, polygon::Polygon};

    #[test]
    fn test_equality_of_two_polygons_starting_at_different_points() {
        let p0 = Point2d::new(1.0, 1.0);
        let p1 = Point2d::new(1.0, 3.0);
        let p2 = Point2d::new(3.0, 3.0);
        let p3 = Point2d::new(3.0, 1.0);
        let points = vec![p0, p1, p2, p3];
        let mut other_points = points.clone();
        let polygon_a = Polygon::from_points(points);

        for i in 1..other_points.len() {
            other_points.rotate_left(i);
            let polygon_b = Polygon::from_points(other_points.clone());
            assert_eq!(polygon_a, polygon_b);
        }

        // same as above but just making sure
        for i in 1..other_points.len() {
            other_points.rotate_right(i);
            let polygon_b = Polygon::from_points(other_points.clone());
            assert_eq!(polygon_a, polygon_b);
        }
    }
}
