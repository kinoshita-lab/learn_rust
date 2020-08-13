mod sort;

fn main() {
    let mut nums = sort::sortcore::generate_random_array(100);
    println!("unsorted: {:?}", nums);
    sort::sortcore::bubblesort(&mut nums);
    println!("sorted: {:?}", nums);
}
