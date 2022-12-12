use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("data.txt").unwrap();
    let lines = file_contents.lines();

    let number_subsets: i32 = lines
        .into_iter()
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();

            let mut mins_and_maxes: Vec<_> = Vec::new();

            for range in ranges {
                mins_and_maxes.push(range.split('-').collect::<Vec<&str>>());
            }

            let (min_1, max_1) = (
                mins_and_maxes[0][0].parse::<u32>().unwrap(),
                mins_and_maxes[0][1].parse::<u32>().unwrap(),
            );

            let (min_2, max_2) = (
                mins_and_maxes[1][0].parse::<u32>().unwrap(),
                mins_and_maxes[1][1].parse::<u32>().unwrap(),
            );

            if (min_1 <= min_2 && max_1 >= min_2) || (min_2 <= min_1 && max_2 >= min_1) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{}", number_subsets);
}
