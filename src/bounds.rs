use crate::point::Point2d;

// remember that the cost of storing a reference and 64 bits is the same
#[derive(Clone, Debug)]
pub struct Bounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl Bounds {
    pub fn from_points(points: &[Point2d]) -> Bounds {
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
