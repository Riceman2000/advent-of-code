use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(4_277_556))]
#[expected_long(Some(6_378_679_666_679))]
pub struct Day;

#[derive(Debug)]
enum Ops {
    Add,
    Mul,
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let input: Vec<Vec<&[u8]>> = input
        .split(|c| *c == b'\n')
        .map(|l| l.split(|c| *c == b' ').filter(|l| !l.is_empty()).collect())
        .collect();
    let split = input
        .iter()
        .position(|l| l[0][0] == b'*' || l[0][0] == b'+')
        .unwrap();
    let nums: Vec<Vec<usize>> = input[..split]
        .iter()
        .map(|l| l.iter().map(|n| atoi(n).unwrap()).collect())
        .collect();
    let ops: Vec<Ops> = input[split]
        .iter()
        .map(|o| match o[0] {
            b'+' => Ops::Add,
            b'*' => Ops::Mul,
            _ => unreachable!("Bad input"),
        })
        .collect();

    let ic: Vec<usize> = ops
        .iter()
        .map(|o| match o {
            Ops::Add => 0,
            Ops::Mul => 1,
        })
        .collect();
    nums.iter()
        .fold(ic, |mut acc, l| {
            for (x, o) in ops.iter().enumerate() {
                match o {
                    Ops::Add => acc[x] += l[x],
                    Ops::Mul => acc[x] *= l[x],
                }
            }
            acc
        })
        .iter()
        .sum()
}
