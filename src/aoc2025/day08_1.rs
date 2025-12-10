use std::{collections::BinaryHeap, hash::Hash};

use rstar::{PointDistance, RTree, RTreeObject, AABB};

use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(25272))]
#[expected_long(Some(100_011_612))]
pub struct Day;

// Point type

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl nohash_hasher::IsEnabled for Point {}
impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let id = self.x ^ self.y ^ self.z;
        state.write_isize(id);
    }
}

impl Point {
    fn as_array(&self) -> [isize; 3] {
        [self.x, self.y, self.z]
    }
}

// Pair type

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    a: Point,
    b: Point,
    d2: isize,
}

impl nohash_hasher::IsEnabled for Pair {}
impl Hash for Pair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_isize(self.d2);
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.d2.cmp(&other.d2)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// RStar stuff

impl RTreeObject for Point {
    type Envelope = AABB<[isize; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.as_array())
    }
}

impl PointDistance for Point {
    fn distance_2(&self, point: &[isize; 3]) -> isize {
        let dx = self.x - point[0];
        let dy = self.y - point[1];
        let dz = self.z - point[2];
        dx * dx + dy * dy + dz * dz
    }
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

    let mut pairs = BinaryHeap::new();
    let mut seen = nohash_hasher::IntSet::default();
    let scan_range = 50;
    for (i, point) in points.iter().enumerate() {
        let nearest_neighbors = tree
            .nearest_neighbor_iter(&point.as_array())
            .skip(1) // Skip self
            .take(scan_range);

        for neigbor in nearest_neighbors {
            let j = points.iter().position(|p| p == neigbor).unwrap();
            let (a, b) = if i < j { (i, j) } else { (j, i) };
            let (a, b) = (points[a], points[b]);
            let d2 = a.distance_2(&b.as_array());
            let pair = Pair { a, b, d2 };

            if seen.insert(pair) {
                // new pair
                // let (a, b) = (points[a], points[b]);
                pairs.push(pair);
            }
        }
    }
    let pairs = pairs.into_sorted_vec();

    let mut nets: Vec<nohash_hasher::IntSet<Point>> = points
        .iter()
        .map(|p| {
            let mut s = nohash_hasher::IntSet::default();
            s.insert(*p);
            s
        })
        .collect();

    for pair in pairs {
        let a = pair.a;
        let b = pair.b;
        let connected_nets: Vec<_> = nets
            .iter()
            .enumerate()
            .filter(|(_i, net)| net.contains(&a) || net.contains(&b))
            .map(|(i, _n)| i)
            .collect();
        match connected_nets.len() {
            0 => unreachable!("All points are already their own nets"),
            1 => {
                nets[connected_nets[0]].insert(a);
                nets[connected_nets[0]].insert(b);
            }
            2 => {
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
        if nets.len() == 1 {
            return (a.x * b.x).cast_unsigned();
        }
    }
    0
}
