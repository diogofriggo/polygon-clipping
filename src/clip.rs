use crate::{polygon::Polygon, segment::Segment};

// TODO: handle case where polygons do not overlap
// assumes that they overlap, could be enforced by a enum
pub fn sum_pair(polygon_a: &Polygon, polygon_b: &Polygon) -> Polygon {
    let upper_bound = polygon_a.segments.len().max(polygon_b.segments.len());
    // each segment can cross a polygon at most twice and there are 2 polygons so 2 * 2
    let mut segments = Vec::with_capacity(2 * 2 * upper_bound);

    clip(polygon_a, polygon_b, &mut segments);
    clip(polygon_b, polygon_a, &mut segments);

    Polygon::from_unordered_segments(segments)
}

fn clip(polygon: &Polygon, mould: &Polygon, segments: &mut Vec<Segment>) {
    for segment in &polygon.segments {
        clip_segment(mould, segment, segments);
    }
}

// TODO: plenty of room for improvement: compute intersections only once
fn clip_segment(mould: &Polygon, segment: &Segment, segments: &mut Vec<Segment>) {
    // println!("clipping {:?} with {:?}", segment, mould);
    for mould_segment in &mould.segments {
        let intersections = mould_segment.intersections_with(segment);

        if intersections.is_empty() {
            segments.push(segment.clone());
        }

        for intersection in intersections {
            let sub_segment_a = Segment::new(segment.start.clone(), intersection.clone());
            let sub_segment_b = Segment::new(intersection.clone(), segment.end.clone());

            let kept_sub_segment = if sub_segment_a.points_inwards(mould_segment) {
                sub_segment_a
            } else {
                sub_segment_b
            };

            segments.push(kept_sub_segment);
        }
    }
    // panic!()
}
