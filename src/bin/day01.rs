use aoc2025::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let (part_one, part_two, _last_step) = input.lines().fold((0, 0, 50), |(part_one, part_two, last_step), rotation| {
        let (direction, steps_str) = rotation.split_at(1);

        let steps: i32 = steps_str.parse().unwrap();

        let current_step = match direction {
            "L" => (last_step - steps).rem_euclid(100),
            "R" => (last_step + steps).rem_euclid(100),
            _ => panic!("Invalid direction! {direction}")
        };

        // no shame, full transparency, it's 23:16
        let mut clicks = 0;
        let mut curr_steps = last_step;
        for _i in 0..steps {
            if direction == "L" { curr_steps -= 1 }
            if direction == "R" { curr_steps += 1 }

            if curr_steps.rem_euclid(100) == 0 { clicks += 1 }
        }

        (if current_step == 0 { part_one + 1 } else { part_one }, part_two + clicks, current_step)
    });

    println!("{part_one}");
    println!("{part_two}");
}
