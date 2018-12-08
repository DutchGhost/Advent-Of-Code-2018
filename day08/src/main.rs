use aoc::aoc;

fn solve2(iter: &mut impl Iterator<Item = usize>) -> usize {
    match (iter.next(), iter.next()) {
        (Some(child_nodes), Some(meta_nodes)) => {
            (0..child_nodes).map(|_| solve2(iter)).sum::<usize>()
                + iter.take(meta_nodes).sum::<usize>()
        }

        _ => 0,
    }
}

#[aoc(2018, 8, 1)]
fn main(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    solve2(&mut input)
}
