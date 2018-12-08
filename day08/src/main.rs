use aoc::aoc;

fn solve(mut iter: Vec<usize>) -> (usize, Vec<usize>) {
    let mut sum = 0;

    let n_child_nodes = iter.pop().unwrap();
    let n_meta_nodes = iter.pop().unwrap();

    for _ in 0..n_child_nodes {
        let (sum2, iter2) = solve(iter);
        sum += sum2;

        iter = iter2;
    }

    for _ in 0..n_meta_nodes {
        sum += iter.pop().unwrap();
    }

    (sum, iter)
}
#[aoc(2018, 8, 1)]
fn main(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input.reverse();
    solve(input).0
}
