use std::sync::mpsc;
use thread_pool::ThreadPool;

use crate::{
    polygon::{polygons_from_unordered_segments, Polygon},
    segment::Segment,
    vector::Vector2d,
};

// // TODO: many polygons
// pub fn sum(polygon_a: &Polygon, polygon_b: &Polygon) -> Vec<Polygon> {
//     let upper_bound = polygon_a.segments.len().max(polygon_b.segments.len());
//     // each segment can cross a polygon at most twice and there are 2 polygons so 2 * 2
//     let mut clipped_segments = Vec::with_capacity(2 * 2 * upper_bound);
//
//     clip(
//         &polygon_a.segments,
//         &polygon_b.segments,
//         &mut clipped_segments,
//     );
//     clip(
//         &polygon_b.segments,
//         &polygon_a.segments,
//         &mut clipped_segments,
//     );
//     polygons_from_unordered_segments(clipped_segments)
// }

// IDEA: I could CoW segment and just clone points at the very end when building a polygon
// this would avoid the extra memory of Rc
// TODO: enable jmalloc (won't work with WebAssembly?)
// TODO: implement a manual ThreadPool
// TODO: consider rust async
// TODO: consider different segment types with less data for each stage of the calc
pub fn sum(mut polygons: Vec<Polygon>) -> Vec<Polygon> {
    if polygons.is_empty() || polygons.len() == 1 {
        return polygons;
    }

    let (tx, rx) = mpsc::channel();

    let n = polygons.len();
    while let Some(polygon) = polygons.pop() {
        // TODO: introduce error approach
        // TODO Segment as CoW makes this impossible?
        tx.send(polygon.segments).unwrap();
    }

    let last_task = n - 1;
    let mut task = 0;
    let core_count = num_cpus::get();
    let (sender, _pool) = ThreadPool::fixed_size(1);
    loop {
        match (rx.recv(), rx.recv()) {
            (Ok(segments_a), Ok(segments_b)) => {
                task += 1;
                let tx_for_closure = tx.clone();
                let _ = sender.send(move || {
                    let segments = clip_one_another(&segments_a, &segments_b);
                    let _ = tx_for_closure.send(segments);
                });
            }
            _ => unreachable!(),
        };

        if task == last_task {
            let unordered_segments = rx.recv().unwrap();

            return polygons_from_unordered_segments(unordered_segments);
        }
    }
}

pub fn clip_one_another(segments_a: &[Segment], segments_b: &[Segment]) -> Vec<Segment> {
    let upper_bound = segments_a.len().max(segments_b.len());
    // each segment can cross a polygon at most twice and there are 2 polygons so 2 * 2
    let mut clipped_segments = Vec::with_capacity(2 * 2 * upper_bound);

    clip(segments_b, segments_a, &mut clipped_segments);
    clip(segments_a, segments_b, &mut clipped_segments);

    clipped_segments
}

// fn is_inside_of(larger_segments: &[Segment], smaller_segments: &[Segment]) -> bool {
//     for smaller_segment in smaller_segments {
//         let is_outside = !smaller_segment.is_inside_of(larger_segments);
//         if is_outside {
//             return false;
//         }
//     }
//     true
// }

fn clip(
    mould_segments: &[Segment],
    polygon_segments: &[Segment],
    clipped_segments: &mut Vec<Segment>,
) {
    println!(
        "=== MOULD === {} POLYGON {}",
        mould_segments[0], polygon_segments[0]
    );
    for segment in polygon_segments {
        clip_segment(mould_segments, segment, polygon_segments, clipped_segments);
    }
}

// TODO: plenty of room for improvement: compute intersections only once
fn clip_segment(
    mould_segments: &[Segment],
    segment: &Segment,
    polygon_segments: &[Segment],
    clipped_segments: &mut Vec<Segment>,
) {
    // println!("EVALUATING segment {segment}");
    // println!("IS_SEGMENT_INSIDE_OF_MOULD");
    if segment.is_inside_of_or_touches(mould_segments) {
        println!("segment {segment} is inside of or touches mould, skipping it");
        return;
    }

    let clipped_segments_before = clipped_segments.len();

    for mould_segment in mould_segments {
        // println!("IS_MOULD_SEGMENT_INSIDE_OF_POLYGON");
        if mould_segment.is_inside_of_or_touches(polygon_segments) {
            let vector: Vector2d = segment.into();
            let mould_vector: Vector2d = mould_segment.into();
            let is_outside = mould_vector.dot(&vector) == -1.0;
            let process = mould_segment.is_collinear_with(segment) && is_outside;
            if !process {
                // println!("mould_segment {mould_segment} is inside of polygon but it is NOT collinear with {segment}, next mould_segment");
                continue;
            }
        }

        let intersections = mould_segment.intersections_with(segment);
        for intersection in intersections {
            let sub_segment_a = Segment::new(intersection.clone(), segment.start.clone());
            let sub_segment_b = Segment::new(intersection.clone(), segment.end.clone());

            if sub_segment_a.is_point() {
                println!("a is a point so don't push anything (the undivided segment will be pushed at loop end): {}", sub_segment_a);
                continue;
            }

            if sub_segment_b.is_point() {
                println!("b is a point so don't push anything (the undivided segment will be pushed at loop end): {}", sub_segment_b);
                continue;
            }

            // println!("a: {sub_segment_a} b: {sub_segment_b} mould: {mould_segment}");
            let kept_sub_segment = if sub_segment_a.points_inwards_of(mould_segment) {
                sub_segment_b
            } else {
                // sub_segment_a_with_correct_orientation
                Segment::new(segment.start.clone(), intersection.clone())
            };

            println!(
                "intersection between mould {} and {} pushing segment {}",
                mould_segment, segment, kept_sub_segment
            );
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

#[cfg(test)]
mod tests {
    use crate::{point::Point2d, vector::Vector2d};

    #[test]
    fn test() {
        let start_a = Point2d::new(0.0, 0.0);
        let end_a = Point2d::new(2.0, 1.0);
        let a = Vector2d::from_points(&start_a, &end_a);

        let start_b = Point2d::new(2.0, 1.0);
        let end_b = Point2d::new(1.0, 1.0);
        let b = Vector2d::from_points(&start_b, &end_b);
        assert!(a.dot(&b) < 0.0);
    }
}
