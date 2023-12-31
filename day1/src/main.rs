#[derive(Debug)]
struct Number<'a> {
    num: usize,
    num_char: char,
    label: &'a str,
}

#[derive(Debug)]
struct NumberPosition {
    number: usize,
    position: usize,
}

fn obtain_digit(line: &str, acceptable_values: &Vec<Number>) -> Option<String> {
    let mut min_label: Option<NumberPosition> = None;
    let mut max_label: Option<NumberPosition> = None;

    for val in acceptable_values.iter() {
        let position = line.find(val.label);

        match (&min_label, position) {
            (Some(label), Some(pos)) => {
                if label.position > pos {
                    min_label = Some(NumberPosition {
                        number: val.num,
                        position: pos,
                    });
                }
            }
            (None, Some(pos)) => {
                min_label = Some(NumberPosition {
                    number: val.num,
                    position: pos,
                });
            }
            _ => (),
        }

        let last_pos = line.to_lowercase().rfind(val.label);

        match (&max_label, last_pos) {
            (Some(max), Some(pos)) => {
                if pos > max.position {
                    max_label = Some(NumberPosition {
                        number: val.num,
                        position: pos,
                    });
                }
            }
            (None, Some(pos)) => {
                max_label = Some(NumberPosition {
                    number: val.num,
                    position: pos,
                });
            }
            _ => (),
        }
    }

    let mut min_num: Option<NumberPosition> = None;
    let mut max_num: Option<NumberPosition> = None;

    for val in acceptable_values.iter() {
        let min_position = line.chars().position(|x| x == val.num_char);

        match (&min_num, min_position) {
            (Some(min), Some(pos)) => {
                if min.position > pos {
                    min_num = Some(NumberPosition {
                        number: val.num,
                        position: pos,
                    });
                }
            }
            (None, Some(pos)) => {
                min_num = Some(NumberPosition {
                    number: val.num,
                    position: pos,
                });
            }
            _ => (),
        }

        // Since its reverse positions
        // the smaller position is actually the higher.
        let max_position = line.chars().rev().position(|x| x == val.num_char);

        match (&max_num, max_position) {
            (Some(max), Some(pos)) => {
                let original_pos = line.len() - 1 - pos;
                if original_pos > max.position {
                    max_num = Some(NumberPosition {
                        number: val.num,
                        position: original_pos,
                    });
                }
            }
            (None, Some(pos)) => {
                let original_pos = line.len() - 1 - pos;

                max_num = Some(NumberPosition {
                    number: val.num,
                    position: original_pos,
                });
            }
            _ => (),
        }
    }

    let mut digit: Option<String> = None;

    match (min_label, min_num) {
        (Some(label), Some(num)) => {
            if label.position < num.position {
                digit = Some(label.number.to_string());
            } else if label.position > num.position {
                digit = Some(num.number.to_string());
            }
        }
        (None, Some(num)) => {
            digit = Some(num.number.to_string());
        }
        (Some(label), None) => {
            digit = Some(label.number.to_string());
        }
        _ => (),
    }

    match (max_label, max_num, &digit) {
        (Some(label), Some(num), Some(item)) => {
            if label.position < num.position {
                digit = Some(format!("{}{}", &item, num.number.to_string()));
            } else if label.position > num.position {
                digit = Some(format!("{}{}", &item, label.number.to_string()));
            }
        }
        (None, Some(num), Some(item)) => {
            digit = Some(format!("{}{}", item, num.number.to_string()));
        }
        (Some(label), None, Some(item)) => {
            digit = Some(format!("{}{}", item, label.number.to_string()));
        }
        _ => (),
    }

    return digit;
}

fn main() {
    let input_file = std::fs::read_to_string("src/input").expect("unable to read file");

    let acceptable_values = vec![
        Number {
            num: 1,
            num_char: '1',
            label: "one",
        },
        Number {
            num: 2,
            num_char: '2',
            label: "two",
        },
        Number {
            num: 3,
            num_char: '3',
            label: "three",
        },
        Number {
            num: 4,
            num_char: '4',
            label: "four",
        },
        Number {
            num: 5,
            num_char: '5',
            label: "five",
        },
        Number {
            num: 6,
            num_char: '6',
            label: "six",
        },
        Number {
            num: 7,
            num_char: '7',
            label: "seven",
        },
        Number {
            num: 8,
            num_char: '8',
            label: "eight",
        },
        Number {
            num: 9,
            num_char: '9',
            label: "nine",
        },
    ];
    let mut output: usize = 0;

    input_file.lines().for_each(|line| {
        let digit = obtain_digit(line, &acceptable_values);

        if let Some(item) = digit {
            let parsed = item.parse::<usize>();

            if let Ok(parsed_item) = parsed {
                output += parsed_item;
            }
        }
    });

    println!("{}", output);
}
