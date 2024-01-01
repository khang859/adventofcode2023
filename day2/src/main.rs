use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    green: usize,
    blue: usize,
    red: usize,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game_str = s.split(":");
        let game_id_str = game_str.next().expect("where game id str?!");
        let game_id = game_id_str
            .split(" ")
            .skip(1)
            .next()
            .expect("where game id?!");

        let game_id = game_id.parse::<usize>().expect("game id invalid format");

        let mut draws = vec![];

        game_str
            .next()
            .expect("where my draws?!")
            .split(";")
            .for_each(|draw| {
                let mut draw_struct = Draw {
                    green: 0,
                    blue: 0,
                    red: 0,
                };

                draw.split(",").for_each(|draw_item| {
                    let mut draw_item = draw_item.trim().split(" ");

                    let draw_item_num = draw_item.next().expect("Missing first part of draw item");

                    let draw_item_type =
                        draw_item.next().expect("Missing second part of draw item");

                    match draw_item_type {
                        "green" => {
                            draw_struct.green = draw_item_num
                                .parse()
                                .expect("something went wrong parsing draw item number")
                        }
                        "blue" => {
                            draw_struct.blue = draw_item_num
                                .parse()
                                .expect("something went wrong parsing draw item number")
                        }
                        "red" => {
                            draw_struct.red = draw_item_num
                                .parse()
                                .expect("something went wrong parsing draw item number")
                        }
                        _ => eprintln!("Unknown draw item type"),
                    }
                });

                draws.push(draw_struct);
            });

        return Ok(Game { id: game_id, draws });
    }
}

fn main() {
    let input_string = std::fs::read_to_string("src/input").expect("where input file?!");

    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let mut possible_games_id = 0;

    let lines = input_string.lines();

    'lines: for line in lines {
        let game_result = Game::from_str(line);

        if let Ok(game) = game_result {
            let draws = &game.draws;

            for draw in draws {
                if draw.green > green_max {
                    continue 'lines;
                }

                if draw.red > red_max {
                    continue 'lines;
                }
                if draw.blue > blue_max {
                    continue 'lines;
                }
            }
            
            possible_games_id += game.id;
        }
    }

    println!("possible game {}", possible_games_id);
}
