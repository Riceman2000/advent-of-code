use std::collections::HashSet;

use rstar::{PointDistance, RTree, RTreeObject, AABB};

use atoi::atoi;
use itertools::Itertools;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(40))]
// #[expected_long(Some(1717))]
pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn as_array(&self) -> [isize; 3] {
        [self.x, self.y, self.z]
    }
}

impl RTreeObject for Point {
    type Envelope = AABB<[isize; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.as_array())
    }
}

// ---- PointDistance ----

impl PointDistance for Point {
    fn distance_2(&self, point: &[isize; 3]) -> isize {
        let dx = self.x - point[0];
        let dy = self.y - point[1];
        let dz = self.z - point[2];
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    a: Point,
    b: Point,
    d2: isize,
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let points: Vec<Point> = input
        .split(|c| *c == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut nums = l.split(|c| *c == b',').map(|n| atoi(n).unwrap());
            let x = nums.next().unwrap();
            let y = nums.next().unwrap();
            let z = nums.next().unwrap();
            Point { x, y, z }
        })
        .collect();
    let tree: RTree<Point> = RTree::bulk_load(points.clone());

    let mut pairs = Vec::new();
    for point in &points {
        let nearest_neighbor = tree
            .nearest_neighbor_iter(&point.as_array())
            .skip(1)
            .next()
            .unwrap();
        let d2 = point.distance_2(&nearest_neighbor.as_array());
        let pair = Pair {
            a: *point,
            b: *nearest_neighbor,
            d2,
        };
        pairs.push(pair);
    }
    pairs.sort_unstable_by_key(|c| c.d2);
    pairs.partition_dedup_by_key(|c| c.d2);

    println!("Pairs: {pairs:#?}");

    // Support short input
    let top_n = if pairs.len() > 1000 { 1000 } else { 10 };

    let mut nets: Vec<HashSet<Point>> = points.iter().map(|p| HashSet::from([*p])).collect();
    let pairs = &pairs[..top_n];
    for pair in pairs {
        let a = pair.a;
        let b = pair.b;
        let connected_nets: Vec<_> = nets
            .iter()
            .enumerate()
            .filter(|(i, net)| net.contains(&a) || net.contains(&b))
            .map(|(i, _n)| i)
            .collect();
        match connected_nets.len() {
            0 => {
                println!("Creating new net for pair {pair:?}");
                let new_net = HashSet::from([pair.a, pair.b]);
                nets.push(new_net);
            }
            1 => {
                println!("Adding to net for pair {pair:?}");
                nets[connected_nets[0]].insert(a);
                nets[connected_nets[0]].insert(b);
            }
            2 => {
                println!("Merging nets for pair {pair:?}");
                // Iterators help with multiple mutable references
                let mut nets_iter = nets.iter_mut();
                let n0 = nets_iter.nth(connected_nets[0]).unwrap();
                let n1 = nets_iter
                    .nth(connected_nets[1] - connected_nets[0] - 1)
                    .unwrap();
                n0.extend(n1.iter());
                nets.remove(connected_nets[1]);
            }
            _ => unreachable!("Can only bridge two nets"),
        }
        // println!("{nets:#?}");
        println!("\n-\n-\n-\n");
    }
    nets.sort_unstable_by_key(|n| n.len());
    let net_lens: Vec<usize> = nets.iter().rev().map(|n| n.len()).collect();
    println!("Lens: {net_lens:?}");
    nets.iter().rev().take(3).map(|n| n.len()).product()
}
