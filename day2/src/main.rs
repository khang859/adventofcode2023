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

    let mut possible_games_value = 0;

    let lines = input_string.lines();

    for line in lines {
        let game_result = Game::from_str(line);

        if let Ok(game) = game_result {
            let draws = &game.draws;

            // set to 1 so when we multiply it works.
            let mut total_greens: usize = 1;
            let mut total_blues: usize = 1;
            let mut total_reds: usize = 1;

            for draw in draws {
                total_greens = std::cmp::max(draw.green, total_greens);
                total_blues = std::cmp::max(draw.blue, total_blues);
                total_reds = std::cmp::max(draw.red, total_reds);
            }
            
            possible_games_value += total_reds * total_blues * total_greens;
        }
    }

    println!("possible game {}", possible_games_value);
}
