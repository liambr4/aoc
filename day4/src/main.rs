#[derive(Debug)]
struct Assignment {
    section_min : u32,
    section_max: u32,
}

fn main() {
    let input = include_str!("./input.txt");
    let mut count = 0;
    let pairs : Vec<(Assignment,Assignment) > = input.split("\n").collect::<Vec<&str>>().iter().filter(|row| row.len() > 0).map(|pair| {
        let split = pair.split(",").collect::<Vec<&str>>();
        let first: Vec<&str> = split[0].split("-").collect();
        let second: Vec<&str> = split[1].split("-").collect();
        (
            Assignment{ section_min: first[0].parse().unwrap(), section_max: first[1].parse().unwrap()},
            Assignment{ section_min: second[0].parse().unwrap(), section_max: second[1].parse().unwrap()}
        )
    }).collect();
    pairs.iter().for_each(|row|{
        if assignments_overlap(&row.0, &row.1){
           count+= 1;
        }
    });
    println!("Count: {}", count);

}
fn assignments_overlap( first: &Assignment, second: &Assignment) -> bool {
    if first.section_max <= second.section_max && first.section_min >= second.section_min{
        return true;
    }
    else if second.section_max <= first.section_max && second.section_min >= first.section_min{
        return true;
    }
    else{
        return false;
    }
}
