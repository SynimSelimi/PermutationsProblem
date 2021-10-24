// Test in Rust Playground
// https://play.rust-lang.org/
pub fn permute(perms: &mut Vec<Vec<i32>>, nums: Vec<i32>, start: usize, numerator: &mut i32) {
    *numerator += 1;
    if start >= nums.len() {
        perms.push(nums);
        return
    }
    for i in start..nums.len() {
        let mut nums = nums.clone();
        nums.swap(start, i);
        permute(perms, nums, start + 1, numerator);
    }
}

fn main() {
    let numbers:Vec<i32> = vec![1,2,3];
    let mut permutations: Vec<Vec<i32>> = vec![];
    let mut numerator: i32 = 0;
    permute(&mut permutations, numbers, 0, &mut numerator);
    println!("{:?}", permutations);
    println!("Executed {:?} times", numerator);
}
