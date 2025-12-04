use aoc2025::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let helpful_diagram: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    // we have no time for beauty
    let mut part_one = 0;
    helpful_diagram.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, ch)| {
            if *ch != '@' {
                return;
            }

            let mut adjacent_rolls = 0;
            if i > 0 {
                if helpful_diagram[i - 1][j] == '@' {
                    adjacent_rolls += 1
                }

                if j > 0 && helpful_diagram[i - 1][j - 1] == '@' {
                    adjacent_rolls += 1
                }

                if j < helpful_diagram[i].len() - 1 && helpful_diagram[i - 1][j + 1] == '@' {
                    adjacent_rolls += 1
                }
            }

            if i < helpful_diagram.len() - 1 {
                if helpful_diagram[i + 1][j] == '@' {
                    adjacent_rolls += 1
                }

                if j > 0 && helpful_diagram[i + 1][j - 1] == '@' {
                    adjacent_rolls += 1
                }

                if j < helpful_diagram[i].len() - 1 && helpful_diagram[i + 1][j + 1] == '@' {
                    adjacent_rolls += 1
                }
            }

            if j > 0 && helpful_diagram[i][j - 1] == '@' {
                adjacent_rolls += 1
            }
            if j < helpful_diagram[i].len() - 1 && helpful_diagram[i][j + 1] == '@' {
                adjacent_rolls += 1
            }

            if adjacent_rolls < 4 {
                part_one += 1
            }
        });
    });

    // anarchy
    let mut part_two = 0;
    let mut helpful_diagram_clone = helpful_diagram.clone();
    let mut removed = 0;
    loop {
        for i in 0..helpful_diagram_clone.len() {
            for j in 0..helpful_diagram_clone[i].len() {
                let ch = helpful_diagram_clone[i][j];

                if ch != '@' {
                    continue;
                }

                let mut adjacent_rolls = 0;
                if i > 0 {
                    if helpful_diagram_clone[i - 1][j] == '@' {
                        adjacent_rolls += 1
                    }

                    if j > 0 && helpful_diagram_clone[i - 1][j - 1] == '@' {
                        adjacent_rolls += 1
                    }

                    if j < helpful_diagram_clone[i].len() - 1
                        && helpful_diagram_clone[i - 1][j + 1] == '@'
                    {
                        adjacent_rolls += 1
                    }
                }

                if i < helpful_diagram_clone.len() - 1 {
                    if helpful_diagram_clone[i + 1][j] == '@' {
                        adjacent_rolls += 1
                    }

                    if j > 0 && helpful_diagram_clone[i + 1][j - 1] == '@' {
                        adjacent_rolls += 1
                    }

                    if j < helpful_diagram_clone[i].len() - 1
                        && helpful_diagram_clone[i + 1][j + 1] == '@'
                    {
                        adjacent_rolls += 1
                    }
                }

                if j > 0 && helpful_diagram_clone[i][j - 1] == '@' {
                    adjacent_rolls += 1
                }
                if j < helpful_diagram_clone[i].len() - 1 && helpful_diagram_clone[i][j + 1] == '@'
                {
                    adjacent_rolls += 1
                }

                if adjacent_rolls < 4 {
                    helpful_diagram_clone[i][j] = 'X';
                    removed += 1
                }
            }
        }

        part_two += removed;
        if removed == 0 {
            break;
        }
        removed = 0
    }

    println!("{part_one}");
    println!("{part_two}")
}
