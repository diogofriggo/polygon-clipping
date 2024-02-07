use point::Point;
use polygon::Polygon;

mod bounds;
mod clip;
mod draw;
mod point;
mod polygon;
mod segment;

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

    let sum_polygon = clip::sum(&polygon_a, &polygon_b);
    let polygons = vec![polygon_a, polygon_b, sum_polygon];
    draw::draw(&polygons);
}
