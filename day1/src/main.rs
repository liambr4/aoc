fn max(amount: i32) -> Vec<i32> {
    let input = include_str!("./input.txt");
    let parsed: Vec<i32> = input
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split("\n")
                .map(|y| y.parse::<i32>().unwrap_or_default())
                .sum::<i32>()
        })
        .collect();
    let mut vec: Vec<i32> = Vec::with_capacity(amount.try_into().unwrap());

    for _ in 0..amount {
        vec.push(
            *parsed
                .iter()
                .filter(|val| !vec.contains(val))
                .max()
                .unwrap(),
        );
    }
    println!("{:?}", parsed.iter().max().unwrap());
    return vec.clone();
}
fn main() {
    let max1 = max(3);

    println!("{:?}", max1.iter().sum::<i32>());
}
