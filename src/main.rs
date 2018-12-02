mod day_one;
mod day_two;

fn main(){
    day_one();
    day_two();
}

fn day_one() {
    const FILE: &str = include_str!("inputs/day_one/input.txt");

    println!("I    -- I:  {}", day_one::find_final_freqeuency(FILE));
    println!("I    -- II: {}", day_one::find_first_repeated_frequency(FILE));
}


fn day_two() {
    const FILE: &str = include_str!("inputs/day_two/input.txt");

    println!("II   -- I:  {}", day_two::calculate_checksum(FILE));
    println!("II   -- II: {}", day_two::calcuate_diff(FILE));
}