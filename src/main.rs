mod day_one;

fn main(){
    day_one()
}

fn day_one() {
    const FILE: &str = include_str!("inputs/day_one/input.txt");

    println!("I    -- I:  {}", day_one::find_final_freqeuency(FILE));
    println!("I    -- II: {}", day_one::find_first_repeated_frequency(FILE));
}