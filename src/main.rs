use point::Point2d;
use polygon::Polygon;

mod bounds;
mod clip;
mod draw;
mod point;
mod polygon;
mod segment;
mod vector;

fn main() {
    _case1();
    // _case2();
    // _case3();
}

fn _case1() {
    let a0 = Point2d::new(1.0, 1.0);
    let a1 = Point2d::new(1.0, 3.0);
    let a2 = Point2d::new(3.0, 3.0);
    let a3 = Point2d::new(3.0, 1.0);
    let points_a = vec![a0, a1, a2, a3];
    let polygon_a = Polygon::from_points(points_a);

    let b0 = Point2d::new(2.0, 0.0);
    let b1 = Point2d::new(2.0, 2.0);
    let b2 = Point2d::new(4.0, 2.0);
    let b3 = Point2d::new(4.0, 0.0);
    let points_b = vec![b0, b1, b2, b3];
    let polygon_b = Polygon::from_points(points_b);

    let c0 = Point2d::new(4.0, 2.0);
    let c1 = Point2d::new(4.0, 4.0);
    let c2 = Point2d::new(6.0, 4.0);
    let c3 = Point2d::new(6.0, 2.0);
    let points_c = vec![c0, c1, c2, c3];
    let polygon_c = Polygon::from_points(points_c);

    let d0 = Point2d::new(0.0, 0.0);
    let d1 = Point2d::new(3.0, 6.0);
    let d2 = Point2d::new(6.0, 0.0);
    let points_d = vec![d0, d1, d2];
    let polygon_d = Polygon::from_points(points_d);

    // let polygons = vec![polygon_a, polygon_b, polygon_c, polygon_d];
    let polygons = vec![polygon_a, polygon_d];

    let polygons = clip::sum(polygons);
    draw::draw(&polygons);
}

fn _case2() {
    let a0 = Point2d::new(1.0, 1.0);
    let a1 = Point2d::new(3.0, 1.0);
    let a2 = Point2d::new(2.0, 3.0);
    let points_a = vec![a0, a1, a2];
    let polygon_a = Polygon::from_points(points_a);

    let b0 = Point2d::new(2.0, 2.0);
    let b1 = Point2d::new(4.0, 2.0);
    let b2 = Point2d::new(3.0, 4.0);
    let points_b = vec![b0, b1, b2];
    let polygon_b = Polygon::from_points(points_b);

    let polygons = vec![polygon_a, polygon_b];

    let polygons = clip::sum(polygons);
    draw::draw(&polygons);
}
