mod sort;

fn main() {
    let mut nums = sort::generate_random_array(100);
    println!("unsorted: {:?}", nums);
    sort::bubblesort(&mut nums);
    println!("sorted: {:?}", nums);
}
