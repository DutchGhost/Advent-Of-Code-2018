const PUZZLE: &str = include_str!("input.txt");

#[derive(Default)]
struct IDMatcher<'a> {
    s1: &'a str,
    s2: &'a str,
}

impl<'a> IDMatcher<'a> {
    /// Returns `Some(Self)` if the 2 boxes are correct boxes, a.k.a, they differ only 1 character.
    pub fn find_match(s1: &'a str, s2: &'a str) -> Option<Self> {
        let mut iter = s1.chars().zip(s2.chars());
        let equal_count = iter.by_ref().take_while(|(c1, c2)| c1 == c2).count();
        let equal_count_tail = iter.take_while(|(c1, c2)| c1 == c2).count();

        if equal_count + equal_count_tail == s1.len() - 1 {
            return Some(Self {
                s1: &s1[..equal_count],
                s2: &s1[equal_count + 1..],
            });
        }

        None
    }
}

fn main() {
    let boxes = PUZZLE.lines().collect::<Vec<_>>();

    let common = boxes
        .iter()
        .enumerate()
        .filter_map(|(idx, box1)| {
            boxes[idx..]
                .iter()
                .find_map(|box2| IDMatcher::find_match(box1, box2))
        }).next()
        .expect("Failed to find it.");

    println!("{}{}", common.s1, common.s2);
}
