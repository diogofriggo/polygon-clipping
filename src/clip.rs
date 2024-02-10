use std::sync::mpsc;
use thread_pool::ThreadPool;

use crate::{
    polygon::{polygons_from_unordered_segments, Polygon},
    segment::Segment,
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
    let (sender, _pool) = ThreadPool::fixed_size(core_count);
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

fn clip(
    mould_segments: &[Segment],
    polygon_segments: &[Segment],
    clipped_segments: &mut Vec<Segment>,
) {
    for segment in polygon_segments {
        clip_segment(mould_segments, segment, clipped_segments);
    }
}

// TODO: plenty of room for improvement: compute intersections only once
fn clip_segment(
    mould_segments: &[Segment],
    segment: &Segment,
    clipped_segments: &mut Vec<Segment>,
) {
    let clipped_segments_before = clipped_segments.len();

    for mould_segment in mould_segments {
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
