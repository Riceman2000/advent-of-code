#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(40))]
#[expected_long(Some(231_507_396_180_012))]
pub struct Day;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Splitter,
    Beam,
}
impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Empty,
            b'^' => Tile::Splitter,
            b'|' => Tile::Beam,
            _ => unreachable!(),
        }
    }
}
impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Splitter => '^',
            Tile::Beam => '|',
        }
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let mut input = input.split(|c| *c == b'\n');

    let start_x = input
        .next()
        .unwrap()
        .iter()
        .position(|c| *c == b'S')
        .unwrap();

    let mut grid: Vec<Vec<Tile>> = input
        .map(|l| l.iter().map(|c| (*c).into()).collect::<Vec<Tile>>())
        .filter(|l| !l.is_empty())
        .collect();
    grid[0][start_x] = Tile::Beam;

    let mut split_counts = vec![0; grid[0].len()];
    split_counts[start_x] = 1;

    for y in 1..grid.len() {
        for x in 0..grid[0].len() {
            let above = grid[y - 1][x];
            let current = grid[y][x];
            if above != Tile::Beam {
                continue;
            }
            if current == Tile::Splitter {
                split_counts[x - 1] += split_counts[x];
                split_counts[x + 1] += split_counts[x];
                split_counts[x] = 0;
                grid[y][x - 1] = Tile::Beam;
                grid[y][x + 1] = Tile::Beam;
            } else {
                grid[y][x] = Tile::Beam;
            }
        }
    }

    // pretty_print(&grid);
    split_counts.iter().sum()
}

#[allow(dead_code)]
fn pretty_print(grid: &[Vec<Tile>]) {
    for l in grid {
        for c in l {
            let c: char = (*c).into();
            print!("{c}");
        }
        println!();
    }
}
