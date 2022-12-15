use std::collections::HashSet;
fn main() {
    let input = include_str!("./input.txt");
    let sacks: _ = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|sack| sack.len() > 0)
        .map(|sack| {
            let first: HashSet<char> = sack[..(sack.len() / 2)].chars().collect();
            let second: HashSet<char> = sack[(sack.len() / 2)..].chars().collect();
            (first, second)
        })
        .collect::<Vec<(HashSet<char>, HashSet<char>)>>();
    let lower: Vec<char> = ('a'..='z').collect();
    let upper: Vec<char> = ('A'..='Z').collect();
    let mut prio = 0;
    sacks.iter().for_each(|sack| {
        let letter = sack
            .0
            .intersection(&sack.1)
            .collect::<Vec<&char>>()
            .first()
            .unwrap()
            .clone();
        if lower.contains(&letter) {
            prio += lower.iter().position(|i| i == letter).unwrap() + 1;
        } else {
            prio += upper.iter().position(|i| i == letter).unwrap() + 27;
        }
    });
    println!("{}", prio);
}
