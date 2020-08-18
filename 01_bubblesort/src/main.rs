extern crate bubblesort;

use bubblesort::*;

fn main() {
    let mut nums = arraygenerator::generate_random_array(100);
    println!("unsorted: {:?}", nums);
    sort::bubblesort(&mut nums);
    println!("sorted: {:?}", nums);
}
