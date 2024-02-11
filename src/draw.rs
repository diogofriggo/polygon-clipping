use crate::polygon::Polygon;
use rustvision::{image::Image, rgb, save_pnm_p6, shapes::Polygon as RustVisionPolygon, vec2};

impl From<&Polygon> for RustVisionPolygon {
    fn from(polygon: &Polygon) -> Self {
        // TODO: drop the need for scaling
        let scale = 100.0;
        let points = polygon
            .iter_points()
            .map(|point| vec2![point.x, point.y] * scale)
            .collect();
        let mut polygon = RustVisionPolygon::from_points(points);
        polygon.set_color(rgb!(255, 255, 255));
        polygon.set_filled(false);
        polygon
    }
}

pub fn draw(polygons: &[Polygon]) {
    // let min_x = polygons
    //     .iter()
    //     .map(|polygon| polygon.bounds.min_x)
    //     .min_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();
    // let max_x = polygons
    //     .iter()
    //     .map(|polygon| polygon.bounds.max_x)
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();
    // let min_y = polygons
    //     .iter()
    //     .map(|polygon| polygon.bounds.min_y)
    //     .min_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();
    // let max_y = polygons
    //     .iter()
    //     .map(|polygon| polygon.bounds.max_y)
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let cols = (max_x - min_x) as usize;
    // let rows = (max_y - min_y) as usize;
    // let scale = 100;
    // let mut img = Image::new(scale * cols, (scale * rows) + 10);
    let mut img = Image::new(700, 700);
    img.fill_with(&rgb!(0, 0, 0));
    for polygon in polygons {
        let polygon: RustVisionPolygon = polygon.into();
        img.draw(&polygon);
    }
    save_pnm_p6!("clip.ppm", img);
}
