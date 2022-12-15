#[derive(Debug)]
struct Pick {
    wins: String,
    point: u32,
    draws: String,
}

trait Fight {
    fn fight(&self, them: &str) -> &u32;
}
impl Fight for Pick {
    fn fight(&self, them: &str) -> &u32 {
        if &self.wins == them {
            return &self.point;
        } else {
            return &0;
        }
    }
}
trait Create {
    fn create_new(pick_type: &str) -> Self;
}
impl Create for Pick {
    fn create_new(pick_type: &str) -> Self {
        match pick_type {
            "X" => {
                return Pick {
                    point: 1,
                    wins: String::from("C"),
                    draws: String::from("A"),
                }
            }
            "Y" => {
                return Pick {
                    point: 2,
                    wins: String::from("A"),
                    draws: String::from("B"),
                }
            }
            "Z" => {
                return Pick {
                    point: 3,
                    wins: String::from("B"),
                    draws: String::from("C"),
                }
            }
            _ => {
                return Pick {
                    point: 0,
                    wins: String::from(""),
                    draws: String::from("W"),
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut points = 0;
    let _score = input.split("\n").enumerate().for_each(|(index, row)| {
        let rounds: Vec<&str> = row.split(",").collect();
        rounds
            .iter()
            .filter(|r| !r.split(" ").collect::<Vec<&str>>().contains(&""))
            .for_each(|round| {
                let picks = round.split(" ").collect::<Vec<&str>>();
                let them = picks.first().unwrap();
                let me_as_pick = Pick::create_new(picks.last().unwrap());
                println!(
                    "Round {}: Them {} | Me: {}",
                    index,
                    them,
                    picks.last().unwrap()
                );
                if me_as_pick.draws == them.to_string() {
                    points += me_as_pick.point + 3;
                } else if me_as_pick.wins == them.to_string() {
                    points += me_as_pick.point + 6;
                } else {
                    points += me_as_pick.point;
                }
                println!("{}", points)
            })
    });
}
