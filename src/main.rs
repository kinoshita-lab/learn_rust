use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let mut nums: Vec<i32> = (0..100).collect();
    nums.shuffle(&mut rng);
    println!("unsorted: {:?}", nums);

    for i in 0..nums.len() - 1 {
        for j in 1..nums.len() - i {
            if nums[j] < nums[j - 1] {
                nums.swap(j, j - 1)
            }
        }
    }

    println!("sorted: {:?}", nums);
}
