pub fn bubblesort(nums: &mut Vec<u32>) {
    for i in 0..nums.len() - 1 {
        for j in 1..nums.len() - i {
            if nums[j] < nums[j - 1] {
                nums.swap(j, j - 1)
            }
        }
    }
}
