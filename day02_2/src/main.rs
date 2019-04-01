use aoc::aoc;

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

#[aoc(2018, 2, 2)]
fn main(input: &str) -> String {
    let boxes = input.lines().collect::<Vec<_>>();

    let common = boxes
        .iter()
        .enumerate()
        .filter_map(|(idx, box1)| {
            boxes[idx..]
                .iter()
                .find_map(|box2| IDMatcher::find_match(box1, box2))
        })
        .next()
        .expect("Failed to find it.");

    format!("{}{}", common.s1, common.s2)
}
