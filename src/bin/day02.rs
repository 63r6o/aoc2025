use aoc2025::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let (part_one, part_two) = input
        .split(',')
        .fold((0, 0), |(part_one, part_two), id_range| {
            let (start, end) = id_range
                .split_once("-")
                .map(|(start, end)| {
                    (
                        start.trim().parse::<i64>().unwrap(),
                        end.trim().parse::<i64>().unwrap(),
                    )
                })
                .unwrap();

            let part_one = part_one
                + (start..end + 1).fold(0, |sum, id| {
                    let id_str = id.to_string();
                    let (first_half, second_half) = id_str.split_at(id_str.len() / 2);

                    if first_half == second_half {
                        sum + id
                    } else {
                        sum
                    }
                });

            // horrible
            let part_two = part_two
                + (start..end + 1).fold(0, |sum, id| {
                    let id_string = id.to_string();

                    let mut steps = 1..(id_string.len());

                    let is_invalid = steps.any(|step| {
                        let first = &id_string[0..step];

                        (0..id_string.len()).step_by(step).all(|start| {
                            if start + step > id_string.len() {
                                return false;
                            }
                            let slice = &id_string[start..start + step];
                            first == slice
                        })
                    });

                    if is_invalid { sum + id } else { sum }
                });

            (part_one, part_two)
        });

    println!("{part_one}");
    println!("{part_two}")
}
