use aoc2025::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    // full transparency
    let part_one = input.lines().fold(0, |result, line| {
        let mut largest = 0;
        let mut largest_first: Option<u32> = None;
        let mut largest_second: Option<u32> = None;
        line.chars().for_each(|ch| {
            let joltage = ch.to_digit(10).unwrap();
            if largest_first.is_none() {
                largest_first = Some(joltage);

                return;
            }

            if largest_first.is_some_and(|digit| digit < joltage) {
                let maybe_new_largest = (largest_first.unwrap() * 10) + joltage;
                if maybe_new_largest > largest {
                    largest = maybe_new_largest
                }

                largest_first = Some(joltage);
                largest_second = None;

                return;
            }

            if largest_second.is_some_and(|digit| digit < joltage) || largest_second.is_none() {
                largest_second = Some(joltage);
                let new_largest = (largest_first.unwrap() * 10) + largest_second.unwrap();

                if new_largest > largest {
                    largest = new_largest
                }
            }
        });

        result + largest
    });

    let part_two = input.lines().fold(0, |result, line| {
        let joltages: Vec<u32> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

        let mut max_joltage_digits: Vec<u32> = vec![];

        let mut prev_index = 0;
        (0..12).rev().for_each(|remaining_digits| {
            let (max_i, max) = joltages[prev_index..(joltages.len() - remaining_digits)]
                .iter()
                .enumerate()
                .reduce(|(max_i, max), (curr_i, curr)| {
                    if curr > max {
                        (curr_i, curr)
                    } else {
                        (max_i, max)
                    }
                })
                .unwrap();

            max_joltage_digits.push(*max);
            prev_index += max_i + 1;
        });

        result
            + max_joltage_digits
                .iter()
                .enumerate()
                .fold(0, |digits, (curr_i, curr)| {
                    let base: u64 = (10_u64).pow(11 - (curr_i as u32));
                    digits + *curr as u64 * base
                })
    });

    println!("{part_one}");
    println!("{part_two}")
}
