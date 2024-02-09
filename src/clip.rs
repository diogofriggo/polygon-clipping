use crate::{polygon::Polygon, segment::Segment};

// TODO: handle case where polygons do not overlap
// assumes that they overlap, could be enforced by a enum
pub fn sum_pair(polygon_a: &Polygon, polygon_b: &Polygon) -> Polygon {
    let upper_bound = polygon_a.segments.len().max(polygon_b.segments.len());
    // each segment can cross a polygon at most twice and there are 2 polygons so 2 * 2
    let mut clipped_segments = Vec::with_capacity(2 * 2 * upper_bound);

    clip(polygon_a, polygon_b, &mut clipped_segments);
    clip(polygon_b, polygon_a, &mut clipped_segments);
    Polygon::from_unordered_segments(clipped_segments)
}

fn clip(mould: &Polygon, polygon: &Polygon, clipped_segments: &mut Vec<Segment>) {
    for segment in &polygon.segments {
        clip_segment(mould, segment, clipped_segments);
    }
}

// TODO: plenty of room for improvement: compute intersections only once
fn clip_segment(mould: &Polygon, segment: &Segment, clipped_segments: &mut Vec<Segment>) {
    let clipped_segments_before = clipped_segments.len();

    for mould_segment in &mould.segments {
        let intersections = mould_segment.intersections_with(segment);

        for intersection in intersections {
            let sub_segment_a = Segment::new(segment.start.clone(), intersection.clone());
            let sub_segment_b = Segment::new(intersection.clone(), segment.end.clone());

            let kept_sub_segment = if sub_segment_a.points_inwards(mould_segment) {
                sub_segment_a
            } else {
                sub_segment_b
            };

            println!("pushing segment {}", kept_sub_segment);
            clipped_segments.push(kept_sub_segment);
        }
    }

    let clipped_segments_after = clipped_segments.len();
    let segment_went_unclipped = clipped_segments_after == clipped_segments_before;

    if segment_went_unclipped {
        println!("there is no intersection, just pushing {}", segment);
        clipped_segments.push(segment.clone());
    }
}

// enum Recipe<'a> {
//     Add(&'a Segment),
//     Clip(&'a Segment, &'a Segment),
// }
//
// fn recipes<'a>(mould: &'a Polygon, other: &'a Polygon) -> Vec<Recipe<'a>> {
//     for segment in other.segments {
//         for mould_segment in mould.segments {
//             let intersections = segment.intersections_with(mould_segment);
//             if in
//
//             }
//         }
//     }
//
//
//     vec![]
// }
//
