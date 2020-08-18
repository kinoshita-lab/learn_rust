use rand::prelude::*;
pub fn generate_random_array(len: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u32> = (0..len as u32).collect();
    nums.shuffle(&mut rng);
    nums
}
