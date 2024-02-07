use crate::polygon::Polygon;
use rustvision::{image::Image, rgb, save_pnm_p6, shapes::Polygon as RustVisionPolygon, vec2};

impl<'a> From<&Polygon<'a>> for RustVisionPolygon {
    fn from(polygon: &Polygon) -> Self {
        // TODO: drop the need for scaling
        let scale = 100.0;
        let points = polygon
            .iter_points()
            .map(|point| vec2![point.x(), point.y()] * scale)
            .collect();
        let mut polygon = RustVisionPolygon::from_points(points);
        polygon.set_color(rgb!(255, 255, 255));
        polygon.set_filled(false);
        polygon
    }
}

pub fn draw(polygons: &[Polygon]) {
    let mut img = Image::new(500, 500);
    img.fill_with(&rgb!(0, 0, 0));
    for polygon in &polygons {
        let polygon: RustVisionPolygon = polygon.into();
        img.draw(&polygon);
    }
    save_pnm_p6!("clip.ppm", img);
}
