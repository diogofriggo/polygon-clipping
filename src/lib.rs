pub mod bounds;
pub mod clip;
pub mod draw;
pub mod iter_from;
pub mod point;
pub mod polygon;
pub mod segment;
pub mod vector;

#[cfg(test)]
mod tests {
    use crate::{clip, point::Point2d, polygon::Polygon};

    #[test]
    fn test_two_squares_partially_overlapping() {
        let p0 = Point2d::new(1.0, 1.0);
        let p1 = Point2d::new(1.0, 3.0);
        let p2 = Point2d::new(3.0, 3.0);
        let p3 = Point2d::new(3.0, 1.0);
        let points = vec![p0, p1, p2, p3];
        let square_a = Polygon::from_points(points);

        let p0 = Point2d::new(2.0, 0.0);
        let p1 = Point2d::new(2.0, 2.0);
        let p2 = Point2d::new(4.0, 2.0);
        let p3 = Point2d::new(4.0, 0.0);
        let points = vec![p0, p1, p2, p3];
        let square_b = Polygon::from_points(points);

        let polygons = vec![square_a, square_b];
        let actual_polygons = clip::sum(polygons);

        let p0 = Point2d::new(2.0, 0.0);
        let p1 = Point2d::new(2.0, 1.0);
        let p2 = Point2d::new(1.0, 1.0);
        let p3 = Point2d::new(1.0, 3.0);
        let p4 = Point2d::new(3.0, 3.0);
        let p5 = Point2d::new(3.0, 2.0);
        let p6 = Point2d::new(4.0, 2.0);
        let p7 = Point2d::new(4.0, 0.0);
        let points = vec![p0, p1, p2, p3, p4, p5, p6, p7];
        let expected_polygon = Polygon::from_points(points);
        let expected_polygons = vec![expected_polygon];

        assert_eq!(actual_polygons, expected_polygons);
    }

    #[test]
    fn test_two_triangles_partially_overlapping() {
        let p0 = Point2d::new(1.0, 1.0);
        let p1 = Point2d::new(2.0, 3.0);
        let p2 = Point2d::new(3.0, 1.0);
        let points = vec![p0, p1, p2];
        let triangle_a = Polygon::from_points(points);

        let p0 = Point2d::new(2.0, 2.0);
        let p1 = Point2d::new(3.0, 4.0);
        let p2 = Point2d::new(4.0, 2.0);
        let points = vec![p0, p1, p2];
        let triangle_b = Polygon::from_points(points);

        let polygons = vec![triangle_a, triangle_b];
        let actual_polygons = clip::sum(polygons);

        let p0 = Point2d::new(1.0, 1.0);
        let p1 = Point2d::new(2.0, 3.0);
        let p2 = Point2d::new(2.25, 2.5);
        let p3 = Point2d::new(3.0, 4.0);
        let p4 = Point2d::new(4.0, 2.0);
        let p5 = Point2d::new(2.5, 2.0);
        let p6 = Point2d::new(3.0, 1.0);
        let points = vec![p0, p1, p2, p3, p4, p5, p6];
        let expected_polygon = Polygon::from_points(points);
        let expected_polygons = vec![expected_polygon];

        assert_eq!(actual_polygons, expected_polygons);
    }

    #[test]
    fn test_a_square_and_a_triangle_partially_overlapping() {
        let p0 = Point2d::new(1.0, 1.0);
        let p1 = Point2d::new(1.0, 3.0);
        let p2 = Point2d::new(3.0, 3.0);
        let p3 = Point2d::new(3.0, 1.0);
        let points = vec![p0, p1, p2, p3];
        let square = Polygon::from_points(points);

        let p0 = Point2d::new(0.0, 0.0);
        let p1 = Point2d::new(3.0, 6.0);
        let p2 = Point2d::new(6.0, 0.0);
        let points = vec![p0, p1, p2];
        let triangle = Polygon::from_points(points);

        let polygons = vec![square, triangle];
        let actual_polygons = clip::sum(polygons);

        let p0 = Point2d::new(0.0, 0.0);
        let p1 = Point2d::new(1.0, 2.0);
        let p2 = Point2d::new(1.0, 3.0);
        let p3 = Point2d::new(1.5, 3.0);
        let p4 = Point2d::new(3.0, 6.0);
        let p5 = Point2d::new(6.0, 0.0);
        let points = vec![p0, p1, p2, p3, p4, p5];
        let expected_polygon = Polygon::from_points(points);
        let expected_polygons = vec![expected_polygon];

        assert_eq!(actual_polygons, expected_polygons);
    }

    #[test]
    fn test_a_square_sharing_one_edge_with_a_triangle_and_inside_of_the_triangle() {
        let p0 = Point2d::new(2.0, 0.0);
        let p1 = Point2d::new(2.0, 2.0);
        let p2 = Point2d::new(4.0, 2.0);
        let p3 = Point2d::new(4.0, 0.0);
        let points = vec![p0, p1, p2, p3];
        let square = Polygon::from_points(points);

        let p0 = Point2d::new(0.0, 0.0);
        let p1 = Point2d::new(3.0, 6.0);
        let p2 = Point2d::new(6.0, 0.0);
        let points = vec![p0, p1, p2];
        let triangle = Polygon::from_points(points);
        let expected_polygons = vec![triangle.clone()];
        let polygons = vec![square, triangle];
        let actual_polygons = clip::sum(polygons);

        assert_eq!(actual_polygons, expected_polygons);
    }

    #[test]
    fn test_a_square_sharing_one_edge_with_a_triangle_but_otherwise_outside_the_triangle() {
        let p0 = Point2d::new(2.0, 0.0);
        let p1 = Point2d::new(4.0, 0.0);
        let p2 = Point2d::new(4.0, -2.0);
        let p3 = Point2d::new(2.0, -2.0);
        let points = vec![p0, p1, p2, p3];
        let square = Polygon::from_points(points);

        let p0 = Point2d::new(0.0, 0.0);
        let p1 = Point2d::new(3.0, 6.0);
        let p2 = Point2d::new(6.0, 0.0);
        let points = vec![p0, p1, p2];
        let triangle = Polygon::from_points(points);

        let polygons = vec![square, triangle];
        let actual_polygons = clip::sum(polygons);

        let p0 = Point2d::new(0.0, 0.0);
        let p1 = Point2d::new(3.0, 6.0);
        let p2 = Point2d::new(6.0, 0.0);
        let p3 = Point2d::new(4.0, 0.0);
        let p4 = Point2d::new(4.0, -2.0);
        let p5 = Point2d::new(2.0, -2.0);
        let p6 = Point2d::new(2.0, 0.0);
        let points = vec![p0, p1, p2, p3, p4, p5, p6];
        let expected_polygon = Polygon::from_points(points);
        let expected_polygons = vec![expected_polygon];

        assert_eq!(actual_polygons, expected_polygons);
    }
}
