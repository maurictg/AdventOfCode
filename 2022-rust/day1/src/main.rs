use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut elves: Vec<i32> = input.split("\n\n")
        .map(|e|e.split("\n").map(|c|c.parse::<i32>().unwrap()).sum::<i32>()).collect();

    // Part 1
    let part1 = *elves.iter().max().unwrap(); // dereference because else we get reference to int in vector

    // Part 2
    elves.sort_by(|x,y|y.cmp(x));
    
    let part2: i32 = elves.iter().take(3).sum();

    println!("part 1: {}, part 2: {}", part1, part2);
}
