extern crate bubblesort;

fn main() {
    let mut nums = bubblesort::arraygenerator::generate_random_array(100);
    println!("unsorted: {:?}", nums);
    bubblesort::sort::bubblesort(&mut nums);
    println!("sorted: {:?}", nums);
}
