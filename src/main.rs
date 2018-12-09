mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;

fn main(){
    day_one();
    day_two();
    day_three();
    day_four();
    day_five();
    day_six();
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