use crate::{Polygon, Segment};

// assumes that they overlap, could be enforced by a enum
pub fn sum<'a>(polygon_a: &'a Polygon, polygon_b: &'a Polygon) -> Polygon<'a> {
    let upper_bound = polygon_a.segments.len().max(polygon_b.segments.len());
    // each segment can cross a polygon at most twice and there are 2 polygons
    let mut segments = Vec::with_capacity(2 * 2 * upper_bound);

    clip(polygon_a, polygon_b, &mut segments);
    clip(polygon_b, polygon_a, &mut segments);

    Polygon::from_unordered_segments(segments)
}

fn clip<'a>(polygon: &'a Polygon, mould: &'a Polygon, segments: &mut Vec<&Segment>) {
    for segment in polygon.segments {
        clip_segment(mould, &segment, segments);
    }
}

fn clip_segment<'a>(mould: &'a Polygon, segment: &'a Segment, segments: &mut Vec<&Segment>) {
    for mould_segment in mould.segments {
        if let Some(intersection) = mould_segment.intersection_with(segment) {
            let sub_segment_a = Segment::new(&segment.start, &intersection);
            let kept_sub_segment = if sub_segment_a.points_outwards_of(mould) {
                &sub_segment_a
            } else {
                let sub_segment_b = Segment::new(&intersection, &segment.end);
                &sub_segment_b
            };
            segments.push(kept_sub_segment);
        } else {
            segments.push(segment);
        }
    }
}
