#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(3_263_827))]
#[expected_long(Some(11_494_432_585_168))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let grid: Vec<&[u8]> = input
        .split(|c| *c == b'\n')
        .filter(|l| !l.is_empty())
        .collect();

    let mut total = 0;
    let mut nums = Vec::new();
    for x in (0..grid[0].len()).rev() {
        let mut num = 0;
        for r in grid.iter().as_ref() {
            let c = r[x];
            match c {
                b' ' => (),
                b'0'..=b'9' => {
                    num *= 10;
                    num += (c - b'0') as usize;
                }
                b'+' => {
                    nums.push(num);
                    num = 0;
                    total += nums.iter().sum::<usize>();
                    nums.clear();
                }
                b'*' => {
                    nums.push(num);
                    num = 0;
                    total += nums.iter().product::<usize>();
                    nums.clear();
                }
                _ => unreachable!("Bad input"),
            }
        }
        if num != 0 {
            nums.push(num);
        }
    }

    total
}
