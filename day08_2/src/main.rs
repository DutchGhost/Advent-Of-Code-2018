use aoc::aoc;

fn solve(iter: &mut impl Iterator<Item = usize>) -> usize {
    match (iter.next(), iter.next()) {
        (Some(0), Some(meta_nodes)) => iter.take(meta_nodes).sum(),

        (Some(child_nodes), Some(meta_nodes)) => {
            let child_sums = (0..child_nodes).map(|_| solve(iter)).collect::<Vec<_>>();
            iter.take(meta_nodes)
                .filter_map(|idx| child_sums.get(idx - 1))
                .sum()
        }

        _ => 0,
    }
}

#[aoc(2018, 8, 2)]
fn main(input: &str) -> usize {
    let mut input = input.split_whitespace().map(|s| s.parse().unwrap());

    solve(input.by_ref())
}
