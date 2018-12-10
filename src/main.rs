#![feature(vec_remove_item)]

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;

fn main(){
    day_one();
    day_two();
    day_three();
    day_four();
    day_five();
    day_six();
    day_seven();
    day_eight();
    day_nine();
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

fn day_three() {
    const FILE: &str = include_str!("inputs/day_three/input.txt");

    println!("III  -- I:  {}", day_three::find_overlap(FILE));
    println!("III  -- II: {}", day_three::find_nonoverlap(FILE));
}

fn day_four() {
    const FILE: &str = include_str!("inputs/day_four/input.txt");

    println!("IV   -- I:  {}", day_four::find_id_times_minute(FILE));
    println!("IV   -- II: {}", day_four::find_id_times_minute_most_asleep(FILE));
}

fn day_five() {
    const FILE: &str = include_str!("inputs/day_five/input.txt");

    println!("V    -- I:  {}", day_five::find_remainder(FILE));
    println!("V    -- II: {}", day_five::find_extracted_remainder(FILE));
}

fn day_six() {
    const FILE: &str = include_str!("inputs/day_six/input.txt");

    println!("VI   -- I:  {}", day_six::find_largest_area(FILE));
    println!("VI   -- II: {}", day_six::find_safest_region_area(FILE, 10000));
}

fn day_seven() {
    const FILE: &str = include_str!("inputs/day_seven/input.txt");

    println!("VII  -- I:  {}", day_seven::find_instruction_order(FILE));
    println!("VII  -- II: {}", day_seven::find_time_with_workers(FILE, 5, 60));
}

fn day_eight() {
    const FILE: &str = include_str!("inputs/day_eight/input.txt");

    println!("VIII -- I:  {}", day_eight::find_metadata_sum(FILE));
    println!("VIII -- II: {}", day_eight::find_root_node_value(FILE));
}

fn day_nine() {
    const FILE: &str = include_str!("inputs/day_nine/input.txt");

    println!("IX   -- I:  {}", day_nine::find_high_score(FILE));
    println!("VIII -- II: {}", day_nine::find_higher_score(FILE));
}