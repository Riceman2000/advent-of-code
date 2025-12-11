use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(50))]
#[expected_long(Some(4_765_757_080))]
pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
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
            Point { x, y }
        })
        .collect();

    let mut max = 0;
    for p1 in &points {
        for p2 in &points {
            let w = p1.x.abs_diff(p2.x) + 1;
            let h = p1.y.abs_diff(p2.y) + 1;
            let a = w * h;
            if a > max {
                max = a;
            }
        }
    }
    max
}
