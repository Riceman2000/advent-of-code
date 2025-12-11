use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
// This solution is highly specialized to the actual puzzle input, it does not work for the sample
#[expected_short(None)]
#[expected_long(Some(1_498_673_376))]
pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
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
    // pretty_print(&points);
    let len = points.len();
    let mid_top = len / 2;
    let mid_bot = mid_top + 1;

    // Top half
    let mut corner = points[mid_top];

    // First point to left of corner with binary search
    let mut lo = 0;
    let mut hi = mid_top / 2;
    while lo < hi {
        let mid = usize::midpoint(lo, hi);
        if points[mid].x >= corner.x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let mut y_bound = points[lo].y;

    // Find the other corner of the rectangle
    let mut j = mid_top - 1;
    let mut max_x = 0;
    let mut max_area = 0;
    while points[j].y <= y_bound {
        // If we have a new highest x coordinate, it is possible this rectangle is the highest area, so we compute it now
        if points[j].x >= max_x {
            max_x = points[j].x;
            max_area = usize::max(
                max_area,
                (corner.x - max_x + 1) * (points[j].y - corner.y + 1),
            );
        }
        j -= 1;
    }

    // Bottom half
    // The corner of the rectangle in the top half
    corner = points[mid_bot];

    // Find the first point that is to the left of the corner with binary search
    lo = usize::midpoint(len, mid_bot);
    hi = len - 1;
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if points[mid].x >= corner.x {
            hi = mid - 1;
        } else {
            lo = mid;
        }
    }
    y_bound = points[lo].y;

    // Find the other corner of the rectangle
    j = mid_bot + 1;
    max_x = 0;
    while points[j].y >= y_bound {
        // New highest x coord means new area could be the best
        if points[j].x >= max_x {
            max_x = points[j].x;
            max_area = usize::max(
                max_area,
                (corner.x - max_x + 1) * (corner.y - points[j].y + 1),
            );
        }
        j += 1;
    }
    max_area
}

#[allow(dead_code)]
fn pretty_print(points: &[Point]) {
    let shrink_factor = 1;
    let points: Vec<_> = points
        .iter()
        .map(|p| Point {
            x: p.x / shrink_factor,
            y: p.y / shrink_factor,
        })
        .collect();
    let max_x = points.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.y).max().unwrap() + 1;

    let row = vec![b'.'; max_x];
    let mut grid = vec![row; max_y];

    for p in points {
        grid[p.y][p.x] = b'#';
    }

    let mut out = Vec::with_capacity(max_x * max_y);
    for r in grid {
        for c in r {
            out.push(c);
        }
        out.push(b'\n');
    }
    // std::fs::write("./vis.txt", &out).unwrap();
    println!("{}", String::from_utf8(out).unwrap());
}
