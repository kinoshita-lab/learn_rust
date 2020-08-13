mod sort {
    use rand::prelude::*;
    pub fn generate_random_array(len: usize) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<u32> = (0..len as u32).collect();
        nums.shuffle(&mut rng);
        nums
    }

    pub fn bubblesort(nums: &mut Vec<u32>) {
        for i in 0..nums.len() - 1 {
            for j in 1..nums.len() - i {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1)
                }
            }
        }
    }
}

fn main() {
    let mut nums = sort::generate_random_array(100);
    println!("unsorted: {:?}", nums);
    sort::bubblesort(&mut nums);
    println!("sorted: {:?}", nums);
}
