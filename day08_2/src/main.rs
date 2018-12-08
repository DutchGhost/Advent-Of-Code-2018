use aoc::aoc;

fn solve(mut iter: Vec<usize>) -> (usize, Vec<usize>) {

    let mut sum = 0;

    let n_child_nodes = iter.pop().unwrap();
    let n_meta_nodes = iter.pop().unwrap();
    
    if n_child_nodes == 0 {
        for _ in 0..n_meta_nodes {
            sum += iter.pop().unwrap();
        }

        return (sum, iter);
    }
    
    
    let mut child_nodes_sum = Vec::new();
    
    for _ in 0..n_child_nodes {
        let (child_node_sum, iter2) = solve(iter);
        child_nodes_sum.push(child_node_sum);
        iter = iter2;
    }
    
    let mut metas = Vec::new();

    for _ in 0..n_meta_nodes {
        metas.push(iter.pop().unwrap());
    }

    for idx in metas {
        if let Some(elem) = child_nodes_sum.get(idx - 1) {
            sum += elem;
        }
    }

    (sum, iter)
}
#[aoc(2018, 8, 2)]
fn main(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input.reverse();
    solve(input).0
}
